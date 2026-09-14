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
using TransitionOffset = std::uint64_t;

namespace {

#ifndef EHRGPU_RESIDUE_LANES
#define EHRGPU_RESIDUE_LANES 1
#endif

#ifndef EHRGPU_BLOCK_SIZE
#define EHRGPU_BLOCK_SIZE 128
#endif

constexpr std::size_t kResidueLanes = EHRGPU_RESIDUE_LANES;
constexpr unsigned int kBlockSize = EHRGPU_BLOCK_SIZE;
constexpr std::size_t kMaximumVertices = 128;
constexpr std::size_t kDefaultMaximumTransitions = 150'000'000;
constexpr std::uint32_t kDefaultModuli[] = {
    2'147'483'647U, 2'147'483'629U, 2'147'483'587U, 2'147'483'579U,
    2'147'483'563U, 2'147'483'549U, 2'147'483'543U, 2'147'483'497U,
};
static_assert(kResidueLanes >= 1 && kResidueLanes <= 8);
static_assert(kBlockSize >= 64 && kBlockSize <= 1024);

#define HIP_CHECK(call)                                                        \
    do {                                                                       \
        const hipError_t error = (call);                                       \
        if (error != hipSuccess) {                                             \
            throw std::runtime_error(std::string(#call) + ": " +              \
                                     hipGetErrorString(error));                \
        }                                                                      \
    } while (false)

struct Residues {
    std::uint32_t values[kResidueLanes];
};

struct ModularAdd {
    Residues moduli;

    __host__ __device__ Residues operator()(Residues left,
                                            Residues right) const {
#pragma unroll
        for (std::size_t lane = 0; lane < kResidueLanes; ++lane) {
            const std::uint32_t sum = left.values[lane] + right.values[lane];
            left.values[lane] = sum >= moduli.values[lane]
                                    ? sum - moduli.values[lane]
                                    : sum;
        }
        return left;
    }
};

__host__ __device__ Residues multiply(Residues value, std::uint32_t factor,
                                      Residues moduli) {
#pragma unroll
    for (std::size_t lane = 0; lane < kResidueLanes; ++lane) {
        value.values[lane] = static_cast<std::uint32_t>(
            (static_cast<std::uint64_t>(value.values[lane]) * factor) %
            moduli.values[lane]);
    }
    return value;
}

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

struct StepPlan {
    std::uint32_t parent_count = 0;
    std::uint32_t keep_count = 0;
    std::uint32_t current_fields = 0;
    std::uint32_t next_fields = 0;
    std::uint32_t parent_positions[kMaximumVertices]{};
    std::uint32_t keep_positions[kMaximumVertices]{};
    bool enters_live = false;
};

struct PlannedPoset {
    std::vector<StepPlan> steps;
    std::size_t maximum_frontier = 0;
};

__host__ __device__ std::uint32_t field(Key key, std::uint32_t position,
                                        std::uint32_t bits, Key mask) {
    return static_cast<std::uint32_t>((key >> (position * bits)) & mask);
}

__host__ __device__ Key compact_key(Key source, const StepPlan& plan,
                                    std::uint32_t bits, Key mask) {
    Key target = 0;
    for (std::uint32_t output = 0; output < plan.keep_count; ++output) {
        target |= static_cast<Key>(
                      field(source, plan.keep_positions[output], bits, mask))
                  << (output * bits);
    }
    return target;
}

__host__ __device__ std::uint32_t lower_bound(Key source,
                                              const StepPlan& plan,
                                              std::uint32_t bits, Key mask,
                                              bool strict) {
    std::uint32_t lower = 1;
    for (std::uint32_t index = 0; index < plan.parent_count; ++index) {
        const std::uint32_t parent =
            field(source, plan.parent_positions[index], bits, mask);
        const std::uint32_t candidate = parent + (strict ? 1U : 0U);
        lower = candidate > lower ? candidate : lower;
    }
    return lower;
}

__host__ __device__ TransitionOffset transition_count(
    Key source, const StepPlan& plan, std::uint32_t colors,
    std::uint32_t bits, Key mask, bool strict) {
    const std::uint32_t lower = lower_bound(source, plan, bits, mask, strict);
    if (lower > colors) {
        return 0;
    }
    return plan.enters_live ? static_cast<TransitionOffset>(colors - lower + 1)
                            : 1;
}

__host__ __device__ TransitionOffset emit_for_state(
    Key source_key, Residues source_value, const StepPlan& plan,
    std::uint32_t colors, std::uint32_t bits, Key mask, bool strict,
    Residues moduli, TransitionOffset offset, Key* output_keys,
    Residues* output_values) {
    const std::uint32_t lower =
        lower_bound(source_key, plan, bits, mask, strict);
    if (lower > colors) {
        return 0;
    }
    const Key base = compact_key(source_key, plan, bits, mask);
    if (!plan.enters_live) {
        output_keys[offset] = base;
        output_values[offset] =
            multiply(source_value, colors - lower + 1, moduli);
        return 1;
    }
    const std::uint32_t target_position = plan.keep_count;
    TransitionOffset emitted = 0;
    for (std::uint32_t color = lower; color <= colors; ++color) {
        output_keys[offset + emitted] =
            base | (static_cast<Key>(color) << (target_position * bits));
        output_values[offset + emitted] = source_value;
        ++emitted;
    }
    return emitted;
}

__global__ void count_transitions(const Key* keys, std::size_t state_count,
                                  StepPlan plan, std::uint32_t colors,
                                  std::uint32_t bits, Key mask, bool strict,
                                  TransitionOffset* counts) {
    const std::size_t index = blockIdx.x * blockDim.x + threadIdx.x;
    if (index < state_count) {
        counts[index] = transition_count(keys[index], plan, colors, bits, mask,
                                         strict);
    }
}

__global__ void emit_transitions(
    const Key* keys, const Residues* values, std::size_t state_count,
    StepPlan plan, std::uint32_t colors, std::uint32_t bits, Key mask,
    bool strict, Residues moduli, const TransitionOffset* offsets,
    Key* output_keys, Residues* output_values) {
    const std::size_t index = blockIdx.x * blockDim.x + threadIdx.x;
    if (index < state_count) {
        static_cast<void>(emit_for_state(
            keys[index], values[index], plan, colors, bits, mask, strict,
            moduli, offsets[index], output_keys, output_values));
    }
}

struct StateLayer {
    DeviceBuffer<Key> keys;
    DeviceBuffer<Residues> values;
    std::size_t size = 0;
};

struct AdvanceResult {
    StateLayer states;
    std::size_t transitions = 0;
    float device_ms = 0.0F;
};

AdvanceResult advance_layer(const StateLayer& states, const StepPlan& plan,
                            std::uint32_t colors, std::uint32_t bits, Key mask,
                            bool strict, std::size_t maximum_transitions,
                            Residues moduli) {
    const unsigned int blocks =
        static_cast<unsigned int>((states.size + kBlockSize - 1) / kBlockSize);
    DeviceBuffer<TransitionOffset> counts(states.size);
    DeviceBuffer<TransitionOffset> offsets(states.size);
    AdvanceResult result;
    result.device_ms += time_gpu([&] {
        hipLaunchKernelGGL(count_transitions, dim3(blocks), dim3(kBlockSize), 0,
                           0, states.keys.get(), states.size, plan, colors, bits,
                           mask, strict, counts.get());
        HIP_CHECK(hipGetLastError());
    });

    std::size_t scan_bytes = 0;
    HIP_CHECK(rocprim::exclusive_scan(nullptr, scan_bytes, counts.get(),
                                      offsets.get(), TransitionOffset{0},
                                      states.size,
                                      rocprim::plus<TransitionOffset>{}));
    RawDeviceBuffer scan_storage(scan_bytes);
    result.device_ms += time_gpu([&] {
        HIP_CHECK(rocprim::exclusive_scan(
            scan_storage.get(), scan_bytes, counts.get(), offsets.get(),
            TransitionOffset{0}, states.size,
            rocprim::plus<TransitionOffset>{}));
    });

    TransitionOffset last_offset = 0;
    TransitionOffset last_count = 0;
    HIP_CHECK(hipMemcpy(&last_offset, offsets.get() + states.size - 1,
                        sizeof(last_offset), hipMemcpyDeviceToHost));
    HIP_CHECK(hipMemcpy(&last_count, counts.get() + states.size - 1,
                        sizeof(last_count), hipMemcpyDeviceToHost));
    if (last_offset > maximum_transitions ||
        last_count > maximum_transitions - last_offset) {
        throw std::runtime_error("transition count exceeds configured limit");
    }
    const TransitionOffset transition_total = last_offset + last_count;
    if (transition_total == 0) {
        return result;
    }
    result.transitions = static_cast<std::size_t>(transition_total);

    DeviceBuffer<Key> emitted_keys(result.transitions);
    DeviceBuffer<Residues> emitted_values(result.transitions);
    result.device_ms += time_gpu([&] {
        hipLaunchKernelGGL(emit_transitions, dim3(blocks), dim3(kBlockSize), 0,
                           0, states.keys.get(), states.values.get(), states.size,
                           plan, colors, bits, mask, strict, moduli,
                           offsets.get(), emitted_keys.get(),
                           emitted_values.get());
        HIP_CHECK(hipGetLastError());
    });

    DeviceBuffer<Key> sorted_keys(result.transitions);
    DeviceBuffer<Residues> sorted_values(result.transitions);
    DeviceBuffer<std::size_t> unique_count_device(1);
    const unsigned int end_bit =
        std::max(1U, static_cast<unsigned int>(plan.next_fields * bits));
    std::size_t sort_bytes = 0;
    HIP_CHECK(rocprim::radix_sort_pairs(
        nullptr, sort_bytes, emitted_keys.get(), sorted_keys.get(),
        emitted_values.get(), sorted_values.get(), result.transitions, 0,
        end_bit));
    std::size_t reduce_bytes = 0;
    HIP_CHECK(rocprim::reduce_by_key(
        nullptr, reduce_bytes, sorted_keys.get(), sorted_values.get(),
        result.transitions, emitted_keys.get(), emitted_values.get(),
        unique_count_device.get(), ModularAdd{moduli}, rocprim::equal_to<Key>{}));
    RawDeviceBuffer aggregate_storage(std::max(sort_bytes, reduce_bytes));
    result.device_ms += time_gpu([&] {
        HIP_CHECK(rocprim::radix_sort_pairs(
            aggregate_storage.get(), sort_bytes, emitted_keys.get(),
            sorted_keys.get(), emitted_values.get(), sorted_values.get(),
            result.transitions, 0, end_bit));
    });
    result.device_ms += time_gpu([&] {
        HIP_CHECK(rocprim::reduce_by_key(
            aggregate_storage.get(), reduce_bytes, sorted_keys.get(),
            sorted_values.get(), result.transitions, emitted_keys.get(),
            emitted_values.get(), unique_count_device.get(),
            ModularAdd{moduli}, rocprim::equal_to<Key>{}));
    });

    std::size_t unique_count = 0;
    HIP_CHECK(hipMemcpy(&unique_count, unique_count_device.get(),
                        sizeof(unique_count), hipMemcpyDeviceToHost));
    StateLayer compact{DeviceBuffer<Key>(unique_count),
                       DeviceBuffer<Residues>(unique_count), unique_count};
    result.device_ms += time_gpu([&] {
        HIP_CHECK(hipMemcpy(compact.keys.get(), emitted_keys.get(),
                            unique_count * sizeof(Key), hipMemcpyDeviceToDevice));
        HIP_CHECK(hipMemcpy(compact.values.get(), emitted_values.get(),
                            unique_count * sizeof(Residues),
                            hipMemcpyDeviceToDevice));
    });
    result.states = std::move(compact);
    return result;
}

std::uint64_t parse_decimal(const std::string& raw, const std::string& name,
                            std::uint64_t maximum) {
    if (raw.empty() || raw.find_first_not_of("0123456789") != std::string::npos) {
        throw std::runtime_error(name + " must be a nonnegative decimal integer");
    }
    std::uint64_t value = 0;
    try {
        value = std::stoull(raw);
    } catch (const std::exception&) {
        throw std::runtime_error(name + " is outside the supported integer range");
    }
    if (value > maximum) {
        throw std::runtime_error(name + " is outside the supported integer range");
    }
    return value;
}

std::uint32_t parse_u32(const std::string& raw, const std::string& name) {
    return static_cast<std::uint32_t>(
        parse_decimal(raw, name, std::numeric_limits<std::uint32_t>::max()));
}

std::uint32_t gcd(std::uint32_t left, std::uint32_t right) {
    while (right != 0) {
        const std::uint32_t remainder = left % right;
        left = right;
        right = remainder;
    }
    return left;
}

std::vector<std::pair<std::size_t, std::size_t>> parse_covers(
    const std::string& raw, std::size_t vertices) {
    if (raw.empty() || raw == "-") {
        return {};
    }
    std::vector<std::pair<std::size_t, std::size_t>> covers;
    std::size_t begin = 0;
    while (begin <= raw.size()) {
        const std::size_t end = raw.find(',', begin);
        const std::string item = raw.substr(begin, end - begin);
        const std::size_t separator = item.find('<');
        if (separator == std::string::npos ||
            item.find('<', separator + 1) != std::string::npos) {
            throw std::runtime_error("covers must use comma-separated lower<upper pairs");
        }
        const std::size_t lower = static_cast<std::size_t>(parse_decimal(
            item.substr(0, separator), "cover vertex", vertices - 1));
        const std::size_t upper = static_cast<std::size_t>(parse_decimal(
            item.substr(separator + 1), "cover vertex", vertices - 1));
        if (lower == upper) {
            throw std::runtime_error("a cover cannot be a self-loop");
        }
        covers.emplace_back(lower, upper);
        if (end == std::string::npos) {
            break;
        }
        begin = end + 1;
    }
    std::sort(covers.begin(), covers.end());
    covers.erase(std::unique(covers.begin(), covers.end()), covers.end());
    return covers;
}

PlannedPoset build_plans(
    std::size_t vertices,
    const std::vector<std::pair<std::size_t, std::size_t>>& input_covers) {
    if (vertices == 0 || vertices > kMaximumVertices) {
        throw std::runtime_error("vertex count must lie in 1..128");
    }
    std::vector<std::vector<std::size_t>> children(vertices);
    std::vector<std::size_t> indegree(vertices, 0);
    for (const auto& [lower, upper] : input_covers) {
        children[lower].push_back(upper);
        ++indegree[upper];
    }
    std::vector<std::size_t> order;
    order.reserve(vertices);
    for (std::size_t step = 0; step < vertices; ++step) {
        std::size_t selected = vertices;
        for (std::size_t vertex = 0; vertex < vertices; ++vertex) {
            if (indegree[vertex] == 0 &&
                std::find(order.begin(), order.end(), vertex) == order.end()) {
                selected = vertex;
                break;
            }
        }
        if (selected == vertices) {
            throw std::runtime_error("cover relation contains a directed cycle");
        }
        order.push_back(selected);
        indegree[selected] = std::numeric_limits<std::size_t>::max();
        for (const std::size_t child : children[selected]) {
            --indegree[child];
        }
    }

    std::vector<std::size_t> new_index(vertices);
    for (std::size_t index = 0; index < vertices; ++index) {
        new_index[order[index]] = index;
    }
    std::vector<std::vector<std::size_t>> parents(vertices);
    children.assign(vertices, {});
    for (const auto& [old_lower, old_upper] : input_covers) {
        const std::size_t lower = new_index[old_lower];
        const std::size_t upper = new_index[old_upper];
        parents[upper].push_back(lower);
        children[lower].push_back(upper);
    }
    for (std::size_t vertex = 0; vertex < vertices; ++vertex) {
        std::sort(parents[vertex].begin(), parents[vertex].end());
        parents[vertex].erase(
            std::unique(parents[vertex].begin(), parents[vertex].end()),
            parents[vertex].end());
        std::sort(children[vertex].begin(), children[vertex].end());
        children[vertex].erase(
            std::unique(children[vertex].begin(), children[vertex].end()),
            children[vertex].end());
    }

    std::vector<std::size_t> last_child(vertices, vertices);
    for (std::size_t vertex = 0; vertex < vertices; ++vertex) {
        if (!children[vertex].empty()) {
            last_child[vertex] = children[vertex].back();
        }
    }
    std::vector<std::size_t> live;
    std::vector<std::size_t> live_index(vertices, vertices);
    PlannedPoset result;
    result.steps.reserve(vertices);
    for (std::size_t vertex = 0; vertex < vertices; ++vertex) {
        StepPlan plan{};
        plan.current_fields = static_cast<std::uint32_t>(live.size());
        for (const std::size_t parent : parents[vertex]) {
            if (live_index[parent] == vertices) {
                throw std::runtime_error("internal frontier-plan error");
            }
            plan.parent_positions[plan.parent_count++] =
                static_cast<std::uint32_t>(live_index[parent]);
        }
        std::vector<std::size_t> next_live;
        for (std::size_t position = 0; position < live.size(); ++position) {
            if (last_child[live[position]] != vertex) {
                plan.keep_positions[plan.keep_count++] =
                    static_cast<std::uint32_t>(position);
                next_live.push_back(live[position]);
            }
        }
        plan.enters_live = last_child[vertex] != vertices;
        if (plan.enters_live) {
            next_live.push_back(vertex);
        }
        plan.next_fields = static_cast<std::uint32_t>(next_live.size());
        result.maximum_frontier =
            std::max(result.maximum_frontier, next_live.size());
        std::fill(live_index.begin(), live_index.end(), vertices);
        for (std::size_t index = 0; index < next_live.size(); ++index) {
            live_index[next_live[index]] = index;
        }
        live = std::move(next_live);
        result.steps.push_back(plan);
    }
    return result;
}

std::uint32_t bits_for(std::uint32_t colors) {
    std::uint32_t bits = 1;
    while (bits < 32 && (std::uint64_t{1} << bits) <= colors) {
        ++bits;
    }
    return bits;
}

Residues make_moduli(const std::vector<std::uint32_t>& values) {
    if (values.size() != kResidueLanes) {
        throw std::runtime_error("modulus count does not match compiled residue lanes");
    }
    Residues result{};
    for (std::size_t lane = 0; lane < kResidueLanes; ++lane) {
        const std::uint32_t modulus = values[lane];
        if (modulus < 2 || modulus >= (std::uint32_t{1} << 31)) {
            throw std::runtime_error("each modulus must lie in 2..2^31");
        }
        for (std::size_t earlier = 0; earlier < lane; ++earlier) {
            if (gcd(result.values[earlier], modulus) != 1) {
                throw std::runtime_error("moduli must be pairwise coprime");
            }
        }
        result.values[lane] = modulus;
    }
    return result;
}

std::vector<std::uint32_t> parse_moduli(const std::string& raw) {
    std::vector<std::uint32_t> result;
    std::size_t begin = 0;
    while (begin <= raw.size()) {
        const std::size_t end = raw.find(',', begin);
        result.push_back(parse_u32(raw.substr(begin, end - begin), "modulus"));
        if (end == std::string::npos) {
            break;
        }
        begin = end + 1;
    }
    return result;
}

}  // namespace

#ifdef EHRGPU_HOST_TEST
namespace {

struct HostState {
    Key key;
    Residues value;
};

std::uint32_t host_count(
    std::size_t vertices,
    const std::vector<std::pair<std::size_t, std::size_t>>& covers,
    std::uint32_t colors, bool strict) {
    const PlannedPoset poset = build_plans(vertices, covers);
    const std::uint32_t bits = bits_for(colors);
    if (poset.maximum_frontier * bits > 128) {
        throw std::runtime_error("frontier does not fit in a 128-bit key");
    }
    const Key mask = (static_cast<Key>(1) << bits) - 1;
    const Residues moduli = make_moduli(std::vector<std::uint32_t>(
        kResidueLanes, kDefaultModuli[0]));
    Residues one{};
    std::fill(std::begin(one.values), std::end(one.values), 1);
    std::vector<HostState> states{{0, one}};
    for (const StepPlan& plan : poset.steps) {
        std::vector<HostState> emitted;
        for (const HostState& state : states) {
            const TransitionOffset count = transition_count(
                state.key, plan, colors, bits, mask, strict);
            const std::size_t offset = emitted.size();
            emitted.resize(offset + count);
            std::vector<Key> keys(count);
            std::vector<Residues> values(count);
            const TransitionOffset found = emit_for_state(
                state.key, state.value, plan, colors, bits, mask, strict,
                moduli, 0, keys.data(), values.data());
            if (found != count) {
                throw std::runtime_error("host count/emission mismatch");
            }
            for (std::size_t index = 0; index < count; ++index) {
                emitted[offset + index] = HostState{keys[index], values[index]};
            }
        }
        std::sort(emitted.begin(), emitted.end(),
                  [](const HostState& left, const HostState& right) {
                      return left.key < right.key;
                  });
        states.clear();
        for (const HostState& state : emitted) {
            if (!states.empty() && states.back().key == state.key) {
                states.back().value =
                    ModularAdd{moduli}(states.back().value, state.value);
            } else {
                states.push_back(state);
            }
        }
    }
    return states.empty() ? 0 : states.front().value.values[0];
}

std::uint32_t brute_count(
    std::size_t vertices,
    const std::vector<std::pair<std::size_t, std::size_t>>& covers,
    std::uint32_t colors, bool strict) {
    std::vector<std::uint32_t> values(vertices, 1);
    std::uint32_t count = 0;
    while (true) {
        const bool valid = std::all_of(
            covers.begin(), covers.end(), [&](const auto& cover) {
                return strict ? values[cover.first] < values[cover.second]
                              : values[cover.first] <= values[cover.second];
            });
        count += valid ? 1 : 0;
        std::size_t position = 0;
        while (position < vertices && values[position] == colors) {
            values[position] = 1;
            ++position;
        }
        if (position == vertices) {
            break;
        }
        ++values[position];
    }
    return count;
}

}  // namespace

int main() try {
    if (host_count(3, {{0, 1}, {1, 2}}, 4, false) != 20 ||
        host_count(3, {}, 4, false) != 64 ||
        host_count(3, {{0, 2}, {1, 2}}, 4, false) != 30 ||
        host_count(3, {{0, 1}, {1, 2}}, 4, true) != 4 ||
        host_count(3, {}, 4, true) != 64 ||
        host_count(3, {{0, 2}, {1, 2}}, 4, true) != 14 ||
        host_count(3, {{2, 0}, {0, 1}}, 4, false) != 20) {
        throw std::runtime_error("order-poset host regression failed");
    }
    bool rejected_cycle = false;
    try {
        static_cast<void>(build_plans(2, {{0, 1}, {1, 0}}));
    } catch (const std::runtime_error&) {
        rejected_cycle = true;
    }
    if (!rejected_cycle) {
        throw std::runtime_error("cycle validation regression failed");
    }
    for (std::size_t vertices = 1; vertices <= 5; ++vertices) {
        std::vector<std::pair<std::size_t, std::size_t>> possible;
        for (std::size_t lower = 0; lower < vertices; ++lower) {
            for (std::size_t upper = lower + 1; upper < vertices; ++upper) {
                possible.emplace_back(lower, upper);
            }
        }
        const std::uint64_t graph_count = std::uint64_t{1} << possible.size();
        for (std::uint64_t graph = 0; graph < graph_count; ++graph) {
            std::vector<std::pair<std::size_t, std::size_t>> covers;
            for (std::size_t edge = 0; edge < possible.size(); ++edge) {
                if ((graph & (std::uint64_t{1} << edge)) != 0) {
                    covers.push_back(possible[edge]);
                }
            }
            for (std::uint32_t colors = 1; colors <= 4; ++colors) {
                for (const bool strict : {false, true}) {
                    if (host_count(vertices, covers, colors, strict) !=
                        brute_count(vertices, covers, colors, strict)) {
                        throw std::runtime_error(
                            "exhaustive order-poset host comparison failed");
                    }
                }
            }
        }
    }
    std::cout << "order-poset host regressions passed\n";
    return 0;
} catch (const std::exception& error) {
    std::cerr << "error: " << error.what() << '\n';
    return 1;
}
#else
int main(int argc, char** argv) try {
    if (argc < 5 || argc > 7) {
        std::cerr << "usage: order_polytope_gpu_resident VERTICES COVERS "
                     "COLORS weak|strict [MAX_TRANSITIONS] [MODULI]\n";
        return 2;
    }
    const std::size_t vertices = static_cast<std::size_t>(parse_decimal(
        argv[1], "vertices", kMaximumVertices));
    if (vertices == 0) {
        throw std::runtime_error("vertices must be positive");
    }
    const auto covers = parse_covers(argv[2], vertices);
    const std::uint32_t colors = static_cast<std::uint32_t>(parse_decimal(
        argv[3], "colors", std::numeric_limits<std::uint32_t>::max() - 1));
    const std::string mode = argv[4];
    if (mode != "weak" && mode != "strict") {
        throw std::runtime_error("mode must be weak or strict");
    }
    const bool strict = mode == "strict";
    const std::size_t maximum_transitions =
        argc >= 6
            ? static_cast<std::size_t>(parse_decimal(
                  argv[5], "maximum transitions",
                  std::numeric_limits<std::uint32_t>::max() - 1))
            : kDefaultMaximumTransitions;
    const std::vector<std::uint32_t> modulus_values =
        argc == 7 ? parse_moduli(argv[6])
                  : std::vector<std::uint32_t>(
                        kDefaultModuli, kDefaultModuli + kResidueLanes);
    const Residues moduli = make_moduli(modulus_values);
    if (colors == 0) {
        std::cout << "{\"vertices\":" << vertices
                  << ",\"colors\":0,\"mode\":\"" << mode
                  << "\",\"residue\":0,\"residues\":[";
        for (std::size_t lane = 0; lane < kResidueLanes; ++lane) {
            std::cout << (lane == 0 ? "" : ",") << 0;
        }
        std::cout << "]}\n";
        return 0;
    }

    const PlannedPoset poset = build_plans(vertices, covers);
    const std::uint32_t bits = bits_for(colors);
    if (poset.maximum_frontier * bits > 128) {
        throw std::runtime_error("frontier values do not fit in a 128-bit key");
    }
    const Key mask = (static_cast<Key>(1) << bits) - 1;
    int device_count = 0;
    HIP_CHECK(hipGetDeviceCount(&device_count));
    if (device_count == 0) {
        throw std::runtime_error("no HIP device found");
    }
    HIP_CHECK(hipSetDevice(0));
    hipDeviceProp_t properties{};
    HIP_CHECK(hipGetDeviceProperties(&properties, 0));
    Residues one{};
    std::fill(std::begin(one.values), std::end(one.values), 1);
    const Key zero = 0;
    StateLayer states{DeviceBuffer<Key>(1), DeviceBuffer<Residues>(1), 1};
    HIP_CHECK(hipMemcpy(states.keys.get(), &zero, sizeof(zero),
                        hipMemcpyHostToDevice));
    HIP_CHECK(hipMemcpy(states.values.get(), &one, sizeof(one),
                        hipMemcpyHostToDevice));

    const auto start = std::chrono::steady_clock::now();
    double device_ms = 0.0;
    std::size_t peak_states = 1;
    std::size_t peak_transitions = 1;
    for (std::size_t vertex = 0; vertex < poset.steps.size(); ++vertex) {
        const std::size_t source_states = states.size;
        AdvanceResult next = advance_layer(
            states, poset.steps[vertex], colors, bits, mask, strict,
            maximum_transitions, moduli);
        states = std::move(next.states);
        device_ms += next.device_ms;
        peak_states = std::max(peak_states, states.size);
        peak_transitions = std::max(peak_transitions, next.transitions);
        std::cerr << "{\"vertex\":" << vertex
                  << ",\"source_states\":" << source_states
                  << ",\"transitions\":" << next.transitions
                  << ",\"states\":" << states.size << "}\n";
        if (states.size == 0) {
            break;
        }
    }

    Residues answer{};
    if (states.size != 0) {
        HIP_CHECK(hipMemcpy(&answer, states.values.get(), sizeof(answer),
                            hipMemcpyDeviceToHost));
    }
    const double total_ms = std::chrono::duration<double, std::milli>(
                                std::chrono::steady_clock::now() - start)
                                .count();
    std::cout << std::fixed << std::setprecision(3)
              << "{\"device\":\"" << properties.name << "\","
              << "\"vertices\":" << vertices << ','
              << "\"covers\":" << covers.size() << ','
              << "\"colors\":" << colors << ','
              << "\"mode\":\"" << mode << "\","
              << "\"bits_per_value\":" << bits << ','
              << "\"maximum_frontier\":" << poset.maximum_frontier << ','
              << "\"residue\":" << answer.values[0] << ','
              << "\"moduli\":[";
    for (std::size_t lane = 0; lane < kResidueLanes; ++lane) {
        std::cout << (lane == 0 ? "" : ",") << moduli.values[lane];
    }
    std::cout << "],\"residues\":[";
    for (std::size_t lane = 0; lane < kResidueLanes; ++lane) {
        std::cout << (lane == 0 ? "" : ",") << answer.values[lane];
    }
    std::cout << "],\"peak_states\":" << peak_states
              << ",\"peak_transitions\":" << peak_transitions
              << ",\"device_work_ms\":" << device_ms
              << ",\"total_ms\":" << total_ms << "}\n";
    return 0;
} catch (const std::exception& error) {
    std::cerr << "error: " << error.what() << '\n';
    return 1;
}
#endif
