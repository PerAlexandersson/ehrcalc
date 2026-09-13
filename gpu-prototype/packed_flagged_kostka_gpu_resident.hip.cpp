#include <algorithm>
#include <chrono>
#include <cstdint>
#include <iomanip>
#include <iostream>
#include <limits>
#include <stdexcept>
#include <string>
#include <utility>
#include <vector>

#include <hip/hip_runtime.h>
#include <rocprim/device/device_radix_sort.hpp>
#include <rocprim/device/device_reduce_by_key.hpp>
#include <rocprim/device/device_scan.hpp>
#include <rocprim/functional.hpp>

using Key = unsigned __int128;

namespace {

constexpr std::uint32_t kDefaultModulus = 2'147'483'647U;
constexpr std::size_t kMaximumRows = 32;
constexpr std::size_t kDefaultMaximumTransitions = 150'000'000;
constexpr unsigned int kBlockSize = 256;

#define HIP_CHECK(call)                                                        \
    do {                                                                       \
        const hipError_t error = (call);                                       \
        if (error != hipSuccess) {                                             \
            throw std::runtime_error(std::string(#call) + ": " +              \
                                     hipGetErrorString(error));                \
        }                                                                      \
    } while (false)

struct ModularAdd {
    std::uint32_t modulus;

    __host__ __device__ std::uint32_t operator()(std::uint32_t left,
                                                  std::uint32_t right) const {
        const std::uint32_t sum = left + right;
        return sum >= modulus ? sum - modulus : sum;
    }
};

template <typename T>
class DeviceBuffer {
  public:
    DeviceBuffer() = default;
    explicit DeviceBuffer(std::size_t size) : size_(size) {
        if (size != 0) {
            HIP_CHECK(hipMalloc(&data_, size * sizeof(T)));
        }
    }
    DeviceBuffer(const DeviceBuffer&) = delete;
    DeviceBuffer& operator=(const DeviceBuffer&) = delete;
    DeviceBuffer(DeviceBuffer&& other) noexcept
        : data_(std::exchange(other.data_, nullptr)),
          size_(std::exchange(other.size_, 0)) {}
    DeviceBuffer& operator=(DeviceBuffer&& other) noexcept {
        if (this != &other) {
            if (data_ != nullptr) {
                static_cast<void>(hipFree(data_));
            }
            data_ = std::exchange(other.data_, nullptr);
            size_ = std::exchange(other.size_, 0);
        }
        return *this;
    }
    ~DeviceBuffer() {
        if (data_ != nullptr) {
            static_cast<void>(hipFree(data_));
        }
    }
    T* get() { return data_; }
    const T* get() const { return data_; }
    std::size_t size() const { return size_; }

  private:
    T* data_ = nullptr;
    std::size_t size_ = 0;
};

class RawDeviceBuffer {
  public:
    explicit RawDeviceBuffer(std::size_t bytes) {
        if (bytes != 0) {
            HIP_CHECK(hipMalloc(&data_, bytes));
        }
    }
    RawDeviceBuffer(const RawDeviceBuffer&) = delete;
    RawDeviceBuffer& operator=(const RawDeviceBuffer&) = delete;
    ~RawDeviceBuffer() {
        if (data_ != nullptr) {
            static_cast<void>(hipFree(data_));
        }
    }
    void* get() { return data_; }

  private:
    void* data_ = nullptr;
};

template <typename Function>
float time_gpu(Function&& function) {
    hipEvent_t start{};
    hipEvent_t stop{};
    HIP_CHECK(hipEventCreate(&start));
    HIP_CHECK(hipEventCreate(&stop));
    HIP_CHECK(hipEventRecord(start));
    function();
    HIP_CHECK(hipEventRecord(stop));
    HIP_CHECK(hipEventSynchronize(stop));
    float milliseconds = 0.0F;
    HIP_CHECK(hipEventElapsedTime(&milliseconds, start, stop));
    HIP_CHECK(hipEventDestroy(start));
    HIP_CHECK(hipEventDestroy(stop));
    return milliseconds;
}

struct DeviceShape {
    std::uint32_t rows;
    std::uint32_t bits;
    Key mask;
    std::uint32_t outer[kMaximumRows];
};

DeviceShape make_shape(const std::vector<std::uint32_t>& outer) {
    if (outer.empty() || outer.size() > kMaximumRows) {
        throw std::runtime_error("unsupported partition row count");
    }
    DeviceShape shape{};
    shape.rows = static_cast<std::uint32_t>(outer.size());
    shape.bits = 1;
    while (shape.bits < 32 &&
           (std::uint64_t{1} << shape.bits) <= outer.front()) {
        ++shape.bits;
    }
    if (shape.rows * shape.bits > 128) {
        throw std::runtime_error("partition does not fit in a 128-bit key");
    }
    shape.mask = (static_cast<Key>(1) << shape.bits) - 1;
    std::copy(outer.begin(), outer.end(), shape.outer);
    return shape;
}

__host__ __device__ Key pack_parts(const DeviceShape& shape,
                                    const std::uint32_t* parts) {
    Key key = 0;
    for (std::uint32_t row = 0; row < shape.rows; ++row) {
        key |= static_cast<Key>(parts[row]) << (shape.bits * row);
    }
    return key;
}

template <bool Emit>
__device__ std::uint64_t enumerate_extensions(
    Key source_key, std::uint32_t source_value, const DeviceShape& shape,
    std::uint32_t strip_size, std::uint32_t row_lo, std::uint32_t row_hi,
    std::uint64_t output_offset, Key* output_keys,
    std::uint32_t* output_values) {
    std::uint32_t alpha[kMaximumRows]{};
    std::uint32_t capacities[kMaximumRows]{};
    std::uint32_t choices[kMaximumRows]{};
    std::uint32_t next_choice[kMaximumRows]{};
    for (std::uint32_t row = 0; row < shape.rows; ++row) {
        alpha[row] = static_cast<std::uint32_t>(
            (source_key >> (shape.bits * row)) & shape.mask);
        if (row < row_lo || row >= row_hi) {
            capacities[row] = 0;
        } else {
            const std::uint32_t shape_capacity = shape.outer[row] - alpha[row];
            const std::uint32_t strip_capacity =
                row == 0 ? strip_size : alpha[row - 1] - alpha[row];
            capacities[row] =
                shape_capacity < strip_capacity ? shape_capacity : strip_capacity;
        }
    }

    int row = 0;
    std::uint32_t remaining = strip_size;
    std::uint64_t found = 0;
    next_choice[0] = 0;
    while (row >= 0) {
        if (row == static_cast<int>(shape.rows)) {
            if (remaining == 0) {
                if constexpr (Emit) {
                    std::uint32_t beta[kMaximumRows]{};
                    for (std::uint32_t index = 0; index < shape.rows; ++index) {
                        beta[index] = alpha[index] + choices[index];
                    }
                    output_keys[output_offset + found] = pack_parts(shape, beta);
                    output_values[output_offset + found] = source_value;
                }
                ++found;
            }
            --row;
            if (row >= 0) {
                remaining += choices[row];
                next_choice[row] = choices[row] + 1;
            }
            continue;
        }

        const std::uint32_t choice = next_choice[row];
        if (choice <= capacities[row] && choice <= remaining) {
            choices[row] = choice;
            remaining -= choice;
            ++row;
            if (row < static_cast<int>(shape.rows)) {
                next_choice[row] = 0;
            }
        } else {
            next_choice[row] = 0;
            --row;
            if (row >= 0) {
                remaining += choices[row];
                next_choice[row] = choices[row] + 1;
            }
        }
    }
    return found;
}

__global__ void count_transitions(const Key* state_keys,
                                  const std::uint32_t* state_values,
                                  std::size_t state_count, DeviceShape shape,
                                  std::uint32_t strip_size,
                                  std::uint32_t row_lo, std::uint32_t row_hi,
                                  std::uint64_t* counts) {
    const std::size_t index = blockIdx.x * blockDim.x + threadIdx.x;
    if (index < state_count) {
        counts[index] = enumerate_extensions<false>(
            state_keys[index], state_values[index], shape, strip_size, row_lo,
            row_hi, 0, nullptr, nullptr);
    }
}

__global__ void emit_transitions(const Key* state_keys,
                                 const std::uint32_t* state_values,
                                 std::size_t state_count, DeviceShape shape,
                                 std::uint32_t strip_size,
                                 std::uint32_t row_lo, std::uint32_t row_hi,
                                 const std::uint64_t* offsets,
                                 Key* transition_keys,
                                 std::uint32_t* transition_values) {
    const std::size_t index = blockIdx.x * blockDim.x + threadIdx.x;
    if (index < state_count) {
        static_cast<void>(enumerate_extensions<true>(
            state_keys[index], state_values[index], shape, strip_size, row_lo,
            row_hi, offsets[index], transition_keys, transition_values));
    }
}

struct LayerTimings {
    float count_ms = 0.0F;
    float scan_ms = 0.0F;
    float emit_ms = 0.0F;
    float sort_ms = 0.0F;
    float reduce_ms = 0.0F;
    float compact_ms = 0.0F;
};

struct StateLayer {
    DeviceBuffer<Key> keys;
    DeviceBuffer<std::uint32_t> values;
    std::size_t size = 0;
};

struct AdvanceResult {
    StateLayer states;
    std::size_t transitions = 0;
    std::size_t sort_temp_bytes = 0;
    LayerTimings timings;
};

AdvanceResult advance_layer(const StateLayer& states, const DeviceShape& shape,
                            std::uint32_t strip_size, std::uint32_t row_lo,
                            std::uint32_t row_hi,
                            std::size_t maximum_transitions,
                            std::uint32_t modulus) {
    const unsigned int blocks =
        static_cast<unsigned int>((states.size + kBlockSize - 1) / kBlockSize);
    DeviceBuffer<std::uint64_t> counts(states.size);
    DeviceBuffer<std::uint64_t> offsets(states.size);
    AdvanceResult result;
    result.timings.count_ms = time_gpu([&] {
        hipLaunchKernelGGL(count_transitions, dim3(blocks), dim3(kBlockSize), 0, 0,
                           states.keys.get(), states.values.get(), states.size,
                           shape, strip_size, row_lo, row_hi, counts.get());
        HIP_CHECK(hipGetLastError());
    });

    std::size_t scan_temp_bytes = 0;
    HIP_CHECK(rocprim::exclusive_scan(nullptr, scan_temp_bytes, counts.get(),
                                      offsets.get(), std::uint64_t{0}, states.size,
                                      rocprim::plus<std::uint64_t>{}));
    RawDeviceBuffer scan_temp(scan_temp_bytes);
    result.timings.scan_ms = time_gpu([&] {
        HIP_CHECK(rocprim::exclusive_scan(
            scan_temp.get(), scan_temp_bytes, counts.get(), offsets.get(),
            std::uint64_t{0}, states.size, rocprim::plus<std::uint64_t>{}));
    });
    std::uint64_t last_offset = 0;
    std::uint64_t last_count = 0;
    HIP_CHECK(hipMemcpy(&last_offset, offsets.get() + states.size - 1,
                        sizeof(last_offset), hipMemcpyDeviceToHost));
    HIP_CHECK(hipMemcpy(&last_count, counts.get() + states.size - 1,
                        sizeof(last_count), hipMemcpyDeviceToHost));
    const std::uint64_t transition_count_u64 = last_offset + last_count;
    if (transition_count_u64 == 0 || transition_count_u64 > maximum_transitions) {
        throw std::runtime_error("transition count is zero or exceeds configured limit");
    }
    result.transitions = static_cast<std::size_t>(transition_count_u64);

    DeviceBuffer<Key> transition_keys(result.transitions);
    DeviceBuffer<std::uint32_t> transition_values(result.transitions);
    result.timings.emit_ms = time_gpu([&] {
        hipLaunchKernelGGL(emit_transitions, dim3(blocks), dim3(kBlockSize), 0, 0,
                           states.keys.get(), states.values.get(), states.size,
                           shape, strip_size, row_lo, row_hi, offsets.get(),
                           transition_keys.get(), transition_values.get());
        HIP_CHECK(hipGetLastError());
    });

    DeviceBuffer<Key> sorted_keys(result.transitions);
    DeviceBuffer<std::uint32_t> sorted_values(result.transitions);
    DeviceBuffer<Key> unique_keys(result.transitions);
    DeviceBuffer<std::uint32_t> unique_values(result.transitions);
    DeviceBuffer<std::size_t> unique_count_device(1);
    std::size_t sort_temp_bytes = 0;
    HIP_CHECK(rocprim::radix_sort_pairs(
        nullptr, sort_temp_bytes, transition_keys.get(), sorted_keys.get(),
        transition_values.get(), sorted_values.get(), result.transitions));
    std::size_t reduce_temp_bytes = 0;
    HIP_CHECK(rocprim::reduce_by_key(
        nullptr, reduce_temp_bytes, sorted_keys.get(), sorted_values.get(),
        result.transitions, unique_keys.get(), unique_values.get(),
        unique_count_device.get(), ModularAdd{modulus}, rocprim::equal_to<Key>{}));
    RawDeviceBuffer aggregate_temp(std::max(sort_temp_bytes, reduce_temp_bytes));
    result.sort_temp_bytes = sort_temp_bytes;
    result.timings.sort_ms = time_gpu([&] {
        HIP_CHECK(rocprim::radix_sort_pairs(
            aggregate_temp.get(), sort_temp_bytes, transition_keys.get(),
            sorted_keys.get(), transition_values.get(), sorted_values.get(),
            result.transitions));
    });
    result.timings.reduce_ms = time_gpu([&] {
        HIP_CHECK(rocprim::reduce_by_key(
            aggregate_temp.get(), reduce_temp_bytes, sorted_keys.get(),
            sorted_values.get(), result.transitions, unique_keys.get(),
            unique_values.get(), unique_count_device.get(), ModularAdd{modulus},
            rocprim::equal_to<Key>{}));
    });

    std::size_t unique_count = 0;
    HIP_CHECK(hipMemcpy(&unique_count, unique_count_device.get(),
                        sizeof(unique_count), hipMemcpyDeviceToHost));
    StateLayer compact{DeviceBuffer<Key>(unique_count),
                       DeviceBuffer<std::uint32_t>(unique_count), unique_count};
    result.timings.compact_ms = time_gpu([&] {
        HIP_CHECK(hipMemcpy(compact.keys.get(), unique_keys.get(),
                            unique_count * sizeof(Key), hipMemcpyDeviceToDevice));
        HIP_CHECK(hipMemcpy(compact.values.get(), unique_values.get(),
                            unique_count * sizeof(std::uint32_t),
                            hipMemcpyDeviceToDevice));
    });
    result.states = std::move(compact);
    return result;
}

std::vector<std::uint32_t> parse_list(const std::string& raw) {
    if (raw.empty() || raw == "-") {
        return {};
    }
    std::vector<std::uint32_t> values;
    std::size_t begin = 0;
    while (begin <= raw.size()) {
        const std::size_t end = raw.find(',', begin);
        values.push_back(static_cast<std::uint32_t>(
            std::stoul(raw.substr(begin, end - begin))));
        if (end == std::string::npos) {
            break;
        }
        begin = end + 1;
    }
    return values;
}

void scale(std::vector<std::uint32_t>& values, std::uint32_t dilation) {
    for (std::uint32_t& value : values) {
        if (value > std::numeric_limits<std::uint32_t>::max() / dilation) {
            throw std::runtime_error("dilation overflow");
        }
        value *= dilation;
    }
}

std::uint64_t sum(const std::vector<std::uint32_t>& values) {
    std::uint64_t total = 0;
    for (const std::uint32_t value : values) {
        total += value;
    }
    return total;
}

}  // namespace

int main(int argc, char** argv) try {
    if (argc < 7 || argc > 9) {
        std::cerr << "usage: packed_flagged_kostka_gpu_resident DILATION OUTER "
                     "INNER WEIGHT UPPER_FLAGS LOWER_FLAGS [MAX_TRANSITIONS] "
                     "[MODULUS]\n";
        return 2;
    }
    const std::uint32_t dilation = static_cast<std::uint32_t>(std::stoul(argv[1]));
    if (dilation == 0) {
        throw std::runtime_error("dilation must be positive");
    }
    std::vector<std::uint32_t> outer = parse_list(argv[2]);
    std::vector<std::uint32_t> inner = parse_list(argv[3]);
    std::vector<std::uint32_t> weight = parse_list(argv[4]);
    const std::vector<std::uint32_t> upper = parse_list(argv[5]);
    const std::vector<std::uint32_t> lower = parse_list(argv[6]);
    const std::size_t maximum_transitions =
        argc >= 8 ? std::stoull(argv[7]) : kDefaultMaximumTransitions;
    const std::uint32_t modulus =
        argc == 9 ? static_cast<std::uint32_t>(std::stoul(argv[8]))
                  : kDefaultModulus;
    if (modulus < 2 || modulus >= (std::uint32_t{1} << 31)) {
        throw std::runtime_error("modulus must lie in 2..2^31");
    }
    scale(outer, dilation);
    scale(inner, dilation);
    scale(weight, dilation);
    if (outer.empty() || inner.size() > outer.size() || sum(outer) < sum(inner) ||
        sum(outer) - sum(inner) != sum(weight)) {
        throw std::runtime_error("incompatible shape and weight");
    }
    inner.resize(outer.size(), 0);
    for (std::size_t row = 0; row < outer.size(); ++row) {
        if ((row > 0 && outer[row - 1] < outer[row]) ||
            (row > 0 && inner[row - 1] < inner[row]) || inner[row] > outer[row]) {
            throw std::runtime_error("invalid partition containment");
        }
    }

    int device_count = 0;
    HIP_CHECK(hipGetDeviceCount(&device_count));
    if (device_count == 0) {
        throw std::runtime_error("no HIP device found");
    }
    HIP_CHECK(hipSetDevice(0));
    hipDeviceProp_t properties{};
    HIP_CHECK(hipGetDeviceProperties(&properties, 0));
    const DeviceShape shape = make_shape(outer);
    const Key initial_key = pack_parts(shape, inner.data());
    const std::uint32_t initial_value = 1;
    StateLayer states{DeviceBuffer<Key>(1), DeviceBuffer<std::uint32_t>(1), 1};
    HIP_CHECK(hipMemcpy(states.keys.get(), &initial_key, sizeof(initial_key),
                        hipMemcpyHostToDevice));
    HIP_CHECK(hipMemcpy(states.values.get(), &initial_value, sizeof(initial_value),
                        hipMemcpyHostToDevice));

    std::size_t peak_states = 1;
    std::size_t peak_transitions = 1;
    double device_work_ms = 0.0;
    const auto total_start = std::chrono::steady_clock::now();
    for (std::size_t label = 0; label < weight.size(); ++label) {
        if (weight[label] == 0) {
            std::cerr << "{\"label\":" << label + 1
                      << ",\"source_states\":" << states.size
                      << ",\"transitions\":" << states.size
                      << ",\"states\":" << states.size
                      << ",\"zero_strip\":true}\n";
            continue;
        }
        const std::uint32_t row_lo =
            label < lower.size()
                ? static_cast<std::uint32_t>(std::min<std::size_t>(
                      lower[label] == 0 ? 0 : lower[label] - 1, outer.size()))
                : 0;
        const std::uint32_t row_hi =
            label < upper.size()
                ? static_cast<std::uint32_t>(
                      std::min<std::size_t>(upper[label], outer.size()))
                : static_cast<std::uint32_t>(outer.size());
        const std::size_t source_states = states.size;
        AdvanceResult next = advance_layer(states, shape, weight[label], row_lo,
                                           row_hi, maximum_transitions, modulus);
        states = std::move(next.states);
        peak_states = std::max(peak_states, states.size);
        peak_transitions = std::max(peak_transitions, next.transitions);
        const LayerTimings& timing = next.timings;
        const double layer_work_ms = timing.count_ms + timing.scan_ms + timing.emit_ms +
                                     timing.sort_ms + timing.reduce_ms + timing.compact_ms;
        device_work_ms += layer_work_ms;
        std::cerr << std::fixed << std::setprecision(3)
                  << "{\"label\":" << label + 1
                  << ",\"source_states\":" << source_states
                  << ",\"transitions\":" << next.transitions
                  << ",\"states\":" << states.size
                  << ",\"count_ms\":" << timing.count_ms
                  << ",\"scan_ms\":" << timing.scan_ms
                  << ",\"emit_ms\":" << timing.emit_ms
                  << ",\"sort_ms\":" << timing.sort_ms
                  << ",\"reduce_ms\":" << timing.reduce_ms
                  << ",\"compact_ms\":" << timing.compact_ms
                  << ",\"sort_temp_bytes\":" << next.sort_temp_bytes << "}\n";
    }

    std::vector<Key> final_keys(states.size);
    std::vector<std::uint32_t> final_values(states.size);
    HIP_CHECK(hipMemcpy(final_keys.data(), states.keys.get(), states.size * sizeof(Key),
                        hipMemcpyDeviceToHost));
    HIP_CHECK(hipMemcpy(final_values.data(), states.values.get(),
                        states.size * sizeof(std::uint32_t),
                        hipMemcpyDeviceToHost));
    const Key target = pack_parts(shape, outer.data());
    const auto found = std::lower_bound(final_keys.begin(), final_keys.end(), target);
    const std::uint32_t answer =
        found == final_keys.end() || *found != target
            ? 0
            : final_values[static_cast<std::size_t>(found - final_keys.begin())];
    const double total_ms = std::chrono::duration<double, std::milli>(
                                std::chrono::steady_clock::now() - total_start)
                                .count();
    std::cout << std::fixed << std::setprecision(3)
              << "{\"device\":\"" << properties.name << "\","
              << "\"dilation\":" << dilation << ','
              << "\"rows\":" << outer.size() << ','
              << "\"bits_per_row\":" << shape.bits << ','
              << "\"modulus\":" << modulus << ','
              << "\"residue\":" << answer << ','
              << "\"peak_states\":" << peak_states << ','
              << "\"peak_transitions\":" << peak_transitions << ','
              << "\"device_work_ms\":" << device_work_ms << ','
              << "\"total_ms\":" << total_ms << "}\n";
    return 0;
} catch (const std::exception& error) {
    std::cerr << "error: " << error.what() << '\n';
    return 1;
}
