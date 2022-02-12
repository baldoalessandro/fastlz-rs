mod compress;
mod decompress;

pub use compress::compress;
pub use decompress::decompress;

#[cfg(test)]
mod tests {
    use super::decompress;

    const FIXTURE_ORIG: &[u8] = include_bytes!("../../data/sample.txt");
    const FIXTURE_COMP_LV1: &[u8] = include_bytes!("../../data/compressed-lvl1.lz");
    const FIXTURE_COMP_LV2: &[u8] = include_bytes!("../../data/compressed-lvl2.lz");

    #[test]
    fn test_fastlz1_decompress() {
        let mut output = vec![0u8; FIXTURE_ORIG.len()];
        let output_size = decompress(&FIXTURE_COMP_LV1, &mut output).unwrap_or(0);
        assert_eq!(FIXTURE_ORIG.len(), output_size);
        assert_eq!(FIXTURE_ORIG, &output[..]);
    }

    #[test]
    fn test_fastlz2_decompress() {
        let mut output = vec![0u8; FIXTURE_ORIG.len()];
        let output_size = decompress(&FIXTURE_COMP_LV2, &mut output).unwrap_or(0);
        assert_eq!(FIXTURE_ORIG.len(), output_size);
        assert_eq!(FIXTURE_ORIG, &output[..]);
    }
}
