#[cfg(feature = "sys")]
mod crosschecks {
    use std::os::raw::{c_void, c_int};

    use fastlz_rs::{sys, native};

    fn compare_buffers(a: &[u8], b: &[u8]) -> bool {
        a.iter().zip(b).find(|(x, y)| x != y).is_none()
    }

    #[test]
    fn test_fastlz_compress_level_1() {
        let input: &[u8] = include_bytes!("./data/sample.txt");
        let input_size = input.len();

        // compress the input (require a buffer 5% bigger than the input)
        let mut sys_buff = vec![0u8; (input_size as f64 * 1.05) as usize];
        let sys_size = sys::compress(1, &input, &mut sys_buff).unwrap_or(0);

        let mut native_buff = vec![0u8; (input_size as f64 * 1.05) as usize];
        let native_size = native::compress(1, &input, &mut native_buff).unwrap_or(0);

        assert_eq!(sys_size, native_size);
        assert!(compare_buffers(&sys_buff, &native_buff));
    }

    #[test]
    fn test_fastlz_compress_level_2() {
        let input: &[u8] = include_bytes!("./data/sample.txt");
        let input_size = input.len();

        // compress the input (require a buffer 5% bigger than the input)
        let mut sys_buff = vec![0u8; (input_size as f64 * 1.05) as usize];
        let sys_size = sys::compress(2, &input, &mut sys_buff).unwrap_or(0);

        let mut native_buff = vec![0u8; (input_size as f64 * 1.05) as usize];
        let native_size = native::compress(2, &input, &mut native_buff).unwrap_or(0);

        assert_eq!(sys_size, native_size);
        assert!(compare_buffers(&sys_buff, &native_buff));
    }

    #[test]
    fn test_fastlz_decompress_level_1() {
        let input_orig: &[u8] = include_bytes!("./data/sample.txt");
        let input_comp: &[u8] = include_bytes!("./data/compressed-lvl1.lz");
        let orig_size = input_orig.len();
        let comp_size = input_comp.len();

        let mut sys_buff = vec![0u8; orig_size + 100];
        let sys_size = sys::decompress(&input_comp, &sys_buff).unwrap_or(0);

        let mut native_buff = vec![0u8; orig_size + 100];
        let native_size = native::decompress(&input_comp, &native_buff).unwrap_or(0);

        assert_eq!(sys_size, native_size);
        assert_eq!(orig_size, sys_size);
        assert!(compare_buffers(&sys_buff, &native_buff));
    }

    #[test]
    fn test_fastlz_decompress_level_2() {
        let input_orig: &[u8] = include_bytes!("./data/sample.txt");
        let input_comp: &[u8] = include_bytes!("./data/compressed-lvl2.lz");
        let orig_size = input_orig.len();
        let comp_size = input_comp.len();

        let mut sys_buff = vec![0u8; orig_size];
        let sys_size = sys::decompress(&input_comp, &sys_buff).unwrap_or(0);

        let mut native_buff = vec![0u8; orig_size];
        let native_size = native::decompress(&input_comp, &native_buff).unwrap_or(0);

        assert_eq!(sys_size, native_size);
        assert_eq!(orig_size, sys_size);
        assert!(compare_buffers(&sys_buff, &native_buff));
    }
}
