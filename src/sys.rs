#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

use std::os::raw::{c_void, c_int};

use fastlz_sys::{
    fastlz_compress_level,
    fastlz_decompress
};

/// Compress a block of data in the input buffer and returns the size of
/// compressed block. The size of input buffer is specified by length. The
/// minimum input buffer size is 16.
///
/// The output buffer must be at least 5% larger than the input buffer
/// and can not be smaller than 66 bytes.
///
/// If the input is not compressible, the return value might be larger than
/// length (input buffer size).
///
/// The input buffer and the output buffer can not overlap.
pub fn compress(
    level: u8,
    input: &[u8],
    output: &mut [u8]
) -> Option<usize> {
    let size = unsafe {
        fastlz_compress_level(
            level as c_int,
            input.as_ptr() as *const c_void,
            input.len() as c_int,
            output.as_mut_ptr() as *mut c_void
        )
    } as usize;

    if size > output.len() {
        // Output buffer overflow!
        return None;
    }

    Some(size)
}

/// Decompress a block of compressed data and returns the size of the
/// decompressed block. If error occurs, e.g. the compressed data is
/// corrupted or the output buffer is not large enough, then 0 (zero)
/// will be returned instead.
///
/// The input buffer and the output buffer can not overlap.
///
/// Decompression is memory safe and guaranteed not to write the output buffer
/// more than what is specified in maxout.
pub fn decompress(
    input: &[u8],
    output: &mut [u8]
) -> Option<usize> {
    let size = unsafe {
        fastlz_decompress(
            input.as_ptr() as *const c_void,
            input.len() as c_int,
            output.as_mut_ptr() as *mut c_void,
            output.len() as c_int,
        )
    } as usize;

    Some(size)
}
