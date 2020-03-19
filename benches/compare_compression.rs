use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

use std::fs::File;
use std::io::prelude::*;
use std::os::raw::{c_void, c_int};

use fastlz_sys::fastlz_compress_level as sys_compress;
use fastlz_rs::fastlz_compress_level as native_compress;


const CORPORA_DIR: &'static str = "../compression-corpus/";
const CORPORA: &'static str = include_str!("../data/corpora-list.txt");



fn bench_compression_against_c_impl(c: &mut Criterion) {
    let mut group = c.benchmark_group("Benchmark FastLZ compression");

    for corpus in CORPORA.lines().filter(|l| !l.is_empty()) {
        let file_name = format!("{}{}", CORPORA_DIR, &corpus);
        let mut f = File::open(&file_name)
            .expect(&format!("Error: can not open {}", &file_name));

        let mut file_buf: Vec<u8> = Vec::new();
        f.read_to_end(&mut file_buf)
            .expect("Error: Cannot read all file into memory");
        let file_size = file_buf.len();

        let comp_buf_size = (1.05 * file_size as f64) as usize;
        let comp_buf_sys: Vec<u8> = vec![0u8; comp_buf_size];
        let comp_buf_native: Vec<u8> = vec![0u8; comp_buf_size];

        group.bench_with_input(BenchmarkId::new("C via FFI", &corpus), &corpus, 
            |b, _corpus| b.iter(|| {
                unsafe {
                    sys_compress(
                        1 as c_int,
                        file_buf.as_ptr() as *const c_void,
                        file_size as c_int,
                        comp_buf_sys.as_ptr() as *mut c_void
                    )
                }
            }));
        group.bench_with_input(BenchmarkId::new("RUST", &corpus), &corpus, 
            |b, _corpus| b.iter(|| {
                unsafe {
                    native_compress(
                        1 as c_int,
                        file_buf.as_ptr() as *const c_void,
                        file_size as c_int,
                        comp_buf_native.as_ptr() as *mut c_void
                    )
                }
            }));
    }

    group.finish();
}

criterion_group!(benches, bench_compression_against_c_impl);
criterion_main!(benches);
