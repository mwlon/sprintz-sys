// Thin extern "C" shim over sprintz C++ functions, since sprintz has no
// extern "C" linkage of its own.

#include "sprintz.h"

extern "C" {

int64_t sprintz_c_compress_delta_8b(const uint8_t* src, uint32_t len,
                                    int8_t* dest, uint16_t ndims) {
    return sprintz_compress_delta_8b(src, len, dest, ndims, /*write_size=*/true);
}
int64_t sprintz_c_decompress_delta_8b(const int8_t* src, uint8_t* dest) {
    return sprintz_decompress_delta_8b(src, dest);
}

int64_t sprintz_c_compress_xff_8b(const uint8_t* src, uint32_t len,
                                   int8_t* dest, uint16_t ndims) {
    return sprintz_compress_xff_8b(src, len, dest, ndims, /*write_size=*/true);
}
int64_t sprintz_c_decompress_xff_8b(const int8_t* src, uint8_t* dest) {
    return sprintz_decompress_xff_8b(src, dest);
}

int64_t sprintz_c_compress_delta_16b(const uint16_t* src, uint32_t len,
                                     int16_t* dest, uint16_t ndims) {
    return sprintz_compress_delta_16b(src, len, dest, ndims, /*write_size=*/true);
}
int64_t sprintz_c_decompress_delta_16b(const int16_t* src, uint16_t* dest) {
    return sprintz_decompress_delta_16b(src, dest);
}

int64_t sprintz_c_compress_xff_16b(const uint16_t* src, uint32_t len,
                                    int16_t* dest, uint16_t ndims) {
    return sprintz_compress_xff_16b(src, len, dest, ndims, /*write_size=*/true);
}
int64_t sprintz_c_decompress_xff_16b(const int16_t* src, uint16_t* dest) {
    return sprintz_decompress_xff_16b(src, dest);
}

} // extern "C"
