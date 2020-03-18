//! Raw FastLZ FFI bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


#[cfg(test)]
mod tests {
    use std::os::raw::{c_int, c_void};
    use super::*;

    #[test]
    fn sanity_test() {
        let input = include_str!("../sample.txt").as_bytes();
        let input_size = input.len();

        // compress the input ( require a buffer 5% bigger than the input)
        let compress_buff = vec![0u8; (input_size as f64 * 1.05) as usize];
        let compress_size = unsafe {
            fastlz_compress_level(
                1 as c_int,
                input.as_ptr() as *const c_void,
                input_size as c_int,
                compress_buff.as_ptr() as *mut c_void
            )
        };

        // now decompress the output back to the input
        let decompress_buff = vec![0u8; input_size];
        let decompress_size = unsafe {
            fastlz_decompress(
                compress_buff.as_ptr() as *const c_void,
                compress_size,
                decompress_buff.as_ptr() as *mut c_void,
                input_size as c_int,
            )
        };

        // they should match
        assert_eq!(decompress_size, input_size as i32);
        assert!(decompress_buff.iter().zip(input).find(|(a, b)| a != b).is_none());
    }
}
