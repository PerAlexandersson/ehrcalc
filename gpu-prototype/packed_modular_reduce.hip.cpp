#include <algorithm>
#include <chrono>
#include <cstring>
#include <cstdint>
#include <cstdlib>
#include <iomanip>
#include <iostream>
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

std::uint64_t mix(std::uint64_t value) {
    value ^= value >> 30;
    value *= 0xbf58476d1ce4e5b9ULL;
    value ^= value >> 27;
    value *= 0x94d049bb133111ebULL;
    return value ^ (value >> 31);
}

Key make_key(std::uint64_t group) {
    const std::uint64_t low = mix(group ^ 0x123456789abcdef0ULL);
    const std::uint64_t high = mix(group ^ 0xfedcba9876543210ULL);
    return (static_cast<Key>(high) << 64) | static_cast<Key>(low);
}

std::uint32_t make_value(std::uint64_t index) {
    return static_cast<std::uint32_t>(mix(index + 17) % (kModulus - 1)) + 1;
}

struct Record {
    Key key;
    std::uint32_t value;
};

std::uint32_t reduce_mod(std::uint32_t left, std::uint32_t right) {
    return ModularAdd{kModulus}(left, right);
}

std::pair<std::uint64_t, std::uint64_t> split_key(Key key) {
    return {static_cast<std::uint64_t>(key >> 64),
            static_cast<std::uint64_t>(key)};
}

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

float median(std::vector<float> values) {
    std::sort(values.begin(), values.end());
    return values[values.size() / 2];
}

template <typename T>
class DeviceBuffer {
  public:
    explicit DeviceBuffer(std::size_t size) : size_(size) {
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
    const T* get() const { return data_; }
    std::size_t size() const { return size_; }

  private:
    T* data_ = nullptr;
    std::size_t size_;
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

std::vector<Record> cpu_reference(const std::vector<Key>& keys,
                                  const std::vector<std::uint32_t>& values) {
    std::vector<Record> records;
    records.reserve(keys.size());
    for (std::size_t index = 0; index < keys.size(); ++index) {
        records.push_back({keys[index], values[index]});
    }
    std::sort(records.begin(), records.end(), [](const Record& left,
                                                  const Record& right) {
        return left.key < right.key;
    });

    std::vector<Record> reduced;
    reduced.reserve(records.size());
    for (const Record& record : records) {
        if (reduced.empty() || reduced.back().key != record.key) {
            reduced.push_back(record);
        } else {
            reduced.back().value = reduce_mod(reduced.back().value, record.value);
        }
    }
    return reduced;
}

}  // namespace

int main(int argc, char** argv) try {
    const std::size_t record_count =
        argc > 1 ? std::stoull(argv[1]) : 1'000'000ULL;
    const std::size_t requested_groups =
        argc > 2 ? std::stoull(argv[2]) : record_count / 4;
    const std::size_t repetitions = argc > 3 ? std::stoull(argv[3]) : 5;
    if (record_count == 0 || requested_groups == 0 || repetitions == 0) {
        throw std::runtime_error("record and group counts must be positive");
    }

    int device_count = 0;
    HIP_CHECK(hipGetDeviceCount(&device_count));
    if (device_count == 0) {
        throw std::runtime_error("no HIP device found");
    }
    HIP_CHECK(hipSetDevice(0));
    hipDeviceProp_t properties{};
    HIP_CHECK(hipGetDeviceProperties(&properties, 0));

    std::vector<Key> host_keys(record_count);
    std::vector<std::uint32_t> host_values(record_count);
    for (std::size_t index = 0; index < record_count; ++index) {
        const std::uint64_t group =
            mix(index * 0x9e3779b97f4a7c15ULL) % requested_groups;
        host_keys[index] = make_key(group);
        host_values[index] = make_value(index);
    }

    const auto cpu_start = std::chrono::steady_clock::now();
    const std::vector<Record> expected = cpu_reference(host_keys, host_values);
    const auto cpu_stop = std::chrono::steady_clock::now();

    DeviceBuffer<Key> keys_input(record_count);
    DeviceBuffer<Key> keys_sorted(record_count);
    DeviceBuffer<Key> keys_unique(record_count);
    DeviceBuffer<std::uint32_t> values_input(record_count);
    DeviceBuffer<std::uint32_t> values_sorted(record_count);
    DeviceBuffer<std::uint32_t> values_reduced(record_count);
    DeviceBuffer<std::size_t> unique_count_device(1);

    const float host_to_device_ms = time_gpu([&] {
        HIP_CHECK(hipMemcpy(keys_input.get(), host_keys.data(),
                            record_count * sizeof(Key), hipMemcpyHostToDevice));
        HIP_CHECK(hipMemcpy(values_input.get(), host_values.data(),
                            record_count * sizeof(std::uint32_t),
                            hipMemcpyHostToDevice));
    });

    std::size_t sort_temp_bytes = 0;
    HIP_CHECK(rocprim::radix_sort_pairs(
        nullptr, sort_temp_bytes, keys_input.get(), keys_sorted.get(),
        values_input.get(), values_sorted.get(), record_count));
    std::size_t reduce_temp_bytes = 0;
    HIP_CHECK(rocprim::reduce_by_key(
        nullptr, reduce_temp_bytes, keys_sorted.get(), values_sorted.get(),
        record_count, keys_unique.get(), values_reduced.get(),
        unique_count_device.get(), ModularAdd{kModulus}, rocprim::equal_to<Key>{}));
    RawDeviceBuffer temporary(std::max(sort_temp_bytes, reduce_temp_bytes));

    // Warm up both device primitives before collecting timings.
    HIP_CHECK(rocprim::radix_sort_pairs(
        temporary.get(), sort_temp_bytes, keys_input.get(), keys_sorted.get(),
        values_input.get(), values_sorted.get(), record_count));
    HIP_CHECK(rocprim::reduce_by_key(
        temporary.get(), reduce_temp_bytes, keys_sorted.get(), values_sorted.get(),
        record_count, keys_unique.get(), values_reduced.get(),
        unique_count_device.get(), ModularAdd{kModulus}, rocprim::equal_to<Key>{}));
    HIP_CHECK(hipDeviceSynchronize());

    std::vector<float> sort_times;
    std::vector<float> reduce_times;
    sort_times.reserve(repetitions);
    reduce_times.reserve(repetitions);
    for (std::size_t repetition = 0; repetition < repetitions; ++repetition) {
        sort_times.push_back(time_gpu([&] {
            HIP_CHECK(rocprim::radix_sort_pairs(
                temporary.get(), sort_temp_bytes, keys_input.get(), keys_sorted.get(),
                values_input.get(), values_sorted.get(), record_count));
        }));
        reduce_times.push_back(time_gpu([&] {
            HIP_CHECK(rocprim::reduce_by_key(
                temporary.get(), reduce_temp_bytes, keys_sorted.get(), values_sorted.get(),
                record_count, keys_unique.get(), values_reduced.get(),
                unique_count_device.get(), ModularAdd{kModulus}, rocprim::equal_to<Key>{}));
        }));
    }
    const float sort_milliseconds = median(sort_times);
    const float reduce_milliseconds = median(reduce_times);

    std::size_t unique_count = 0;
    const float count_copy_ms = time_gpu([&] {
        HIP_CHECK(hipMemcpy(&unique_count, unique_count_device.get(), sizeof(unique_count),
                            hipMemcpyDeviceToHost));
    });
    std::vector<Key> actual_keys(unique_count);
    std::vector<std::uint32_t> actual_values(unique_count);
    const float device_to_host_ms = count_copy_ms + time_gpu([&] {
        HIP_CHECK(hipMemcpy(actual_keys.data(), keys_unique.get(),
                            unique_count * sizeof(Key), hipMemcpyDeviceToHost));
        HIP_CHECK(hipMemcpy(actual_values.data(), values_reduced.get(),
                            unique_count * sizeof(std::uint32_t),
                            hipMemcpyDeviceToHost));
    });

    if (unique_count != expected.size()) {
        throw std::runtime_error("GPU and CPU unique counts differ");
    }
    for (std::size_t index = 0; index < unique_count; ++index) {
        if (actual_keys[index] != expected[index].key ||
            actual_values[index] != expected[index].value) {
            const auto [actual_high, actual_low] = split_key(actual_keys[index]);
            const auto [expected_high, expected_low] = split_key(expected[index].key);
            std::cerr << "mismatch at " << index << ": actual_key=" << std::hex
                      << actual_high << actual_low << " expected_key="
                      << expected_high << expected_low << std::dec
                      << " actual_value=" << actual_values[index]
                      << " expected_value=" << expected[index].value << '\n';
            return 2;
        }
    }

    const double cpu_milliseconds =
        std::chrono::duration<double, std::milli>(cpu_stop - cpu_start).count();
    const float gpu_pipeline_milliseconds = host_to_device_ms + sort_milliseconds
                                            + reduce_milliseconds + device_to_host_ms;
    std::cout << std::fixed << std::setprecision(3)
              << "{\"device\":\"" << properties.name << "\","
              << "\"records\":" << record_count << ','
              << "\"requested_groups\":" << requested_groups << ','
              << "\"unique_groups\":" << unique_count << ','
              << "\"modulus\":" << kModulus << ','
              << "\"repetitions\":" << repetitions << ','
              << "\"host_to_device_ms\":" << host_to_device_ms << ','
              << "\"sort_ms_median\":" << sort_milliseconds << ','
              << "\"reduce_ms_median\":" << reduce_milliseconds << ','
              << "\"device_to_host_ms\":" << device_to_host_ms << ','
              << "\"gpu_pipeline_ms\":" << gpu_pipeline_milliseconds << ','
              << "\"cpu_reference_ms\":" << cpu_milliseconds << ','
              << "\"sort_temp_bytes\":" << sort_temp_bytes << ','
              << "\"reduce_temp_bytes\":" << reduce_temp_bytes << ','
              << "\"verified\":true}\n";
    return 0;
} catch (const std::exception& error) {
    std::cerr << "error: " << error.what() << '\n';
    return 1;
}
