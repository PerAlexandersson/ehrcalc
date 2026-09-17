#include <algorithm>
#include <chrono>
#include <cstdint>
#include <cstdlib>
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
static_assert(sizeof(TransitionOffset) == 8);

namespace {

#ifndef EHRGPU_RESIDUE_LANES
#define EHRGPU_RESIDUE_LANES 1
#endif

constexpr std::size_t kResidueLanes = EHRGPU_RESIDUE_LANES;
static_assert(kResidueLanes >= 1 && kResidueLanes <= 8);
constexpr std::uint32_t kDefaultModuli[] = {
    2'147'483'647U, 2'147'483'629U, 2'147'483'587U, 2'147'483'579U,
    2'147'483'563U, 2'147'483'549U, 2'147'483'543U, 2'147'483'497U,
};
constexpr std::size_t kMaximumRows = 32;
#ifndef EHRGPU_MAX_STRIP_SIZE
#define EHRGPU_MAX_STRIP_SIZE 128
#endif
constexpr std::uint32_t kMaximumStripSize = EHRGPU_MAX_STRIP_SIZE;
static_assert(kMaximumStripSize >= 1 && kMaximumStripSize <= 640);
constexpr std::size_t kDefaultMaximumTransitions = 150'000'000;
constexpr std::size_t kGpuMemoryReserveBytes = std::size_t{2} << 30;
#ifndef EHRGPU_BLOCK_SIZE
#define EHRGPU_BLOCK_SIZE 128
#endif
constexpr unsigned int kBlockSize = EHRGPU_BLOCK_SIZE;
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

struct DeviceLevelBounds {
    std::uint32_t lower[kMaximumRows];
    std::uint32_t upper[kMaximumRows];
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

struct IncrementBounds {
    std::uint32_t minimum;
    std::uint32_t maximum;
};

__host__ __device__ bool increment_bounds(
    Key source_key, const DeviceShape& shape,
    const DeviceLevelBounds& level_bounds, std::uint32_t row_lo,
    std::uint32_t row_hi, std::uint32_t forbidden_mask,
    std::uint32_t strict_lower_mask, std::uint32_t strict_diagonal_mask,
    std::uint32_t row, IncrementBounds& bounds) {
    const std::uint32_t row_bit = std::uint32_t{1} << row;
    const bool forced_zero = row < row_lo || row >= row_hi ||
                             (forbidden_mask & row_bit) != 0;
    const bool strict_lower = (strict_lower_mask & row_bit) != 0;
    const bool strict_diagonal = row > 0 &&
                                 (strict_diagonal_mask & row_bit) != 0;
    const std::uint32_t alpha = static_cast<std::uint32_t>(
        (source_key >> (shape.bits * row)) & shape.mask);
    const std::uint32_t gap = row == 0
                                  ? std::numeric_limits<std::uint32_t>::max()
                                  : static_cast<std::uint32_t>(
                                        (source_key >> (shape.bits * (row - 1))) &
                                        shape.mask) -
                                        alpha;
    if (forced_zero) {
        if (strict_lower || (strict_diagonal && gap == 0)) {
            return false;
        }
        bounds = IncrementBounds{0, 0};
    } else {
        if (strict_diagonal && gap == 0) {
            return false;
        }
        const std::uint32_t shape_capacity = shape.outer[row] - alpha;
        const std::uint32_t diagonal_capacity = strict_diagonal ? gap - 1 : gap;
        bounds.minimum = strict_lower ? 1 : 0;
        bounds.maximum = shape_capacity < diagonal_capacity ? shape_capacity
                                                            : diagonal_capacity;
    }
    if (alpha > level_bounds.upper[row]) {
        return false;
    }
    const std::uint32_t bound_minimum =
        level_bounds.lower[row] > alpha ? level_bounds.lower[row] - alpha : 0;
    const std::uint32_t bound_maximum = level_bounds.upper[row] - alpha;
    bounds.minimum = bounds.minimum > bound_minimum ? bounds.minimum
                                                    : bound_minimum;
    bounds.maximum = bounds.maximum < bound_maximum ? bounds.maximum
                                                    : bound_maximum;
    return bounds.minimum <= bounds.maximum;
}

__host__ __device__ std::uint32_t count_bounded_compositions(
    const std::uint32_t* extra_capacity, std::uint32_t rows,
    std::uint32_t target, std::uint32_t saturation_limit) {
    std::uint32_t ways[kMaximumStripSize + 1]{};
    ways[0] = 1;
    for (std::uint32_t row = 0; row < rows; ++row) {
        const std::uint32_t capacity = extra_capacity[row];
        const std::uint32_t first = target > capacity ? target - capacity : 0;
        std::uint64_t window = 0;
        for (std::uint32_t index = first; index <= target; ++index) {
            window += ways[index];
        }
        for (std::int32_t total = static_cast<std::int32_t>(target);
             total >= 0; --total) {
            const std::uint32_t index = static_cast<std::uint32_t>(total);
            const std::uint32_t saved = ways[index];
            ways[index] = static_cast<std::uint32_t>(
                window > saturation_limit ? saturation_limit : window);
            window -= saved;
            if (index > capacity) {
                window += ways[index - capacity - 1];
            }
        }
    }
    return ways[target];
}

__host__ __device__ std::uint32_t count_extensions_fast(
    Key source_key, const DeviceShape& shape,
    const DeviceLevelBounds& level_bounds, std::uint32_t strip_size,
    std::uint32_t row_lo, std::uint32_t row_hi,
    std::uint32_t forbidden_mask, std::uint32_t strict_lower_mask,
    std::uint32_t strict_diagonal_mask,
    std::uint32_t saturation_limit) {
    std::uint32_t extra_capacity[kMaximumRows]{};
    std::uint64_t mandatory = 0;
    std::uint64_t total_capacity = 0;
    for (std::uint32_t row = 0; row < shape.rows; ++row) {
        IncrementBounds bounds{};
        if (!increment_bounds(source_key, shape, level_bounds, row_lo, row_hi,
                              forbidden_mask, strict_lower_mask,
                              strict_diagonal_mask, row, bounds)) {
            return 0;
        }
        mandatory += bounds.minimum;
        extra_capacity[row] = bounds.maximum - bounds.minimum;
        total_capacity += extra_capacity[row];
    }
    if (mandatory > strip_size) {
        return 0;
    }
    const std::uint32_t target =
        strip_size - static_cast<std::uint32_t>(mandatory);
    if (total_capacity < target) {
        return 0;
    }
    return count_bounded_compositions(extra_capacity, shape.rows, target,
                                      saturation_limit);
}

__global__ void count_transitions(const Key* state_keys,
                                  std::size_t state_count, DeviceShape shape,
                                  DeviceLevelBounds level_bounds,
                                  std::uint32_t strip_size,
                                  std::uint32_t row_lo, std::uint32_t row_hi,
                                  std::uint32_t forbidden_mask,
                                  std::uint32_t strict_lower_mask,
                                  std::uint32_t strict_diagonal_mask,
                                  std::uint32_t saturation_limit,
                                  TransitionOffset* counts) {
    const std::size_t index = blockIdx.x * blockDim.x + threadIdx.x;
    if (index < state_count) {
        counts[index] = count_extensions_fast(state_keys[index], shape,
                                              level_bounds, strip_size,
                                              row_lo, row_hi, forbidden_mask,
                                              strict_lower_mask,
                                              strict_diagonal_mask,
                                              saturation_limit);
    }
}

__host__ __device__ TransitionOffset enumerate_extensions(
    Key source_key, Residues source_value, const DeviceShape& shape,
    const DeviceLevelBounds& level_bounds, std::uint32_t strip_size,
    std::uint32_t row_lo, std::uint32_t row_hi,
    std::uint32_t forbidden_mask, std::uint32_t strict_lower_mask,
    std::uint32_t strict_diagonal_mask,
    TransitionOffset output_offset, Key* output_keys,
    Residues* output_values) {
    std::uint32_t row_minimum[kMaximumRows]{};
    std::uint32_t row_maximum[kMaximumRows]{};
    std::uint32_t suffix_minimum[kMaximumRows + 1]{};
    std::uint32_t suffix_maximum[kMaximumRows + 1]{};
    std::uint32_t choices[kMaximumRows]{};
    for (std::uint32_t row = 0; row < shape.rows; ++row) {
        IncrementBounds bounds{};
        if (!increment_bounds(source_key, shape, level_bounds, row_lo, row_hi,
                              forbidden_mask, strict_lower_mask,
                              strict_diagonal_mask, row, bounds)) {
            return 0;
        }
        row_minimum[row] = bounds.minimum;
        row_maximum[row] = bounds.maximum;
    }
    for (std::uint32_t row = shape.rows; row-- > 0;) {
        const std::uint64_t minimum =
            static_cast<std::uint64_t>(suffix_minimum[row + 1]) +
            row_minimum[row];
        suffix_minimum[row] = static_cast<std::uint32_t>(
            minimum <= strip_size ? minimum : std::uint64_t{strip_size} + 1);
        const std::uint64_t maximum =
            static_cast<std::uint64_t>(suffix_maximum[row + 1]) +
            row_maximum[row];
        suffix_maximum[row] = static_cast<std::uint32_t>(
            maximum < strip_size ? maximum : strip_size);
    }
    if (strip_size < suffix_minimum[0] || strip_size > suffix_maximum[0]) {
        return 0;
    }

    int row = 0;
    std::uint32_t remaining = strip_size;
    TransitionOffset found = 0;
    Key delta_key = 0;
    choices[0] = remaining > suffix_maximum[1]
                     ? remaining - suffix_maximum[1]
                     : row_minimum[0];
    if (choices[0] < row_minimum[0]) {
        choices[0] = row_minimum[0];
    }
    while (row >= 0) {
        if (row == static_cast<int>(shape.rows)) {
            if (remaining == 0) {
                output_keys[output_offset + found] = source_key + delta_key;
                output_values[output_offset + found] = source_value;
                ++found;
            }
            --row;
            if (row >= 0) {
                delta_key -= static_cast<Key>(choices[row])
                             << (shape.bits * row);
                remaining += choices[row];
                ++choices[row];
            }
            continue;
        }

        const std::uint32_t choice = choices[row];
        const std::uint32_t maximum_here =
            remaining >= suffix_minimum[row + 1]
                ? remaining - suffix_minimum[row + 1]
                : 0;
        if (choice >= row_minimum[row] && choice <= row_maximum[row] &&
            choice <= remaining && choice <= maximum_here) {
            delta_key += static_cast<Key>(choice) << (shape.bits * row);
            remaining -= choice;
            ++row;
            if (row < static_cast<int>(shape.rows)) {
                choices[row] =
                    remaining > suffix_maximum[row + 1]
                        ? remaining - suffix_maximum[row + 1]
                        : row_minimum[row];
                if (choices[row] < row_minimum[row]) {
                    choices[row] = row_minimum[row];
                }
            }
        } else {
            --row;
            if (row >= 0) {
                delta_key -= static_cast<Key>(choices[row])
                             << (shape.bits * row);
                remaining += choices[row];
                ++choices[row];
            }
        }
    }
    return found;
}

__global__ void emit_transitions(
    const Key* state_keys, const Residues* state_values,
    std::size_t state_count, DeviceShape shape, DeviceLevelBounds level_bounds,
    std::uint32_t strip_size,
    std::uint32_t row_lo, std::uint32_t row_hi,
    std::uint32_t forbidden_mask, std::uint32_t strict_lower_mask,
    std::uint32_t strict_diagonal_mask,
    const TransitionOffset* offsets, TransitionOffset offset_base,
    Key* transition_keys, Residues* transition_values) {
    const std::size_t index = blockIdx.x * blockDim.x + threadIdx.x;
    if (index < state_count) {
        static_cast<void>(enumerate_extensions(
            state_keys[index], state_values[index], shape, level_bounds,
            strip_size, row_lo,
            row_hi, forbidden_mask, strict_lower_mask, strict_diagonal_mask,
            offsets[index] - offset_base, transition_keys, transition_values));
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
    DeviceBuffer<Residues> values;
    std::size_t size = 0;
};

struct AdvanceResult {
    StateLayer states;
    std::size_t transitions = 0;
    std::size_t chunks = 0;
    std::size_t largest_chunk = 0;
    std::size_t sort_temp_bytes = 0;
    std::size_t memory_free_bytes = 0;
    std::size_t memory_required_bytes = 0;
    LayerTimings timings;
};

struct TransitionChunk {
    std::size_t source_begin = 0;
    std::size_t source_end = 0;
    TransitionOffset offset_base = 0;
    std::size_t transitions = 0;
};

std::vector<TransitionChunk> plan_source_chunks(
    const std::vector<TransitionOffset>& counts, std::size_t chunk_limit,
    std::size_t total_limit) {
    if (chunk_limit == 0 || total_limit == 0) {
        throw std::runtime_error("transition limits must be positive");
    }
    std::vector<TransitionChunk> chunks;
    TransitionOffset total = 0;
    std::size_t begin = 0;
    TransitionOffset chunk_total = 0;
    TransitionOffset chunk_base = 0;
    for (std::size_t source = 0; source < counts.size(); ++source) {
        const TransitionOffset count = counts[source];
        if (count > chunk_limit) {
            throw std::runtime_error(
                "single-source transition count " + std::to_string(count) +
                " exceeds chunk limit " + std::to_string(chunk_limit));
        }
        if (chunk_total != 0 && count > chunk_limit - chunk_total) {
            chunks.push_back(TransitionChunk{
                begin, source, chunk_base, static_cast<std::size_t>(chunk_total)});
            begin = source;
            chunk_base = total;
            chunk_total = 0;
        }
        if (count > std::numeric_limits<TransitionOffset>::max() - total) {
            throw std::runtime_error("total transition count overflows");
        }
        total += count;
        chunk_total += count;
        if (total > total_limit) {
            throw std::runtime_error(
                "transition count " + std::to_string(total) +
                " exceeds configured total limit " + std::to_string(total_limit));
        }
    }
    if (chunk_total != 0) {
        chunks.push_back(TransitionChunk{
            begin, counts.size(), chunk_base, static_cast<std::size_t>(chunk_total)});
    }
    return chunks;
}

std::size_t checked_bytes(std::size_t count, std::size_t width,
                          const char* description) {
    if (count > std::numeric_limits<std::size_t>::max() / width) {
        throw std::runtime_error(std::string(description) + " byte count overflows");
    }
    return count * width;
}

enum class TransitionDisposition { empty, proceed, over_limit };

TransitionDisposition classify_transition_total(TransitionOffset last_offset,
                                                 TransitionOffset last_count,
                                                 std::size_t maximum) {
    if (last_offset > maximum || last_count > maximum - last_offset) {
        return TransitionDisposition::over_limit;
    }
    return last_offset == 0 && last_count == 0
               ? TransitionDisposition::empty
               : TransitionDisposition::proceed;
}

struct ReductionSizes {
    std::size_t record_bytes = 0;
    std::size_t sort_temp_bytes = 0;
    std::size_t reduce_temp_bytes = 0;
    std::size_t scratch_bytes = 0;
};

ReductionSizes reduction_sizes(std::size_t count, const DeviceShape& shape,
                               Residues moduli) {
    ReductionSizes sizes;
    sizes.record_bytes = checked_bytes(
        count, sizeof(Key) + sizeof(Residues), "transition record");
    HIP_CHECK(rocprim::radix_sort_pairs(
        nullptr, sizes.sort_temp_bytes, static_cast<Key*>(nullptr),
        static_cast<Key*>(nullptr), static_cast<Residues*>(nullptr),
        static_cast<Residues*>(nullptr), count, 0, shape.rows * shape.bits));
    HIP_CHECK(rocprim::reduce_by_key(
        nullptr, sizes.reduce_temp_bytes, static_cast<Key*>(nullptr),
        static_cast<Residues*>(nullptr), count, static_cast<Key*>(nullptr),
        static_cast<Residues*>(nullptr), static_cast<std::size_t*>(nullptr),
        ModularAdd{moduli}, rocprim::equal_to<Key>{}));
    sizes.scratch_bytes = std::max(sizes.sort_temp_bytes, sizes.reduce_temp_bytes);
    return sizes;
}

void require_gpu_memory(std::size_t additional_bytes,
                        std::size_t& minimum_free_bytes,
                        std::size_t& maximum_required_bytes) {
    std::size_t free_bytes = 0;
    std::size_t total_bytes = 0;
    HIP_CHECK(hipMemGetInfo(&free_bytes, &total_bytes));
    minimum_free_bytes = minimum_free_bytes == 0
                             ? free_bytes
                             : std::min(minimum_free_bytes, free_bytes);
    maximum_required_bytes = std::max(maximum_required_bytes, additional_bytes);
    if (free_bytes <= kGpuMemoryReserveBytes ||
        additional_bytes > free_bytes - kGpuMemoryReserveBytes) {
        throw std::runtime_error(
            "GPU byte budget exceeded: need " + std::to_string(additional_bytes) +
            " additional bytes plus " + std::to_string(kGpuMemoryReserveBytes) +
            " reserved, only " + std::to_string(free_bytes) + " free of " +
            std::to_string(total_bytes));
    }
}

void add_timings(LayerTimings& target, const LayerTimings& source) {
    target.count_ms += source.count_ms;
    target.scan_ms += source.scan_ms;
    target.emit_ms += source.emit_ms;
    target.sort_ms += source.sort_ms;
    target.reduce_ms += source.reduce_ms;
    target.compact_ms += source.compact_ms;
}

StateLayer reduce_records(DeviceBuffer<Key> keys,
                          DeviceBuffer<Residues> values, std::size_t count,
                          const DeviceShape& shape, Residues moduli,
                          const ReductionSizes& sizes, LayerTimings& timings) {
    std::size_t sort_temp_bytes = sizes.sort_temp_bytes;
    std::size_t reduce_temp_bytes = sizes.reduce_temp_bytes;
    std::size_t unique_count = 0;
    {
        DeviceBuffer<Key> sorted_keys(count);
        DeviceBuffer<Residues> sorted_values(count);
        DeviceBuffer<std::size_t> unique_count_device(1);
        RawDeviceBuffer aggregate_temp(sizes.scratch_bytes);
        timings.sort_ms += time_gpu([&] {
            HIP_CHECK(rocprim::radix_sort_pairs(
                aggregate_temp.get(), sort_temp_bytes, keys.get(),
                sorted_keys.get(), values.get(), sorted_values.get(), count,
                0, shape.rows * shape.bits));
        });
        timings.reduce_ms += time_gpu([&] {
            HIP_CHECK(rocprim::reduce_by_key(
                aggregate_temp.get(), reduce_temp_bytes, sorted_keys.get(),
                sorted_values.get(), count, keys.get(), values.get(),
                unique_count_device.get(), ModularAdd{moduli},
                rocprim::equal_to<Key>{}));
        });
        HIP_CHECK(hipMemcpy(&unique_count, unique_count_device.get(),
                            sizeof(unique_count), hipMemcpyDeviceToHost));
    }
    StateLayer compact{DeviceBuffer<Key>(unique_count),
                       DeviceBuffer<Residues>(unique_count), unique_count};
    timings.compact_ms += time_gpu([&] {
        HIP_CHECK(hipMemcpy(compact.keys.get(), keys.get(),
                            unique_count * sizeof(Key), hipMemcpyDeviceToDevice));
        HIP_CHECK(hipMemcpy(compact.values.get(), values.get(),
                            unique_count * sizeof(Residues),
                            hipMemcpyDeviceToDevice));
    });
    return compact;
}

StateLayer merge_layers(StateLayer&& left, StateLayer&& right,
                        const DeviceShape& shape, Residues moduli,
                        LayerTimings& timings, std::size_t& sort_temp_bytes,
                        std::size_t& minimum_free_bytes,
                        std::size_t& maximum_required_bytes) {
    if (left.size == 0) {
        return std::move(right);
    }
    if (right.size == 0) {
        return std::move(left);
    }
    const std::size_t count = left.size + right.size;
    const ReductionSizes sizes = reduction_sizes(count, shape, moduli);
    const std::size_t required =
        checked_bytes(2, sizes.record_bytes, "merge record layers") +
        sizes.scratch_bytes + sizeof(std::size_t);
    require_gpu_memory(required, minimum_free_bytes, maximum_required_bytes);
    sort_temp_bytes = std::max(sort_temp_bytes, sizes.sort_temp_bytes);
    DeviceBuffer<Key> keys(count);
    DeviceBuffer<Residues> values(count);
    HIP_CHECK(hipMemcpy(keys.get(), left.keys.get(), left.size * sizeof(Key),
                        hipMemcpyDeviceToDevice));
    HIP_CHECK(hipMemcpy(keys.get() + left.size, right.keys.get(),
                        right.size * sizeof(Key), hipMemcpyDeviceToDevice));
    HIP_CHECK(hipMemcpy(values.get(), left.values.get(),
                        left.size * sizeof(Residues), hipMemcpyDeviceToDevice));
    HIP_CHECK(hipMemcpy(values.get() + left.size, right.values.get(),
                        right.size * sizeof(Residues), hipMemcpyDeviceToDevice));
    left = StateLayer{};
    right = StateLayer{};
    return reduce_records(std::move(keys), std::move(values), count, shape,
                          moduli, sizes, timings);
}

AdvanceResult advance_layer(const StateLayer& states, const DeviceShape& shape,
                            const DeviceLevelBounds& level_bounds,
                            std::uint32_t strip_size, std::uint32_t row_lo,
                            std::uint32_t row_hi,
                            std::uint32_t forbidden_mask,
                            std::uint32_t strict_lower_mask,
                            std::uint32_t strict_diagonal_mask,
                            std::size_t maximum_transitions,
                            std::size_t maximum_chunk_transitions,
                            Residues moduli) {
    const unsigned int blocks =
        static_cast<unsigned int>((states.size + kBlockSize - 1) / kBlockSize);
    AdvanceResult result;
    std::size_t scan_temp_bytes = 0;
    HIP_CHECK(rocprim::exclusive_scan(
        nullptr, scan_temp_bytes, static_cast<TransitionOffset*>(nullptr),
        static_cast<TransitionOffset*>(nullptr), TransitionOffset{0}, states.size,
        rocprim::plus<TransitionOffset>{}));
    const std::size_t preparation_bytes = checked_bytes(
        states.size, 2 * sizeof(TransitionOffset), "transition count and offset") +
        scan_temp_bytes;
    require_gpu_memory(preparation_bytes, result.memory_free_bytes,
                       result.memory_required_bytes);
    DeviceBuffer<TransitionOffset> counts(states.size);
    DeviceBuffer<TransitionOffset> offsets(states.size);
    RawDeviceBuffer scan_temp(scan_temp_bytes);
    result.timings.count_ms = time_gpu([&] {
        hipLaunchKernelGGL(count_transitions, dim3(blocks), dim3(kBlockSize), 0, 0,
                           states.keys.get(), states.size, shape, level_bounds,
                           strip_size, row_lo, row_hi, forbidden_mask,
                           strict_lower_mask, strict_diagonal_mask,
                           static_cast<std::uint32_t>(maximum_chunk_transitions + 1),
                           counts.get());
        HIP_CHECK(hipGetLastError());
    });
    result.timings.scan_ms = time_gpu([&] {
        HIP_CHECK(rocprim::exclusive_scan(
            scan_temp.get(), scan_temp_bytes, counts.get(), offsets.get(),
            TransitionOffset{0}, states.size,
            rocprim::plus<TransitionOffset>{}));
    });
    std::vector<TransitionOffset> host_counts(states.size);
    HIP_CHECK(hipMemcpy(host_counts.data(), counts.get(),
                        states.size * sizeof(TransitionOffset),
                        hipMemcpyDeviceToHost));
    const std::vector<TransitionChunk> chunks = plan_source_chunks(
        host_counts, maximum_chunk_transitions, maximum_transitions);
    result.chunks = chunks.size();
    StateLayer accumulator;
    for (const TransitionChunk& chunk : chunks) {
        result.transitions += chunk.transitions;
        result.largest_chunk = std::max(result.largest_chunk, chunk.transitions);
        const ReductionSizes sizes = reduction_sizes(chunk.transitions, shape, moduli);
        const std::size_t required =
            checked_bytes(2, sizes.record_bytes, "chunk record layers") +
            sizes.scratch_bytes + sizeof(std::size_t);
        require_gpu_memory(required, result.memory_free_bytes,
                           result.memory_required_bytes);
        result.sort_temp_bytes = std::max(result.sort_temp_bytes,
                                          sizes.sort_temp_bytes);
        DeviceBuffer<Key> transition_keys(chunk.transitions);
        DeviceBuffer<Residues> transition_values(chunk.transitions);
        const std::size_t source_count = chunk.source_end - chunk.source_begin;
        const unsigned int chunk_blocks = static_cast<unsigned int>(
            (source_count + kBlockSize - 1) / kBlockSize);
        LayerTimings chunk_timings;
        chunk_timings.emit_ms = time_gpu([&] {
            hipLaunchKernelGGL(
                emit_transitions, dim3(chunk_blocks), dim3(kBlockSize), 0, 0,
                states.keys.get() + chunk.source_begin,
                states.values.get() + chunk.source_begin, source_count, shape,
                level_bounds, strip_size, row_lo, row_hi, forbidden_mask,
                strict_lower_mask, strict_diagonal_mask,
                offsets.get() + chunk.source_begin, chunk.offset_base,
                transition_keys.get(), transition_values.get());
            HIP_CHECK(hipGetLastError());
        });
        StateLayer partial = reduce_records(
            std::move(transition_keys), std::move(transition_values),
            chunk.transitions, shape, moduli, sizes, chunk_timings);
        accumulator = merge_layers(
            std::move(accumulator), std::move(partial), shape, moduli,
            chunk_timings, result.sort_temp_bytes, result.memory_free_bytes,
            result.memory_required_bytes);
        add_timings(result.timings, chunk_timings);
    }
    result.states = std::move(accumulator);
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

std::vector<std::uint32_t> parse_list(const std::string& raw,
                                      const std::string& name) {
    if (raw.empty() || raw == "-") {
        return {};
    }
    std::vector<std::uint32_t> values;
    std::size_t begin = 0;
    while (begin <= raw.size()) {
        const std::size_t end = raw.find(',', begin);
        values.push_back(parse_u32(raw.substr(begin, end - begin), name));
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

std::uint32_t gcd(std::uint32_t left, std::uint32_t right) {
    while (right != 0) {
        const std::uint32_t remainder = left % right;
        left = right;
        right = remainder;
    }
    return left;
}

}  // namespace

#ifdef EHRGPU_HOST_TEST
struct HostState {
    Key key;
    Residues value;
};

std::uint32_t brute_bounded_compositions(const std::uint32_t* capacities,
                                         std::uint32_t rows,
                                         std::uint32_t row,
                                         std::uint32_t remaining) {
    if (row == rows) {
        return remaining == 0 ? 1 : 0;
    }
    std::uint32_t count = 0;
    const std::uint32_t maximum =
        capacities[row] < remaining ? capacities[row] : remaining;
    for (std::uint32_t value = 0; value <= maximum; ++value) {
        count += brute_bounded_compositions(capacities, rows, row + 1,
                                              remaining - value);
    }
    return count;
}

std::uint32_t host_masked_face_count(
    const std::vector<std::uint32_t>& upper,
    const std::vector<std::uint32_t>& lower,
    const std::vector<std::uint32_t>& forbidden,
    const std::vector<std::uint32_t>& strict_lower,
    const std::vector<std::uint32_t>& strict_diagonal) {
    const std::vector<std::uint32_t> outer{2, 1};
    const std::vector<std::uint32_t> weight{1, 1, 1};
    const DeviceShape shape = make_shape(outer);
    const std::uint32_t zero_parts[2]{0, 0};
    Residues one{};
    std::fill(std::begin(one.values), std::end(one.values), 1);
    Residues moduli{};
    std::fill(std::begin(moduli.values), std::end(moduli.values),
              kDefaultModuli[0]);
    std::vector<HostState> states{{pack_parts(shape, zero_parts), one}};

    for (std::size_t label = 0; label < weight.size(); ++label) {
        DeviceLevelBounds level_bounds{};
        for (std::uint32_t row = 0; row < shape.rows; ++row) {
            level_bounds.upper[row] = shape.outer[row];
        }
        const std::uint32_t row_lo =
            lower.empty() ? 0 : std::min<std::uint32_t>(
                                      lower[label] == 0 ? 0 : lower[label] - 1,
                                      shape.rows);
        const std::uint32_t row_hi =
            upper.empty() ? shape.rows
                          : std::min<std::uint32_t>(upper[label], shape.rows);
        std::vector<HostState> emitted;
        for (const HostState& source : states) {
            const std::uint32_t count = count_extensions_fast(
                source.key, shape, level_bounds, weight[label], row_lo, row_hi,
                forbidden[label], strict_lower[label], strict_diagonal[label],
                1'000'000);
            const std::size_t begin = emitted.size();
            emitted.resize(begin + count);
            std::vector<Key> keys(count);
            std::vector<Residues> values(count);
            const TransitionOffset found = enumerate_extensions(
                source.key, source.value, shape, level_bounds, weight[label],
                row_lo, row_hi,
                forbidden[label], strict_lower[label], strict_diagonal[label],
                0, keys.data(), values.data());
            if (found != count) {
                throw std::runtime_error(
                    "host count/emission transition mismatch");
            }
            for (std::size_t index = 0; index < count; ++index) {
                emitted[begin + index] = HostState{keys[index], values[index]};
            }
        }
        std::sort(emitted.begin(), emitted.end(),
                  [](const HostState& left, const HostState& right) {
                      return left.key < right.key;
                  });
        states.clear();
        for (const HostState& item : emitted) {
            if (!states.empty() && states.back().key == item.key) {
                states.back().value = ModularAdd{moduli}(states.back().value,
                                                        item.value);
            } else {
                states.push_back(item);
            }
        }
    }

    const Key target = pack_parts(shape, outer.data());
    const auto found = std::lower_bound(
        states.begin(), states.end(), target,
        [](const HostState& state, Key key) { return state.key < key; });
    return found != states.end() && found->key == target
               ? found->value.values[0]
               : 0;
}

int main() try {
    for (std::uint32_t a = 0; a <= 4; ++a) {
        for (std::uint32_t b = 0; b <= 4; ++b) {
            for (std::uint32_t c = 0; c <= 4; ++c) {
                const std::uint32_t capacities[]{a, b, c};
                for (std::uint32_t target = 0; target <= a + b + c + 1;
                     ++target) {
                    const std::uint32_t expected =
                        brute_bounded_compositions(capacities, 3, 0, target);
                    for (const std::uint32_t limit : {1U, 2U, 5U, 1'000U}) {
                        const std::uint32_t actual = count_bounded_compositions(
                            capacities, 3, target, limit);
                        if (actual != std::min(expected, limit)) {
                            throw std::runtime_error(
                                "sliding-window composition regression failed");
                        }
                    }
                }
            }
        }
    }
    {
        const DeviceShape shape = make_shape({3, 2});
        const std::uint32_t source_parts[]{1, 0};
        const Key source = pack_parts(shape, source_parts);
        DeviceLevelBounds bounds{};
        bounds.lower[0] = 2;
        bounds.upper[0] = 2;
        bounds.upper[1] = 2;
        if (count_extensions_fast(source, shape, bounds, 1, 0, 2, 0, 0, 0,
                                  100) != 1 ||
            count_extensions_fast(source, shape, bounds, 1, 0, 2, 1, 0, 0,
                                  100) != 0) {
            throw std::runtime_error("level lower-bound regression failed");
        }
        Residues one{};
        std::fill(std::begin(one.values), std::end(one.values), 1);
        Key destination{};
        Residues destination_value{};
        if (enumerate_extensions(source, one, shape, bounds, 1, 0, 2, 0, 0,
                                 0, 0, &destination, &destination_value) != 1) {
            throw std::runtime_error("bounded destination emission failed");
        }
        const std::uint32_t expected_parts[]{2, 0};
        if (destination != pack_parts(shape, expected_parts)) {
            throw std::runtime_error("bounded destination set regression failed");
        }
    }
    if (classify_transition_total(4'304'268'255ULL, 64'609'566ULL,
                                  150'000'000) !=
        TransitionDisposition::over_limit) {
        throw std::runtime_error("wide transition overflow regression failed");
    }
    if (classify_transition_total(0, 0, 150'000'000) !=
        TransitionDisposition::empty) {
        throw std::runtime_error("empty transition regression failed");
    }
    if (classify_transition_total(100, 5, 150'000'000) !=
        TransitionDisposition::proceed) {
        throw std::runtime_error("ordinary transition regression failed");
    }
    if (!plan_source_chunks({0, 0, 0}, 2, 10).empty()) {
        throw std::runtime_error("all-zero chunk plan regression failed");
    }
    const std::vector<TransitionChunk> planned =
        plan_source_chunks({0, 2, 0, 1, 1, 0, 2}, 2, 6);
    if (planned.size() != 3 || planned[0].source_begin != 0 ||
        planned[0].source_end != 3 || planned[0].offset_base != 0 ||
        planned[0].transitions != 2 || planned[1].source_begin != 3 ||
        planned[1].source_end != 6 || planned[1].offset_base != 2 ||
        planned[1].transitions != 2 || planned[2].source_begin != 6 ||
        planned[2].source_end != 7 || planned[2].offset_base != 4 ||
        planned[2].transitions != 2) {
        throw std::runtime_error("source chunk planning regression failed");
    }
    bool rejected_single_source = false;
    try {
        static_cast<void>(plan_source_chunks({3}, 2, 10));
    } catch (const std::runtime_error&) {
        rejected_single_source = true;
    }
    bool rejected_total = false;
    try {
        static_cast<void>(plan_source_chunks({2, 2}, 2, 3));
    } catch (const std::runtime_error&) {
        rejected_total = true;
    }
    const std::vector<TransitionChunk> wide_plan = plan_source_chunks(
        {std::numeric_limits<std::uint32_t>::max(), 1},
        std::numeric_limits<std::uint32_t>::max(),
        std::numeric_limits<TransitionOffset>::max());
    if (!rejected_single_source || !rejected_total || wide_plan.size() != 2 ||
        wide_plan[1].offset_base + wide_plan[1].transitions <=
            std::numeric_limits<std::uint32_t>::max()) {
        throw std::runtime_error("chunk limit regression failed");
    }
    bool rejected_suffix = false;
    try {
        static_cast<void>(parse_u32("1junk", "test value"));
    } catch (const std::runtime_error&) {
        rejected_suffix = true;
    }
    bool rejected_overflow = false;
    try {
        static_cast<void>(parse_u32("4294967297", "test value"));
    } catch (const std::runtime_error&) {
        rejected_overflow = true;
    }
    if (!rejected_suffix || !rejected_overflow) {
        throw std::runtime_error("strict parser regression failed");
    }
    const std::vector<std::uint32_t> zero_masks(3, 0);
    if (host_masked_face_count({}, {}, zero_masks, zero_masks, zero_masks) !=
        2) {
        throw std::runtime_error("ordinary host transition regression failed");
    }
    const std::vector<std::uint32_t> internal_hole{0, 1, 0};
    const std::vector<std::uint32_t> strict_top_last{0, 0, 1};
    if (host_masked_face_count({}, {}, internal_hole, strict_top_last,
                               zero_masks) != 1) {
        throw std::runtime_error("masked strict face regression failed");
    }
    const std::vector<std::uint32_t> upper{2, 2, 1};
    const std::vector<std::uint32_t> strict_forced_diagonal{0, 0, 2};
    if (host_masked_face_count(upper, {}, zero_masks, zero_masks,
                               strict_forced_diagonal) != 0) {
        throw std::runtime_error(
            "flag-forced strict diagonal regression failed");
    }
    std::cout << "host regressions passed\n";
    return 0;
} catch (const std::exception& error) {
    std::cerr << "error: " << error.what() << '\n';
    return 1;
}
#else
int main(int argc, char** argv) try {
    if ((argc < 7 || argc > 9) && argc != 12 && argc != 14) {
        std::cerr << "usage: packed_flagged_kostka_gpu_resident DILATION OUTER "
                     "INNER WEIGHT UPPER_FLAGS LOWER_FLAGS [MAX_TRANSITIONS] "
                     "[MODULI [FORBIDDEN_MASKS STRICT_LOWER_MASKS "
                     "STRICT_DIAGONAL_MASKS [LEVEL_LOWER_BOUNDS "
                     "LEVEL_UPPER_BOUNDS]]]\n";
        return 2;
    }
    const std::uint32_t dilation = parse_u32(argv[1], "dilation");
    if (dilation == 0) {
        throw std::runtime_error("dilation must be positive");
    }
    std::vector<std::uint32_t> outer = parse_list(argv[2], "outer part");
    std::vector<std::uint32_t> inner = parse_list(argv[3], "inner part");
    std::vector<std::uint32_t> weight = parse_list(argv[4], "weight part");
    const std::vector<std::uint32_t> upper = parse_list(argv[5], "upper flag");
    const std::vector<std::uint32_t> lower = parse_list(argv[6], "lower flag");
    std::vector<std::uint32_t> forbidden_masks;
    std::vector<std::uint32_t> strict_lower_masks;
    std::vector<std::uint32_t> strict_diagonal_masks;
    std::vector<std::uint32_t> level_lower_bounds;
    std::vector<std::uint32_t> level_upper_bounds;
    if (argc == 12 || argc == 14) {
        forbidden_masks = parse_list(argv[9], "forbidden-row mask");
        strict_lower_masks = parse_list(argv[10], "strict-lower mask");
        strict_diagonal_masks = parse_list(argv[11], "strict-diagonal mask");
    }
    if (argc == 14) {
        level_lower_bounds = parse_list(argv[12], "level lower bound");
        level_upper_bounds = parse_list(argv[13], "level upper bound");
    }
    const std::size_t maximum_transitions =
        argc >= 8
            ? static_cast<std::size_t>(parse_decimal(
                  argv[7], "maximum transitions",
                  std::numeric_limits<std::size_t>::max()))
            : kDefaultMaximumTransitions;
    const char* chunk_environment = std::getenv("EHRGPU_CHUNK_TRANSITIONS");
    const std::size_t maximum_chunk_transitions = chunk_environment == nullptr
        ? std::min(maximum_transitions, kDefaultMaximumTransitions)
        : static_cast<std::size_t>(parse_decimal(
              chunk_environment, "EHRGPU_CHUNK_TRANSITIONS",
              std::numeric_limits<std::size_t>::max()));
    std::vector<std::uint32_t> modulus_values;
    if (argc == 9 || argc == 12 || argc == 14) {
        modulus_values = parse_list(argv[8], "modulus");
    } else {
        modulus_values.assign(kDefaultModuli, kDefaultModuli + kResidueLanes);
    }
    if (modulus_values.size() != kResidueLanes) {
        throw std::runtime_error("modulus count does not match compiled residue lanes");
    }
    Residues moduli{};
    for (std::size_t lane = 0; lane < kResidueLanes; ++lane) {
        const std::uint32_t modulus = modulus_values[lane];
        if (modulus < 2 || modulus >= (std::uint32_t{1} << 31)) {
            throw std::runtime_error("each modulus must lie in 2..2^31");
        }
        for (std::size_t previous = 0; previous < lane; ++previous) {
            if (gcd(moduli.values[previous], modulus) != 1) {
                throw std::runtime_error("moduli must be pairwise coprime");
            }
        }
        moduli.values[lane] = modulus;
    }
    if (maximum_transitions >= std::numeric_limits<std::uint32_t>::max()) {
        throw std::runtime_error("maximum transitions must be below 2^32-1");
    }
    if (maximum_chunk_transitions == 0 ||
        maximum_chunk_transitions > maximum_transitions ||
        maximum_chunk_transitions >= std::numeric_limits<std::uint32_t>::max()) {
        throw std::runtime_error(
            "EHRGPU_CHUNK_TRANSITIONS must lie in 1..maximum transitions");
    }
    scale(outer, dilation);
    scale(inner, dilation);
    scale(weight, dilation);
    scale(level_lower_bounds, dilation);
    scale(level_upper_bounds, dilation);
    if (std::any_of(weight.begin(), weight.end(), [](std::uint32_t strip_size) {
            return strip_size > kMaximumStripSize;
        })) {
        throw std::runtime_error("scaled weight part exceeds GPU strip-size limit");
    }
    if (outer.empty() || inner.size() > outer.size() || sum(outer) < sum(inner) ||
        sum(outer) - sum(inner) != sum(weight)) {
        throw std::runtime_error("incompatible shape and weight");
    }
    if (outer.size() > kMaximumRows) {
        throw std::runtime_error("shape exceeds GPU row limit");
    }
    if ((!upper.empty() && upper.size() != weight.size()) ||
        (!lower.empty() && lower.size() != weight.size())) {
        throw std::runtime_error("flag lengths must match weight length");
    }
    auto normalize_masks = [&](std::vector<std::uint32_t>& masks,
                               const char* name) {
        if (masks.empty()) {
            masks.resize(weight.size(), 0);
        } else if (masks.size() != weight.size()) {
            throw std::runtime_error(std::string(name) +
                                     " length must match weight length");
        }
    };
    normalize_masks(forbidden_masks, "forbidden-row mask");
    normalize_masks(strict_lower_masks, "strict-lower mask");
    normalize_masks(strict_diagonal_masks, "strict-diagonal mask");
    const std::uint32_t allowed_row_mask =
        outer.size() == std::numeric_limits<std::uint32_t>::digits
            ? std::numeric_limits<std::uint32_t>::max()
            : (std::uint32_t{1} << outer.size()) - 1;
    for (std::size_t label = 0; label < weight.size(); ++label) {
        if (((forbidden_masks[label] | strict_lower_masks[label] |
              strict_diagonal_masks[label]) &
             ~allowed_row_mask) != 0) {
            throw std::runtime_error("constraint mask contains a bit outside the shape rows");
        }
        if ((strict_diagonal_masks[label] & 1) != 0) {
            throw std::runtime_error(
                "strict-diagonal masks cannot contain the first-row bit");
        }
        if (weight[label] == 0 &&
            (strict_lower_masks[label] != 0 ||
             strict_diagonal_masks[label] != 0)) {
            throw std::runtime_error(
                "zero-weight labels must have zero strictness masks");
        }
    }
    inner.resize(outer.size(), 0);
    for (std::size_t row = 0; row < outer.size(); ++row) {
        if ((row > 0 && outer[row - 1] < outer[row]) ||
            (row > 0 && inner[row - 1] < inner[row]) || inner[row] > outer[row]) {
            throw std::runtime_error("invalid partition containment");
        }
    }
    const std::size_t expected_level_bounds = weight.size() * outer.size();
    if (level_lower_bounds.empty() && level_upper_bounds.empty()) {
        level_lower_bounds.resize(expected_level_bounds, 0);
        level_upper_bounds.reserve(expected_level_bounds);
        for (std::size_t label = 0; label < weight.size(); ++label) {
            level_upper_bounds.insert(level_upper_bounds.end(), outer.begin(),
                                      outer.end());
        }
    } else if (level_lower_bounds.size() != expected_level_bounds ||
               level_upper_bounds.size() != expected_level_bounds) {
        throw std::runtime_error(
            "level-bound lengths must equal labels times shape rows");
    }
    for (std::size_t index = 0; index < expected_level_bounds; ++index) {
        if (level_lower_bounds[index] > level_upper_bounds[index] ||
            level_upper_bounds[index] > outer[index % outer.size()]) {
            throw std::runtime_error("invalid level-bound interval");
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
    Residues initial_value{};
    std::fill(std::begin(initial_value.values), std::end(initial_value.values), 1);
    StateLayer states{DeviceBuffer<Key>(1), DeviceBuffer<Residues>(1), 1};
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
        DeviceLevelBounds level_bounds{};
        for (std::size_t row = 0; row < outer.size(); ++row) {
            const std::size_t index = label * outer.size() + row;
            level_bounds.lower[row] = level_lower_bounds[index];
            level_bounds.upper[row] = level_upper_bounds[index];
        }
        const std::size_t source_states = states.size;
        AdvanceResult next = advance_layer(
            states, shape, level_bounds, weight[label], row_lo, row_hi,
            forbidden_masks[label], strict_lower_masks[label],
            strict_diagonal_masks[label], maximum_transitions,
            maximum_chunk_transitions, moduli);
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
                  << ",\"chunks\":" << next.chunks
                  << ",\"largest_chunk\":" << next.largest_chunk
                  << ",\"states\":" << states.size
                  << ",\"count_ms\":" << timing.count_ms
                  << ",\"scan_ms\":" << timing.scan_ms
                  << ",\"emit_ms\":" << timing.emit_ms
                  << ",\"sort_ms\":" << timing.sort_ms
                  << ",\"reduce_ms\":" << timing.reduce_ms
                  << ",\"compact_ms\":" << timing.compact_ms
                  << ",\"sort_temp_bytes\":" << next.sort_temp_bytes
                  << ",\"memory_free_bytes\":" << next.memory_free_bytes
                  << ",\"memory_required_bytes\":"
                  << next.memory_required_bytes
                  << ",\"memory_reserve_bytes\":" << kGpuMemoryReserveBytes
                  << ",\"maximum_transitions\":" << maximum_transitions
                  << ",\"maximum_chunk_transitions\":"
                  << maximum_chunk_transitions
                  << "}\n";
        if (states.size == 0) {
            break;
        }
    }

    std::vector<Key> final_keys(states.size);
    std::vector<Residues> final_values(states.size);
    if (states.size != 0) {
        HIP_CHECK(hipMemcpy(final_keys.data(), states.keys.get(),
                            states.size * sizeof(Key), hipMemcpyDeviceToHost));
        HIP_CHECK(hipMemcpy(final_values.data(), states.values.get(),
                            states.size * sizeof(Residues),
                            hipMemcpyDeviceToHost));
    }
    const Key target = pack_parts(shape, outer.data());
    const auto found = std::lower_bound(final_keys.begin(), final_keys.end(), target);
    Residues answer{};
    if (found != final_keys.end() && *found == target) {
        answer = final_values[static_cast<std::size_t>(found - final_keys.begin())];
    }
    const double total_ms = std::chrono::duration<double, std::milli>(
                                std::chrono::steady_clock::now() - total_start)
                                .count();
    std::cout << std::fixed << std::setprecision(3)
              << "{\"device\":\"" << properties.name << "\","
              << "\"dilation\":" << dilation << ','
              << "\"rows\":" << outer.size() << ','
              << "\"bits_per_row\":" << shape.bits << ','
              << "\"modulus\":" << moduli.values[0] << ','
              << "\"residue\":" << answer.values[0] << ','
              << "\"moduli\":[";
    for (std::size_t lane = 0; lane < kResidueLanes; ++lane) {
        std::cout << (lane == 0 ? "" : ",") << moduli.values[lane];
    }
    std::cout << "],\"residues\":[";
    for (std::size_t lane = 0; lane < kResidueLanes; ++lane) {
        std::cout << (lane == 0 ? "" : ",") << answer.values[lane];
    }
    std::cout << "],"
              << "\"peak_states\":" << peak_states << ','
              << "\"peak_transitions\":" << peak_transitions << ','
              << "\"device_work_ms\":" << device_work_ms << ','
              << "\"total_ms\":" << total_ms << "}\n";
    return 0;
} catch (const std::exception& error) {
    std::cerr << "error: " << error.what() << '\n';
    return 1;
}
#endif
