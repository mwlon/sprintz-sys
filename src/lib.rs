// sprintz requires AVX2 and only compiles on x86_64.
#[cfg(target_arch = "x86_64")]
extern "C" {
    /// Compress `len` u8 values from `src` into `dest` using delta + RLE encoding.
    ///
    /// # Parameters
    /// - `src`: input data (number of elements = `len`)
    /// - `len`: number of **elements** (not bytes)
    /// - `dest`: output buffer; must be at least `len * 3 / 2 + 64` bytes
    /// - `ndims`: number of dimensions; use `1` for a flat 1-D array
    ///
    /// # Returns
    /// Number of bytes written to `dest`, or negative on error.
    pub fn sprintz_c_compress_delta_8b(src: *const u8, len: u32, dest: *mut i8, ndims: u16) -> i64;

    /// Decompress delta-encoded data produced by [`sprintz_c_compress_delta_8b`].
    ///
    /// The original length is encoded in the compressed stream.
    ///
    /// # Parameters
    /// - `src`: compressed data
    /// - `dest`: output buffer; must be at least `original_len + 64` bytes
    ///
    /// # Returns
    /// Number of bytes written to `dest`, or negative on error.
    pub fn sprintz_c_decompress_delta_8b(src: *const i8, dest: *mut u8) -> i64;

    /// Compress `len` u8 values using XFF (delta-of-delta) + RLE encoding.
    pub fn sprintz_c_compress_xff_8b(src: *const u8, len: u32, dest: *mut i8, ndims: u16) -> i64;

    /// Decompress XFF-encoded data produced by [`sprintz_c_compress_xff_8b`].
    pub fn sprintz_c_decompress_xff_8b(src: *const i8, dest: *mut u8) -> i64;

    /// Compress `len` u16 values using delta + RLE encoding.
    pub fn sprintz_c_compress_delta_16b(
        src: *const u16,
        len: u32,
        dest: *mut i16,
        ndims: u16,
    ) -> i64;

    /// Decompress delta-encoded data produced by [`sprintz_c_compress_delta_16b`].
    pub fn sprintz_c_decompress_delta_16b(src: *const i16, dest: *mut u16) -> i64;

    /// Compress `len` u16 values using XFF (delta-of-delta) + RLE encoding.
    pub fn sprintz_c_compress_xff_16b(src: *const u16, len: u32, dest: *mut i16, ndims: u16)
        -> i64;

    /// Decompress XFF-encoded data produced by [`sprintz_c_compress_xff_16b`].
    pub fn sprintz_c_decompress_xff_16b(src: *const i16, dest: *mut u16) -> i64;
}

/// Returns the minimum output buffer size (bytes) needed to compress `nelems` elements.
pub fn compress_buf_size(nelems: usize) -> usize {
    nelems * 3 / 2 + 64
}

#[cfg(all(test, target_arch = "x86_64"))]
mod tests {
    use super::*;

    fn round_trip_8b(
        compress: unsafe extern "C" fn(*const u8, u32, *mut i8, u16) -> i64,
        decompress: unsafe extern "C" fn(*const i8, *mut u8) -> i64,
    ) {
        let original: Vec<u8> = (0u8..=127).collect();
        let n = original.len();
        let mut compressed = vec![0i8; compress_buf_size(n)];
        let mut decompressed = vec![0u8; n + 64];

        unsafe {
            let clen = compress(original.as_ptr(), n as u32, compressed.as_mut_ptr(), 1);
            assert!(clen > 0, "compress returned {clen}");
            let dlen = decompress(compressed.as_ptr(), decompressed.as_mut_ptr());
            assert!(dlen > 0, "decompress returned {dlen}");
        }
        assert_eq!(&decompressed[..n], original.as_slice());
    }

    fn round_trip_16b(
        compress: unsafe extern "C" fn(*const u16, u32, *mut i16, u16) -> i64,
        decompress: unsafe extern "C" fn(*const i16, *mut u16) -> i64,
    ) {
        let original: Vec<u16> = (0u16..128).collect();
        let n = original.len();
        let mut compressed = vec![0i16; compress_buf_size(n * 2) / 2 + 1];
        let mut decompressed = vec![0u16; n + 64];

        unsafe {
            let clen = compress(original.as_ptr(), n as u32, compressed.as_mut_ptr(), 1);
            assert!(clen > 0, "compress returned {clen}");
            let dlen = decompress(compressed.as_ptr(), decompressed.as_mut_ptr());
            assert!(dlen > 0, "decompress returned {dlen}");
        }
        assert_eq!(&decompressed[..n], original.as_slice());
    }

    #[test]
    fn test_delta_8b() {
        round_trip_8b(sprintz_c_compress_delta_8b, sprintz_c_decompress_delta_8b);
    }

    #[test]
    fn test_xff_8b() {
        round_trip_8b(sprintz_c_compress_xff_8b, sprintz_c_decompress_xff_8b);
    }

    #[test]
    fn test_delta_16b() {
        round_trip_16b(sprintz_c_compress_delta_16b, sprintz_c_decompress_delta_16b);
    }

    #[test]
    fn test_xff_16b() {
        round_trip_16b(sprintz_c_compress_xff_16b, sprintz_c_decompress_xff_16b);
    }
}
