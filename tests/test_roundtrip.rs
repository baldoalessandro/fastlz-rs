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

use ::libc;
use ::fastlz_rs;

use std::ffi::CString;


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
    /*
  FastLZ - Byte-aligned LZ77 compression library
  Copyright (C) 2005-2020 Ariya Hidayat <ariya.hidayat@gmail.com>

  Permission is hereby granted, free of charge, to any person obtaining a copy
  of this software and associated documentation files (the "Software"), to deal
  in the Software without restriction, including without limitation the rights
  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
  copies of the Software, and to permit persons to whom the Software is
  furnished to do so, subject to the following conditions:

  The above copyright notice and this permission notice shall be included in
  all copies or substantial portions of the Software.

  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
  THE SOFTWARE.
*/
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
/*
  FastLZ - Byte-aligned LZ77 compression library
  Copyright (C) 2005-2020 Ariya Hidayat <ariya.hidayat@gmail.com>

  Permission is hereby granted, free of charge, to any person obtaining a copy
  of this software and associated documentation files (the "Software"), to deal
  in the Software without restriction, including without limitation the rights
  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
  copies of the Software, and to permit persons to whom the Software is
  furnished to do so, subject to the following conditions:

  The above copyright notice and this permission notice shall be included in
  all copies or substantial portions of the Software.

  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
  THE SOFTWARE.
*/
#[no_mangle]
pub unsafe extern "C" fn compare(mut name: *const libc::c_char,
                                 mut a: *const uint8_t, mut b: *const uint8_t,
                                 mut size: libc::c_int) -> libc::c_int {
    let mut bad: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < size {
        if *a.offset(i as isize) as libc::c_int !=
               *b.offset(i as isize) as libc::c_int {
            bad = 1 as libc::c_int;
            printf(b"Error on %s!\n\x00" as *const u8 as *const libc::c_char,
                   name);
            printf(b"Different at index %d: expecting %02x,actual %02x\n\x00"
                       as *const u8 as *const libc::c_char, i,
                   *a.offset(i as isize) as libc::c_int,
                   *b.offset(i as isize) as libc::c_int);
            break ;
        } else { i += 1 }
    }
    return bad;
}
/*
  Same as test_roundtrip_level1 EXCEPT that the decompression is carried out
  using the highly-simplified, unoptimized vanilla reference decompressor.
*/
#[no_mangle]
pub unsafe extern "C" fn test_ref_decompressor_level1(mut name:
                                                          *const libc::c_char,
                                                      mut file_name:
                                                          *const libc::c_char) {
    let mut f: *mut FILE =
        fopen(file_name, b"rb\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        printf(b"Error: can not open %s!\n\x00" as *const u8 as
                   *const libc::c_char, file_name);
        exit(1 as libc::c_int);
    }
    fseek(f, 0 as libc::c_long, 2 as libc::c_int);
    let mut file_size: libc::c_long = ftell(f);
    rewind(f);
    let mut file_buffer: *mut uint8_t =
        malloc(file_size as libc::c_ulong) as *mut uint8_t;
    let mut read: libc::c_long =
        fread(file_buffer as *mut libc::c_void,
              1 as libc::c_int as libc::c_ulong, file_size as libc::c_ulong,
              f) as libc::c_long;
    fclose(f);
    if read != file_size {
        free(file_buffer as *mut libc::c_void);
        printf(b"Error: only read %ld bytes!\n\x00" as *const u8 as
                   *const libc::c_char, read);
        exit(1 as libc::c_int);
    }
    let mut compressed_buffer: *mut uint8_t =
        malloc((1.05f64 * file_size as libc::c_double) as libc::c_ulong) as
            *mut uint8_t;
    let mut compressed_size: libc::c_int =
        fastlz_compress_level(1 as libc::c_int,
                              file_buffer as *const libc::c_void,
                              file_size as libc::c_int,
                              compressed_buffer as *mut libc::c_void);
    let mut ratio: libc::c_double =
        100.0f64 * compressed_size as libc::c_double /
            file_size as libc::c_double;
    let mut uncompressed_buffer: *mut uint8_t =
        malloc(file_size as libc::c_ulong) as *mut uint8_t;
    memset(uncompressed_buffer as *mut libc::c_void, '-' as i32,
           file_size as libc::c_ulong);
    REF_Level1_decompress(compressed_buffer, compressed_size,
                          uncompressed_buffer);
    let mut result: libc::c_int =
        compare(file_name, file_buffer, uncompressed_buffer,
                file_size as libc::c_int);
    if result == 1 as libc::c_int {
        free(uncompressed_buffer as *mut libc::c_void);
        exit(1 as libc::c_int);
    }
    free(file_buffer as *mut libc::c_void);
    free(compressed_buffer as *mut libc::c_void);
    free(uncompressed_buffer as *mut libc::c_void);
    printf(b"%25s %10ld  -> %10d  (%.2f%%)\n\x00" as *const u8 as
               *const libc::c_char, name, file_size, compressed_size, ratio);
}
/*
  Same as test_roundtrip_level2 EXCEPT that the decompression is carried out
  using the highly-simplified, unoptimized vanilla reference decompressor.
*/
#[no_mangle]
pub unsafe extern "C" fn test_ref_decompressor_level2(mut name:
                                                          *const libc::c_char,
                                                      mut file_name:
                                                          *const libc::c_char) {
    let mut f: *mut FILE =
        fopen(file_name, b"rb\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        printf(b"Error: can not open %s!\n\x00" as *const u8 as
                   *const libc::c_char, file_name);
        exit(1 as libc::c_int);
    }
    fseek(f, 0 as libc::c_long, 2 as libc::c_int);
    let mut file_size: libc::c_long = ftell(f);
    rewind(f);
    let mut file_buffer: *mut uint8_t =
        malloc(file_size as libc::c_ulong) as *mut uint8_t;
    let mut read: libc::c_long =
        fread(file_buffer as *mut libc::c_void,
              1 as libc::c_int as libc::c_ulong, file_size as libc::c_ulong,
              f) as libc::c_long;
    fclose(f);
    if read != file_size {
        free(file_buffer as *mut libc::c_void);
        printf(b"Error: only read %ld bytes!\n\x00" as *const u8 as
                   *const libc::c_char, read);
        exit(1 as libc::c_int);
    }
    let mut compressed_buffer: *mut uint8_t =
        malloc((1.05f64 * file_size as libc::c_double) as libc::c_ulong) as
            *mut uint8_t;
    let mut compressed_size: libc::c_int =
        fastlz_compress_level(2 as libc::c_int,
                              file_buffer as *const libc::c_void,
                              file_size as libc::c_int,
                              compressed_buffer as *mut libc::c_void);
    let mut ratio: libc::c_double =
        100.0f64 * compressed_size as libc::c_double /
            file_size as libc::c_double;
    let mut uncompressed_buffer: *mut uint8_t =
        malloc(file_size as libc::c_ulong) as *mut uint8_t;
    memset(uncompressed_buffer as *mut libc::c_void, '-' as i32,
           file_size as libc::c_ulong);
    /* intentionally mask out the block tag */
    *compressed_buffer.offset(0 as libc::c_int as isize) =
        (*compressed_buffer.offset(0 as libc::c_int as isize) as libc::c_int &
             31 as libc::c_int) as uint8_t;
    REF_Level2_decompress(compressed_buffer, compressed_size,
                          uncompressed_buffer);
    let mut result: libc::c_int =
        compare(file_name, file_buffer, uncompressed_buffer,
                file_size as libc::c_int);
    if result == 1 as libc::c_int {
        free(uncompressed_buffer as *mut libc::c_void);
        exit(1 as libc::c_int);
    }
    free(file_buffer as *mut libc::c_void);
    free(compressed_buffer as *mut libc::c_void);
    free(uncompressed_buffer as *mut libc::c_void);
    printf(b"%25s %10ld  -> %10d  (%.2f%%)\n\x00" as *const u8 as
               *const libc::c_char, name, file_size, compressed_size, ratio);
}
/*
  Read the content of the file.
  Compress it first using the Level 1 compressor.
  Decompress the output with Level 1 decompressor.
  Compare the result with the original file content.
*/
#[no_mangle]
pub unsafe extern "C" fn test_roundtrip_level1(mut name: *const libc::c_char,
                                               mut file_name:
                                                   *const libc::c_char) {
    let mut f: *mut FILE =
        fopen(file_name, b"rb\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        printf(b"Error: can not open %s!\n\x00" as *const u8 as
                   *const libc::c_char, file_name);
        exit(1 as libc::c_int);
    }
    fseek(f, 0 as libc::c_long, 2 as libc::c_int);
    let mut file_size: libc::c_long = ftell(f);
    rewind(f);
    let mut file_buffer: *mut uint8_t =
        malloc(file_size as libc::c_ulong) as *mut uint8_t;
    let mut read: libc::c_long =
        fread(file_buffer as *mut libc::c_void,
              1 as libc::c_int as libc::c_ulong, file_size as libc::c_ulong,
              f) as libc::c_long;
    fclose(f);
    if read != file_size {
        free(file_buffer as *mut libc::c_void);
        printf(b"Error: only read %ld bytes!\n\x00" as *const u8 as
                   *const libc::c_char, read);
        exit(1 as libc::c_int);
    }
    let mut compressed_buffer: *mut uint8_t =
        malloc((1.05f64 * file_size as libc::c_double) as libc::c_ulong) as
            *mut uint8_t;
    let mut compressed_size: libc::c_int =
        fastlz_compress_level(1 as libc::c_int,
                              file_buffer as *const libc::c_void,
                              file_size as libc::c_int,
                              compressed_buffer as *mut libc::c_void);
    let mut ratio: libc::c_double =
        100.0f64 * compressed_size as libc::c_double /
            file_size as libc::c_double;
    let mut uncompressed_buffer: *mut uint8_t =
        malloc(file_size as libc::c_ulong) as *mut uint8_t;
    memset(uncompressed_buffer as *mut libc::c_void, '-' as i32,
           file_size as libc::c_ulong);
    fastlz_decompress(compressed_buffer as *const libc::c_void,
                      compressed_size,
                      uncompressed_buffer as *mut libc::c_void,
                      file_size as libc::c_int);
    let mut result: libc::c_int =
        compare(file_name, file_buffer, uncompressed_buffer,
                file_size as libc::c_int);
    if result == 1 as libc::c_int {
        free(uncompressed_buffer as *mut libc::c_void);
        exit(1 as libc::c_int);
    }
    free(file_buffer as *mut libc::c_void);
    free(compressed_buffer as *mut libc::c_void);
    free(uncompressed_buffer as *mut libc::c_void);
    printf(b"%25s %10ld  -> %10d  (%.2f%%)\n\x00" as *const u8 as
               *const libc::c_char, name, file_size, compressed_size, ratio);
}
/*
  Read the content of the file.
  Compress it first using the Level 2 compressor.
  Decompress the output with Level 2 decompressor.
  Compare the result with the original file content.
*/
#[no_mangle]
pub unsafe extern "C" fn test_roundtrip_level2(mut name: *const libc::c_char,
                                               mut file_name:
                                                   *const libc::c_char) {
    let mut f: *mut FILE =
        fopen(file_name, b"rb\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        printf(b"Error: can not open %s!\n\x00" as *const u8 as
                   *const libc::c_char, file_name);
        exit(1 as libc::c_int);
    }
    fseek(f, 0 as libc::c_long, 2 as libc::c_int);
    let mut file_size: libc::c_long = ftell(f);
    rewind(f);
    let mut file_buffer: *mut uint8_t =
        malloc(file_size as libc::c_ulong) as *mut uint8_t;
    let mut read: libc::c_long =
        fread(file_buffer as *mut libc::c_void,
              1 as libc::c_int as libc::c_ulong, file_size as libc::c_ulong,
              f) as libc::c_long;
    fclose(f);
    if read != file_size {
        free(file_buffer as *mut libc::c_void);
        printf(b"Error: only read %ld bytes!\n\x00" as *const u8 as
                   *const libc::c_char, read);
        exit(1 as libc::c_int);
    }
    let mut compressed_buffer: *mut uint8_t =
        malloc((1.05f64 * file_size as libc::c_double) as libc::c_ulong) as
            *mut uint8_t;
    let mut compressed_size: libc::c_int =
        fastlz_compress_level(2 as libc::c_int,
                              file_buffer as *const libc::c_void,
                              file_size as libc::c_int,
                              compressed_buffer as *mut libc::c_void);
    let mut ratio: libc::c_double =
        100.0f64 * compressed_size as libc::c_double /
            file_size as libc::c_double;
    let mut uncompressed_buffer: *mut uint8_t =
        malloc(file_size as libc::c_ulong) as *mut uint8_t;
    memset(uncompressed_buffer as *mut libc::c_void, '-' as i32,
           file_size as libc::c_ulong);
    fastlz_decompress(compressed_buffer as *const libc::c_void,
                      compressed_size,
                      uncompressed_buffer as *mut libc::c_void,
                      file_size as libc::c_int);
    let mut result: libc::c_int =
        compare(file_name, file_buffer, uncompressed_buffer,
                file_size as libc::c_int);
    if result == 1 as libc::c_int {
        free(uncompressed_buffer as *mut libc::c_void);
        exit(1 as libc::c_int);
    }
    free(file_buffer as *mut libc::c_void);
    free(compressed_buffer as *mut libc::c_void);
    free(uncompressed_buffer as *mut libc::c_void);
    printf(b"%25s %10ld  -> %10d  (%.2f%%)\n\x00" as *const u8 as
               *const libc::c_char, name, file_size, compressed_size, ratio);
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
        println!("{}", corpus);
        let f = format!("{}{}", corpora_dir, corpus);

        let name: CString = CString::new(*corpus).unwrap();
        let filename: CString = CString::new(f).unwrap();

        unsafe {
            test_ref_decompressor_level1(name.as_ptr(), filename.as_ptr());
        }
    });
    println!();
}

#[test]
fn test_ref_impl_level2() {
    println!("Test reference decompressor for Level 2");
    corpora.iter().for_each(|corpus| {
        println!("{}", corpus);
        let f = format!("{}{}", corpora_dir, corpus);

        let name: CString = CString::new(*corpus).unwrap();
        let filename: CString = CString::new(f).unwrap();

        unsafe {
            test_ref_decompressor_level2(name.as_ptr(), filename.as_ptr());
        }
    });
    println!();
}

#[test]
fn test_round_trip_level1() {
    println!("Test round-trip for Level 1");
    corpora.iter().for_each(|corpus| {
        println!("{}", corpus);
        let f = format!("{}{}", corpora_dir, corpus);

        let name: CString = CString::new(*corpus).unwrap();
        let filename: CString = CString::new(f).unwrap();

        unsafe {
            test_roundtrip_level1(name.as_ptr(), filename.as_ptr());
        }
    });
    println!();
}

#[test]
fn test_round_trip_level2() {
    println!("Test round-trip for Level 2");
    corpora.iter().for_each(|corpus| {
        println!("{}", corpus);
        let f = format!("{}{}", corpora_dir, corpus);

        let name: CString = CString::new(*corpus).unwrap();
        let filename: CString = CString::new(f).unwrap();

        unsafe {
            test_roundtrip_level2(name.as_ptr(), filename.as_ptr());
        }
    });
    println!();
}
