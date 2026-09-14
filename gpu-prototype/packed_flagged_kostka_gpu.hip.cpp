#include <algorithm>
#include <chrono>
#include <cstring>
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
#include <rocprim/functional.hpp>

using Key = unsigned __int128;

namespace {

constexpr std::uint32_t kModulus = 2'147'483'647U;
constexpr std::size_t kMaximumRows = 32;
constexpr std::size_t kDefaultMaximumTransitions = 150'000'000;

#define HIP_CHECK(call)                                                        \
    do {                                                                       \
        const hipError_t error = (call);                                       \
        if (error != hipSuccess) {                                             \
            throw std::runtime_error(std::string(#call) + ": " +              \
                                     hipGetErrorString(error));                \
        }                                                                      \
    } while (false)

struct ModularAdd {
    __host__ __device__ std::uint32_t operator()(std::uint32_t left,
                                                  std::uint32_t right) const {
        const std::uint32_t sum = left + right;
        return sum >= kModulus ? sum - kModulus : sum;
    }
};

template <typename T>
class DeviceBuffer {
  public:
    explicit DeviceBuffer(std::size_t size) {
        HIP_CHECK(hipMalloc(&data_, size * sizeof(T)));
    }
    DeviceBuffer(const DeviceBuffer&) = delete;
    DeviceBuffer& operator=(const DeviceBuffer&) = delete;
    ~DeviceBuffer() {
        if (data_ != nullptr) {
            static_cast<void>(hipFree(data_));
        }
    }
    T* get() { return data_; }

  private:
    T* data_ = nullptr;
};

class RawDeviceBuffer {
  public:
    explicit RawDeviceBuffer(std::size_t bytes) {
        HIP_CHECK(hipMalloc(&data_, bytes));
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

struct PackedPartitions {
    std::size_t rows;
    unsigned int bits;
    Key mask;

    explicit PackedPartitions(const std::vector<std::uint32_t>& outer)
        : rows(outer.size()), bits(1), mask(1) {
        if (rows > kMaximumRows) {
            throw std::runtime_error("too many partition rows");
        }
        const std::uint32_t largest = outer.empty() ? 0 : outer.front();
        while (bits < 32 && (std::uint64_t{1} << bits) <= largest) {
            ++bits;
        }
        if (rows * bits > 128) {
            throw std::runtime_error("partition does not fit in a 128-bit key");
        }
        mask = (static_cast<Key>(1) << bits) - 1;
    }

    Key pack(const std::vector<std::uint32_t>& parts) const {
        Key key = 0;
        for (std::size_t row = 0; row < rows; ++row) {
            const std::uint32_t part = row < parts.size() ? parts[row] : 0;
            if (static_cast<Key>(part) > mask) {
                throw std::runtime_error("partition part exceeds packed field");
            }
            key |= static_cast<Key>(part) << (bits * row);
        }
        return key;
    }

    std::vector<std::uint32_t> unpack(Key key) const {
        std::vector<std::uint32_t> parts(rows);
        for (std::size_t row = 0; row < rows; ++row) {
            parts[row] = static_cast<std::uint32_t>((key >> (bits * row)) & mask);
        }
        return parts;
    }
};

struct LayerTimings {
    double enumerate_ms = 0.0;
    double allocate_ms = 0.0;
    float upload_ms = 0.0F;
    float sort_ms = 0.0F;
    float reduce_ms = 0.0F;
    float download_ms = 0.0F;
};

struct ReducedLayer {
    std::vector<Key> keys;
    std::vector<std::uint32_t> values;
    LayerTimings timings;
    std::size_t sort_temp_bytes = 0;
};

ReducedLayer gpu_reduce(const std::vector<Key>& keys,
                        const std::vector<std::uint32_t>& values,
                        double enumerate_ms) {
    if (keys.empty() || keys.size() != values.size()) {
        throw std::runtime_error("invalid transition buffers");
    }
    const std::size_t size = keys.size();
    const auto allocation_start = std::chrono::steady_clock::now();
    DeviceBuffer<Key> keys_input(size);
    DeviceBuffer<Key> keys_sorted(size);
    DeviceBuffer<Key> keys_unique(size);
    DeviceBuffer<std::uint32_t> values_input(size);
    DeviceBuffer<std::uint32_t> values_sorted(size);
    DeviceBuffer<std::uint32_t> values_reduced(size);
    DeviceBuffer<std::size_t> unique_count_device(1);
    const auto allocation_stop = std::chrono::steady_clock::now();

    ReducedLayer result;
    result.timings.enumerate_ms = enumerate_ms;
    result.timings.allocate_ms =
        std::chrono::duration<double, std::milli>(allocation_stop - allocation_start).count();
    result.timings.upload_ms = time_gpu([&] {
        HIP_CHECK(hipMemcpy(keys_input.get(), keys.data(), size * sizeof(Key),
                            hipMemcpyHostToDevice));
        HIP_CHECK(hipMemcpy(values_input.get(), values.data(),
                            size * sizeof(std::uint32_t), hipMemcpyHostToDevice));
    });

    std::size_t sort_temp_bytes = 0;
    HIP_CHECK(rocprim::radix_sort_pairs(
        nullptr, sort_temp_bytes, keys_input.get(), keys_sorted.get(),
        values_input.get(), values_sorted.get(), size));
    std::size_t reduce_temp_bytes = 0;
    HIP_CHECK(rocprim::reduce_by_key(
        nullptr, reduce_temp_bytes, keys_sorted.get(), values_sorted.get(), size,
        keys_unique.get(), values_reduced.get(), unique_count_device.get(),
        ModularAdd{}, rocprim::equal_to<Key>{}));
    RawDeviceBuffer temporary(std::max(sort_temp_bytes, reduce_temp_bytes));
    result.sort_temp_bytes = sort_temp_bytes;

    result.timings.sort_ms = time_gpu([&] {
        HIP_CHECK(rocprim::radix_sort_pairs(
            temporary.get(), sort_temp_bytes, keys_input.get(), keys_sorted.get(),
            values_input.get(), values_sorted.get(), size));
    });
    result.timings.reduce_ms = time_gpu([&] {
        HIP_CHECK(rocprim::reduce_by_key(
            temporary.get(), reduce_temp_bytes, keys_sorted.get(), values_sorted.get(),
            size, keys_unique.get(), values_reduced.get(), unique_count_device.get(),
            ModularAdd{}, rocprim::equal_to<Key>{}));
    });

    std::size_t unique_count = 0;
    HIP_CHECK(hipMemcpy(&unique_count, unique_count_device.get(), sizeof(unique_count),
                        hipMemcpyDeviceToHost));
    result.keys.resize(unique_count);
    result.values.resize(unique_count);
    result.timings.download_ms = time_gpu([&] {
        HIP_CHECK(hipMemcpy(result.keys.data(), keys_unique.get(),
                            unique_count * sizeof(Key), hipMemcpyDeviceToHost));
        HIP_CHECK(hipMemcpy(result.values.data(), values_reduced.get(),
                            unique_count * sizeof(std::uint32_t),
                            hipMemcpyDeviceToHost));
    });
    return result;
}

void emit_extensions(const PackedPartitions& packer,
                     const std::vector<std::uint32_t>& alpha,
                     const std::vector<std::uint32_t>& outer,
                     std::uint32_t remaining, std::size_t row,
                     std::size_t row_lo, std::size_t row_hi,
                     std::vector<std::uint32_t>& beta, std::uint32_t value,
                     std::vector<Key>& keys, std::vector<std::uint32_t>& values,
                     std::size_t maximum_transitions) {
    if (row == packer.rows) {
        if (remaining == 0) {
            if (keys.size() == maximum_transitions) {
                throw std::runtime_error("transition limit exceeded");
            }
            keys.push_back(packer.pack(beta));
            values.push_back(value);
        }
        return;
    }

    const std::uint32_t base = alpha[row];
    if (row < row_lo || row >= row_hi) {
        beta[row] = base;
        emit_extensions(packer, alpha, outer, remaining, row + 1, row_lo, row_hi,
                        beta, value, keys, values, maximum_transitions);
        return;
    }
    const std::uint32_t shape_capacity = outer[row] - base;
    const std::uint32_t strip_capacity =
        row == 0 ? remaining : alpha[row - 1] - base;
    const std::uint32_t maximum =
        std::min(remaining, std::min(shape_capacity, strip_capacity));
    for (std::uint32_t increment = 0; increment <= maximum; ++increment) {
        beta[row] = base + increment;
        emit_extensions(packer, alpha, outer, remaining - increment, row + 1,
                        row_lo, row_hi, beta, value, keys, values,
                        maximum_transitions);
    }
    beta[row] = base;
}

std::vector<std::uint32_t> parse_list(const std::string& raw) {
    if (raw.empty() || raw == "-") {
        return {};
    }
    std::vector<std::uint32_t> values;
    std::size_t begin = 0;
    while (begin <= raw.size()) {
        const std::size_t end = raw.find(',', begin);
        const std::string part = raw.substr(begin, end - begin);
        values.push_back(static_cast<std::uint32_t>(std::stoul(part)));
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
    if (argc != 7 && argc != 8) {
        std::cerr << "usage: packed_flagged_kostka_gpu DILATION OUTER INNER WEIGHT "
                     "UPPER_FLAGS LOWER_FLAGS [MAX_TRANSITIONS]\n";
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
        argc == 8 ? std::stoull(argv[7]) : kDefaultMaximumTransitions;
    scale(outer, dilation);
    scale(inner, dilation);
    scale(weight, dilation);
    if (outer.empty() || sum(outer) < sum(inner) ||
        sum(outer) - sum(inner) != sum(weight)) {
        throw std::runtime_error("incompatible shape and weight sizes");
    }
    if (inner.size() > outer.size()) {
        throw std::runtime_error("inner shape has too many rows");
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

    const PackedPartitions packer(outer);
    std::vector<Key> state_keys{packer.pack(inner)};
    std::vector<std::uint32_t> state_values{1};
    std::size_t peak_states = 1;
    std::size_t peak_transitions = 1;
    double total_enumerate_ms = 0.0;
    double total_gpu_ms = 0.0;
    const auto total_start = std::chrono::steady_clock::now();

    for (std::size_t label = 0; label < weight.size(); ++label) {
        if (weight[label] == 0) {
            std::cerr << "{\"label\":" << label + 1
                      << ",\"source_states\":" << state_keys.size()
                      << ",\"transitions\":" << state_keys.size()
                      << ",\"states\":" << state_keys.size()
                      << ",\"zero_strip\":true}\n";
            continue;
        }
        const std::size_t row_lo = label < lower.size()
                                       ? std::min<std::size_t>(
                                             lower[label] == 0 ? 0 : lower[label] - 1,
                                             outer.size())
                                       : 0;
        const std::size_t row_hi = label < upper.size()
                                       ? std::min<std::size_t>(upper[label], outer.size())
                                       : outer.size();
        std::vector<Key> transition_keys;
        std::vector<std::uint32_t> transition_values;
        transition_keys.reserve(state_keys.size() * 2);
        transition_values.reserve(state_keys.size() * 2);
        const auto enumerate_start = std::chrono::steady_clock::now();
        for (std::size_t index = 0; index < state_keys.size(); ++index) {
            const std::vector<std::uint32_t> alpha = packer.unpack(state_keys[index]);
            std::vector<std::uint32_t> beta = alpha;
            emit_extensions(packer, alpha, outer, weight[label], 0, row_lo, row_hi,
                            beta, state_values[index], transition_keys,
                            transition_values, maximum_transitions);
        }
        const auto enumerate_stop = std::chrono::steady_clock::now();
        const double enumerate_ms =
            std::chrono::duration<double, std::milli>(enumerate_stop - enumerate_start)
                .count();
        const std::size_t source_states = state_keys.size();
        const std::size_t transition_count = transition_keys.size();
        if (transition_count == 0) {
            state_keys.clear();
            state_values.clear();
            break;
        }
        ReducedLayer reduced =
            gpu_reduce(transition_keys, transition_values, enumerate_ms);
        state_keys = std::move(reduced.keys);
        state_values = std::move(reduced.values);
        peak_states = std::max(peak_states, state_keys.size());
        peak_transitions = std::max(peak_transitions, transition_count);
        total_enumerate_ms += enumerate_ms;
        const double layer_gpu_ms = reduced.timings.allocate_ms +
                                    reduced.timings.upload_ms + reduced.timings.sort_ms +
                                    reduced.timings.reduce_ms + reduced.timings.download_ms;
        total_gpu_ms += layer_gpu_ms;
        std::cerr << std::fixed << std::setprecision(3)
                  << "{\"label\":" << label + 1
                  << ",\"source_states\":" << source_states
                  << ",\"transitions\":" << transition_count
                  << ",\"states\":" << state_keys.size()
                  << ",\"enumerate_ms\":" << enumerate_ms
                  << ",\"allocate_ms\":" << reduced.timings.allocate_ms
                  << ",\"upload_ms\":" << reduced.timings.upload_ms
                  << ",\"sort_ms\":" << reduced.timings.sort_ms
                  << ",\"reduce_ms\":" << reduced.timings.reduce_ms
                  << ",\"download_ms\":" << reduced.timings.download_ms
                  << ",\"sort_temp_bytes\":" << reduced.sort_temp_bytes << "}\n";
    }

    const Key target = packer.pack(outer);
    const auto found = std::lower_bound(state_keys.begin(), state_keys.end(), target);
    const std::uint32_t answer =
        found == state_keys.end() || *found != target
            ? 0
            : state_values[static_cast<std::size_t>(found - state_keys.begin())];
    const double total_ms = std::chrono::duration<double, std::milli>(
                                std::chrono::steady_clock::now() - total_start)
                                .count();
    std::cout << std::fixed << std::setprecision(3)
              << "{\"device\":\"" << properties.name << "\","
              << "\"dilation\":" << dilation << ','
              << "\"rows\":" << outer.size() << ','
              << "\"bits_per_row\":" << packer.bits << ','
              << "\"modulus\":" << kModulus << ','
              << "\"residue\":" << answer << ','
              << "\"peak_states\":" << peak_states << ','
              << "\"peak_transitions\":" << peak_transitions << ','
              << "\"enumerate_ms\":" << total_enumerate_ms << ','
              << "\"gpu_ms\":" << total_gpu_ms << ','
              << "\"total_ms\":" << total_ms << "}\n";
    return 0;
} catch (const std::exception& error) {
    std::cerr << "error: " << error.what() << '\n';
    return 1;
}
