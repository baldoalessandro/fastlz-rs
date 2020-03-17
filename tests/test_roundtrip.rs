#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]
#![feature(main)]
#![feature(ptr_wrapping_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]

use std::fs::File;
use std::io::prelude::*;
use std::convert::TryInto;

use ::libc;

use ::fastlz_rs;


mod refimpl;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn fread(_: *mut libc::c_void, _: libc::c_ulong, _: libc::c_ulong,
             _: *mut FILE) -> libc::c_ulong;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    #[no_mangle]
    fn rewind(__stream: *mut FILE);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
     -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;

    /* *
  Compress a block of data in the input buffer and returns the size of
  compressed block. The size of input buffer is specified by length. The
  minimum input buffer size is 16.

  The output buffer must be at least 5% larger than the input buffer
  and can not be smaller than 66 bytes.

  If the input is not compressible, the return value might be larger than
  length (input buffer size).

  The input buffer and the output buffer can not overlap.

  Compression level can be specified in parameter level. At the moment,
  only level 1 and level 2 are supported.
  Level 1 is the fastest compression and generally useful for short data.
  Level 2 is slightly slower but it gives better compression ratio.

  Note that the compressed data, regardless of the level, can always be
  decompressed using the function fastlz_decompress below.
*/
    #[no_mangle]
    fn fastlz_compress_level(level: libc::c_int, input: *const libc::c_void,
                             length: libc::c_int, output: *mut libc::c_void)
     -> libc::c_int;
    /* *
  Decompress a block of compressed data and returns the size of the
  decompressed block. If error occurs, e.g. the compressed data is
  corrupted or the output buffer is not large enough, then 0 (zero)
  will be returned instead.

  The input buffer and the output buffer can not overlap.

  Decompression is memory safe and guaranteed not to write the output buffer
  more than what is specified in maxout.

  Note that the decompression will always work, regardless of the
  compression level specified in fastlz_compress_level above (when
  producing the compressed block).
 */
    #[no_mangle]
    fn fastlz_decompress(input: *const libc::c_void, length: libc::c_int,
                         output: *mut libc::c_void, maxout: libc::c_int)
     -> libc::c_int;
    /* prototype, implemented in refimpl.c */
    #[no_mangle]
    fn REF_Level1_decompress(input: *const uint8_t, length: libc::c_int,
                             output: *mut uint8_t);
    #[no_mangle]
    fn REF_Level2_decompress(input: *const uint8_t, length: libc::c_int,
                             output: *mut uint8_t);
}
pub type __uint8_t = libc::c_uchar;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint8_t = __uint8_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;

#[no_mangle]
fn compare(name: &str, a: &Vec<u8>,  b: &Vec<u8>) -> bool {
    let a_iter = a.iter();
    let b_iter = b.iter();
    let res = a_iter.zip(b_iter)
        .enumerate()
        .find(|(_idx, (ea, eb))| {
            ea != eb
        });

    if let Some(invalid_element) = res {
        let (idx,( ea, eb)) = invalid_element;
        println!("Error on : {}", name);
        println!("Different at index {}: expecting {:02x}, actual {:02x}", idx, ea, eb);
        false
    } else {
        true
    }
}
/*
  Same as test_roundtrip_level1 EXCEPT that the decompression is carried out
  using the highly-simplified, unoptimized vanilla reference decompressor.
*/
#[no_mangle]
unsafe fn test_ref_decompressor_level1(name: &str, file_name: &str) {
    let mut f = File::open(file_name)
                .expect(&format!("Error: can not open {}", file_name));
    let file_size: usize = f.metadata().unwrap().len().try_into().unwrap();

    let mut file_buffer: Vec<u8> = Vec::new();
    let read = f.read_to_end(&mut file_buffer)
                    .expect("Error: Cannot read all file into memory");
    assert_eq!(read, file_size, "Error: cannot read all bytes!");

    let compressed_buffer_size = (1.05 * file_size as f64) as usize;
    let mut compressed_buffer: Vec<u8> = vec![0u8; compressed_buffer_size];
    let compressed_size: libc::c_int = fastlz_compress_level(1 as libc::c_int,
                            file_buffer.as_ptr() as *const libc::c_void,
                            file_size as libc::c_int,
                            compressed_buffer.as_ptr() as *mut libc::c_void);

    let ratio = 100.0 * compressed_size as f64 / file_size as f64;

    let mut uncompressed_buffer: Vec<u8>  = vec!['-' as u8; file_size];
    REF_Level1_decompress(
        compressed_buffer.as_ptr() as *const uint8_t,
        compressed_size as libc::c_int,
        uncompressed_buffer.as_ptr() as *mut uint8_t
    );

    assert!(compare(
        file_name,
        &file_buffer,
        &uncompressed_buffer
    ));
    println!("{:25} {:10} -> {:10} ({:.2}%)", name, file_size, compressed_size, ratio);
}

/*
  Same as test_roundtrip_level2 EXCEPT that the decompression is carried out
  using the highly-simplified, unoptimized vanilla reference decompressor.
*/
#[no_mangle]
unsafe fn test_ref_decompressor_level2(name: &str, file_name: &str) {
    let mut f = File::open(file_name)
                .expect(&format!("Error: can not open {}", file_name));
    let file_size: usize = f.metadata().unwrap().len().try_into().unwrap();

    let mut file_buffer: Vec<u8> = Vec::new();
    let read = f.read_to_end(&mut file_buffer)
                    .expect("Error: Cannot read all file into memory");
    assert_eq!(read, file_size, "Error: cannot read all bytes!");

    let compressed_buffer_size = (1.05 * file_size as f64) as usize;
    let mut compressed_buffer: Vec<u8> = vec![0u8; compressed_buffer_size];

    let compressed_size: libc::c_int = fastlz_compress_level(2 as libc::c_int,
                                        file_buffer.as_ptr() as *const libc::c_void,
                                        file_size as libc::c_int,
                                        compressed_buffer.as_ptr() as *mut libc::c_void);
    let ratio = 100.0 * compressed_size as f64 / file_size as f64;

    let mut uncompressed_buffer: Vec<u8>  = vec!['-' as u8; file_size];
    /* intentionally mask out the block tag */
    compressed_buffer[0] = compressed_buffer[0] & 31u8;

    REF_Level2_decompress(
        compressed_buffer.as_ptr() as *const uint8_t,
        compressed_size as libc::c_int,
        uncompressed_buffer.as_ptr() as *mut uint8_t
    );

    assert!(compare(
        file_name,
        &file_buffer,
        &uncompressed_buffer
    ));
    println!("{:25} {:10} -> {:10} ({:.2}%)", name, file_size, compressed_size, ratio);
}
/*
  Read the content of the file.
  Compress it first using the Level 1 compressor.
  Decompress the output with Level 1 decompressor.
  Compare the result with the original file content.
*/
#[no_mangle]
unsafe fn test_roundtrip_level1(name: &str, file_name: &str) {
    let mut f = File::open(file_name)
                        .expect(&format!("Error: can not open {}", file_name));
    let file_size: usize = f.metadata().unwrap().len().try_into().unwrap();

    let mut file_buffer: Vec<u8> = Vec::new();
    let read = f.read_to_end(&mut file_buffer)
                    .expect("Error: Cannot read all file into memory");
    assert_eq!(read, file_size, "Error: cannot read all bytes!");

    let compressed_buffer_size = (1.05 * file_size as f64) as usize;
    let mut compressed_buffer: Vec<u8> = vec![0u8; compressed_buffer_size];
    let  compressed_size: libc::c_int = fastlz_compress_level(1 as libc::c_int,
                              file_buffer.as_ptr() as *const libc::c_void,
                              file_size as libc::c_int,
                              compressed_buffer.as_ptr() as *mut libc::c_void);
    let ratio = 100.0 * compressed_size as f64 / file_size as f64;

    let mut uncompressed_buffer: Vec<u8>  = vec!['-' as u8; file_size];
    fastlz_decompress(compressed_buffer.as_ptr() as *const libc::c_void,
                      compressed_size as libc::c_int,
                      uncompressed_buffer.as_ptr() as *mut libc::c_void,
                      file_size as libc::c_int);

    assert!(compare(
        file_name,
        &file_buffer,
        &uncompressed_buffer
    ));
    println!("{:25} {:10} -> {:10} ({:.2}%)", name, file_size, compressed_size, ratio);
}
/*
  Read the content of the file.
  Compress it first using the Level 2 compressor.
  Decompress the output with Level 2 decompressor.
  Compare the result with the original file content.
*/
#[no_mangle]
unsafe fn test_roundtrip_level2(name: &str, file_name: &str) {
    let mut f = File::open(file_name)
                        .expect(&format!("Error: can not open {}", file_name));
    let file_size: usize = f.metadata().unwrap().len().try_into().unwrap();

    let mut file_buffer: Vec<u8> = Vec::new();
    let read = f.read_to_end(&mut file_buffer)
                    .expect("Error: Cannot read all file into memory");
    assert_eq!(read, file_size, "Error: cannot read all bytes!");

    let compressed_buffer_size = (1.05 * file_size as f64) as usize;
    let mut compressed_buffer: Vec<u8> = vec![0u8; compressed_buffer_size];
    let compressed_size: libc::c_int = fastlz_compress_level(2 as libc::c_int,
                              file_buffer.as_ptr() as *const libc::c_void,
                              file_size as libc::c_int,
                              compressed_buffer.as_ptr() as *mut libc::c_void);
    let ratio = 100.0 * compressed_size as f64 / file_size as f64;

    let mut uncompressed_buffer: Vec<u8>  = vec!['-' as u8; file_size];
    fastlz_decompress(compressed_buffer.as_ptr() as *const libc::c_void,
                      compressed_size as libc::c_int,
                      uncompressed_buffer.as_ptr() as *mut libc::c_void,
                      file_size as libc::c_int);

    assert!(compare(
        file_name,
        &file_buffer,
        &uncompressed_buffer
    ));
    println!("{:25} {:10} -> {:10} ({:.2}%)", name, file_size, compressed_size, ratio);
}



const corpora_dir: &'static str = "../compression-corpus/";
const corpora: &'static[&'static str] = &[
    "canterbury/alice29.txt",
    "canterbury/asyoulik.txt",
    "canterbury/cp.html",
    "canterbury/fields.c",
    "canterbury/grammar.lsp",
    "canterbury/kennedy.xls",
    "canterbury/lcet10.txt",
    "canterbury/plrabn12.txt",
    "canterbury/ptt5",
    "canterbury/sum",
    "canterbury/xargs.1",
    "silesia/dickens",
    "silesia/mozilla",
    "silesia/mr",
    "silesia/nci",
    "silesia/ooffice",
    "silesia/osdb",
    "silesia/reymont",
    "silesia/samba",
    "silesia/sao",
    "silesia/webster",
    "silesia/x-ray",
    "silesia/xml",
    "enwik/enwik8.txt"
];


#[test]
fn test_ref_impl_level1() {
    println!("Test reference decompressor for Level 1");
    corpora.iter().for_each(|corpus| {
        let f = format!("{}{}", corpora_dir, corpus);
        unsafe {
            test_ref_decompressor_level1(*corpus, &f);
        }
    });
    println!();
}

#[test]
fn test_ref_impl_level2() {
    println!("Test reference decompressor for Level 2");
    corpora.iter().for_each(|corpus| {
        let f = format!("{}{}", corpora_dir, corpus);
        unsafe {
            test_ref_decompressor_level2(*corpus, &f);
        }
    });
    println!();
}

#[test]
fn test_round_trip_level1() {
    println!("Test round-trip for Level 1");
    corpora.iter().for_each(|corpus| {
        let f = format!("{}{}", corpora_dir, corpus);
        unsafe {
            test_roundtrip_level1(*corpus, &f);
        }
    });
    println!();
}

#[test]
fn test_round_trip_level2() {
    println!("Test round-trip for Level 2");
    corpora.iter().for_each(|corpus| {
        let f = format!("{}{}", corpora_dir, corpus);
        unsafe {
            test_roundtrip_level2(*corpus, &f);
        }
    });
    println!();
}
