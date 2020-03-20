#[cfg(feature = "sys")]
mod crosschecks {
    use std::os::raw::{c_void, c_int};

    use fastlz_sys as sys;
    use fastlz_rs as native;

    fn compare_buffers(a: &[u8], b: &[u8]) -> bool {
        a.iter().zip(b).find(|(x, y)| x != y).is_none()
    }

    #[test]
    fn test_fastlz_compress_level_1() {
        let input: &[u8] = include_bytes!("./data/sample.txt");
        let input_size = input.len();

        // compress the input ( require a buffer 5% bigger than the input)
        let sys_buff = vec![0u8; (input_size as f64 * 1.05) as usize];
        let sys_size = unsafe {
            sys::fastlz_compress_level(
                1 as c_int,
                input.as_ptr() as *const c_void,
                input_size as c_int,
                sys_buff.as_ptr() as *mut c_void
            )
        };

        let native_buff = vec![0u8; (input_size as f64 * 1.05) as usize];
        let native_size = native::fastlz_compress_level(1, &input, &native_buff);

        assert_eq!(sys_size, native_size);
        assert!(compare_buffers(&sys_buff, &native_buff));
    }

    #[test]
    fn test_fastlz_compress_level_2() {
        let input: &[u8] = include_bytes!("./data/sample.txt");
        let input_size = input.len();

        // compress the input ( require a buffer 5% bigger than the input)
        let sys_buff = vec![0u8; (input_size as f64 * 1.05) as usize];
        let sys_size = unsafe {
            sys::fastlz_compress_level(
                2 as c_int,
                input.as_ptr() as *const c_void,
                input_size as c_int,
                sys_buff.as_ptr() as *mut c_void
            )
        };

        let native_buff = vec![0u8; (input_size as f64 * 1.05) as usize];
        let native_size = native::fastlz_compress_level(2, &input, &native_buff);

        assert_eq!(sys_size, native_size);
        assert!(compare_buffers(&sys_buff, &native_buff));
    }

    #[test]
    fn test_fastlz_decompress_level_1() {
        let input_orig = include_bytes!("./data/sample.txt");
        let input_comp = include_bytes!("./data/compressed-lvl1.lz");
        let orig_size = input_orig.len();
        let comp_size = input_comp.len();

        let sys_buff = vec![0u8; orig_size + 100];
        let sys_size = unsafe {
            sys::fastlz_decompress(
                input_comp.as_ptr() as *const c_void,
                comp_size as c_int,
                sys_buff.as_ptr() as *mut c_void,
                orig_size as c_int,
            )
        };

        let native_buff = vec![0u8; orig_size + 100];
        let native_size = unsafe {
            native::fastlz_decompress(
                input_comp.as_ptr() as *const c_void,
                comp_size as c_int,
                native_buff.as_ptr() as *mut c_void,
                orig_size as c_int,
            )
        };

        assert_eq!(sys_size, native_size);
        assert_eq!(orig_size as i32, sys_size);
        assert!(compare_buffers(&sys_buff, &native_buff));
    }

    #[test]
    fn test_fastlz_decompress_level_2() {
        let input_orig = include_bytes!("./data/sample.txt");
        let input_comp = include_bytes!("./data/compressed-lvl2.lz");
        let orig_size = input_orig.len();
        let comp_size = input_comp.len();

        let sys_buff = vec![0u8; orig_size];
        let sys_size = unsafe {
            sys::fastlz_decompress(
                input_comp.as_ptr() as *const c_void,
                comp_size as c_int,
                sys_buff.as_ptr() as *mut c_void,
                orig_size as c_int,
            )
        };

        let native_buff = vec![0u8; orig_size];
        let native_size = unsafe {
            native::fastlz_decompress(
                input_comp.as_ptr() as *const c_void,
                comp_size as c_int,
                native_buff.as_ptr() as *mut c_void,
                orig_size as c_int,
            )
        };

        assert_eq!(sys_size, native_size);
        assert_eq!(orig_size as i32, sys_size);
        assert!(compare_buffers(&sys_buff, &native_buff));
    }
}
