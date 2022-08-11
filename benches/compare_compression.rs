use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};

use std::fs::File;
use std::io::prelude::*;

use fastlz_rs::{
    sys::compress as sys_compress,
    native::compress as native_compress
};

const CORPORA_DIR: &'static str = "../data/compression-corpus/";
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
        let mut comp_buf_sys: Vec<u8> = vec![0u8; comp_buf_size];
        let mut comp_buf_native: Vec<u8> = vec![0u8; comp_buf_size];

        group.bench_with_input(BenchmarkId::new("C via FFI", &corpus), &corpus, 
            |b, _corpus| b.iter(|| {
                unsafe {
                    sys_compress(
                        1,
                        &file_buf,
                        &mut comp_buf_native
                    )
                }
            }));

        group.bench_with_input(BenchmarkId::new("RUST", &corpus), &corpus, 
            |b, _corpus| b.iter(|| {
                native_compress(
                    1,
                    &file_buf,
                    &mut comp_buf_native
                )
            }));
    }

    group.finish();
}

criterion_group!(benches, bench_compression_against_c_impl);
criterion_main!(benches);
