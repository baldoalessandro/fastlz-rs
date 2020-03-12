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


unsafe fn main_0()
 -> libc::c_int {
    let mut prefix: *const libc::c_char =
        b"../compression-corpus/\x00" as *const u8 as *const libc::c_char;
    let mut names: [*const libc::c_char; 24] =
        [b"canterbury/alice29.txt\x00" as *const u8 as *const libc::c_char,
         b"canterbury/asyoulik.txt\x00" as *const u8 as *const libc::c_char,
         b"canterbury/cp.html\x00" as *const u8 as *const libc::c_char,
         b"canterbury/fields.c\x00" as *const u8 as *const libc::c_char,
         b"canterbury/grammar.lsp\x00" as *const u8 as *const libc::c_char,
         b"canterbury/kennedy.xls\x00" as *const u8 as *const libc::c_char,
         b"canterbury/lcet10.txt\x00" as *const u8 as *const libc::c_char,
         b"canterbury/plrabn12.txt\x00" as *const u8 as *const libc::c_char,
         b"canterbury/ptt5\x00" as *const u8 as *const libc::c_char,
         b"canterbury/sum\x00" as *const u8 as *const libc::c_char,
         b"canterbury/xargs.1\x00" as *const u8 as *const libc::c_char,
         b"silesia/dickens\x00" as *const u8 as *const libc::c_char,
         b"silesia/mozilla\x00" as *const u8 as *const libc::c_char,
         b"silesia/mr\x00" as *const u8 as *const libc::c_char,
         b"silesia/nci\x00" as *const u8 as *const libc::c_char,
         b"silesia/ooffice\x00" as *const u8 as *const libc::c_char,
         b"silesia/osdb\x00" as *const u8 as *const libc::c_char,
         b"silesia/reymont\x00" as *const u8 as *const libc::c_char,
         b"silesia/samba\x00" as *const u8 as *const libc::c_char,
         b"silesia/sao\x00" as *const u8 as *const libc::c_char,
         b"silesia/webster\x00" as *const u8 as *const libc::c_char,
         b"silesia/x-ray\x00" as *const u8 as *const libc::c_char,
         b"silesia/xml\x00" as *const u8 as *const libc::c_char,
         b"enwik/enwik8.txt\x00" as *const u8 as *const libc::c_char];
    let count: libc::c_int =
        (::std::mem::size_of::<[*const libc::c_char; 24]>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                             as libc::c_ulong) as libc::c_int;
    let mut i: libc::c_int = 0;
    printf(b"Test reference decompressor for Level 1\n\n\x00" as *const u8 as
               *const libc::c_char);
    i = 0 as libc::c_int;
    while i < count {
        let mut name: *const libc::c_char = names[i as usize];
        let mut filename: *mut libc::c_char =
            malloc(strlen(prefix).wrapping_add(strlen(name)).wrapping_add(1 as
                                                                              libc::c_int
                                                                              as
                                                                              libc::c_ulong))
                as *mut libc::c_char;
        strcpy(filename, prefix);
        strcat(filename, name);
        test_ref_decompressor_level1(name, filename);
        free(filename as *mut libc::c_void);
        i += 1
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    printf(b"Test reference decompressor for Level 2\n\n\x00" as *const u8 as
               *const libc::c_char);
    i = 0 as libc::c_int;
    while i < count {
        let mut name_0: *const libc::c_char = names[i as usize];
        let mut filename_0: *mut libc::c_char =
            malloc(strlen(prefix).wrapping_add(strlen(name_0)).wrapping_add(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong))
                as *mut libc::c_char;
        strcpy(filename_0, prefix);
        strcat(filename_0, name_0);
        test_ref_decompressor_level2(name_0, filename_0);
        free(filename_0 as *mut libc::c_void);
        i += 1
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    printf(b"Test round-trip for Level 1\n\n\x00" as *const u8 as
               *const libc::c_char);
    i = 0 as libc::c_int;
    while i < count {
        let mut name_1: *const libc::c_char = names[i as usize];
        let mut filename_1: *mut libc::c_char =
            malloc(strlen(prefix).wrapping_add(strlen(name_1)).wrapping_add(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong))
                as *mut libc::c_char;
        strcpy(filename_1, prefix);
        strcat(filename_1, name_1);
        test_roundtrip_level1(name_1, filename_1);
        free(filename_1 as *mut libc::c_void);
        i += 1
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    printf(b"Test round-trip for Level 2\n\n\x00" as *const u8 as
               *const libc::c_char);
    i = 0 as libc::c_int;
    while i < count {
        let mut name_2: *const libc::c_char = names[i as usize];
        let mut filename_2: *mut libc::c_char =
            malloc(strlen(prefix).wrapping_add(strlen(name_2)).wrapping_add(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong))
                as *mut libc::c_char;
        strcpy(filename_2, prefix);
        strcat(filename_2, name_2);
        test_roundtrip_level2(name_2, filename_2);
        free(filename_2 as *mut libc::c_void);
        i += 1
    }
    printf(b"\n\x00" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}


#[test]
pub fn main() {
    unsafe {
        ::std::process::exit(main_0() as i32)
    }
}
