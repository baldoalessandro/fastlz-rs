use ::libc;
extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
}
pub type uint8_t = __uint8_t;
pub type __uint8_t = libc::c_uchar;
pub type uint32_t = __uint32_t;
pub type __uint32_t = libc::c_uint;
pub type uint64_t = __uint64_t;
pub type __uint64_t = libc::c_ulong;
pub type uint16_t = __uint16_t;
pub type __uint16_t = libc::c_ushort;
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
/*
 * Always check for bound when decompressing.
 * Generally it is best to leave it defined.
 */
/*
 * Give hints to the compiler for branch prediction optimization.
 */
/*
 * Specialize custom 64-bit implementation for speed improvements.
 */
unsafe extern "C" fn fastlz_memmove(mut dest: *mut uint8_t,
                                    mut src: *const uint8_t,
                                    mut count: uint32_t) {
    if count > 4 as libc::c_int as libc::c_uint &&
           dest >= src.offset(count as isize) as *mut uint8_t {
        memmove(dest as *mut libc::c_void, src as *const libc::c_void,
                count as libc::c_ulong);
    } else {
        let mut current_block_7: u64;
        match count {
            3 => {
                let fresh2 = src;
                src = src.offset(1);
                let fresh3 = dest;
                dest = dest.offset(1);
                *fresh3 = *fresh2;
                current_block_7 = 10826308906768316228;
            }
            2 => { current_block_7 = 10826308906768316228; }
            1 => { current_block_7 = 14043750191174823896; }
            0 => { current_block_7 = 2968425633554183086; }
            _ => {
                loop  {
                    let fresh0 = src;
                    src = src.offset(1);
                    let fresh1 = dest;
                    dest = dest.offset(1);
                    *fresh1 = *fresh0;
                    count = count.wrapping_sub(1);
                    if !(count != 0) { break ; }
                }
                current_block_7 = 2968425633554183086;
            }
        }
        match current_block_7 {
            10826308906768316228 => {
                let fresh4 = src;
                src = src.offset(1);
                let fresh5 = dest;
                dest = dest.offset(1);
                *fresh5 = *fresh4;
                current_block_7 = 14043750191174823896;
            }
            _ => { }
        }
        match current_block_7 {
            14043750191174823896 => {
                let fresh6 = src;
                src = src.offset(1);
                let fresh7 = dest;
                dest = dest.offset(1);
                *fresh7 = *fresh6
            }
            _ => { }
        }
    };
}
unsafe extern "C" fn fastlz_memcpy(mut dest: *mut uint8_t,
                                   mut src: *const uint8_t,
                                   mut count: uint32_t) {
    memcpy(dest as *mut libc::c_void, src as *const libc::c_void,
           count as libc::c_ulong);
}
unsafe extern "C" fn flz_readu32(mut ptr: *const libc::c_void) -> uint32_t {
    return *(ptr as *const uint32_t);
}
unsafe extern "C" fn flz_readu64(mut ptr: *const libc::c_void) -> uint64_t {
    return *(ptr as *const uint64_t);
}
unsafe extern "C" fn flz_cmp(mut p: *const uint8_t, mut q: *const uint8_t,
                             mut r: *const uint8_t) -> uint32_t {
    let mut start: *const uint8_t = p;
    if flz_readu64(p as *const libc::c_void) ==
           flz_readu64(q as *const libc::c_void) {
        p = p.offset(8 as libc::c_int as isize);
        q = q.offset(8 as libc::c_int as isize)
    }
    if flz_readu32(p as *const libc::c_void) ==
           flz_readu32(q as *const libc::c_void) {
        p = p.offset(4 as libc::c_int as isize);
        q = q.offset(4 as libc::c_int as isize)
    }
    while q < r {
        let fresh8 = p;
        p = p.offset(1);
        let fresh9 = q;
        q = q.offset(1);
        if *fresh8 as libc::c_int != *fresh9 as libc::c_int { break ; }
    }
    return p.wrapping_offset_from(start) as libc::c_long as uint32_t;
}
unsafe extern "C" fn flz_copy64(mut dest: *mut uint8_t,
                                mut src: *const uint8_t,
                                mut count: uint32_t) {
    let mut p: *const uint64_t = src as *const uint64_t;
    let mut q: *mut uint64_t = dest as *mut uint64_t;
    if count < 16 as libc::c_int as libc::c_uint {
        if count >= 8 as libc::c_int as libc::c_uint {
            let fresh10 = p;
            p = p.offset(1);
            let fresh11 = q;
            q = q.offset(1);
            *fresh11 = *fresh10
        }
        let fresh12 = p;
        p = p.offset(1);
        let fresh13 = q;
        q = q.offset(1);
        *fresh13 = *fresh12
    } else {
        let fresh14 = p;
        p = p.offset(1);
        let fresh15 = q;
        q = q.offset(1);
        *fresh15 = *fresh14;
        let fresh16 = p;
        p = p.offset(1);
        let fresh17 = q;
        q = q.offset(1);
        *fresh17 = *fresh16;
        let fresh18 = p;
        p = p.offset(1);
        let fresh19 = q;
        q = q.offset(1);
        *fresh19 = *fresh18;
        let fresh20 = p;
        p = p.offset(1);
        let fresh21 = q;
        q = q.offset(1);
        *fresh21 = *fresh20
    };
}
unsafe extern "C" fn flz_copy256(mut dest: *mut libc::c_void,
                                 mut src: *const libc::c_void) {
    let mut p: *const uint64_t = src as *const uint64_t;
    let mut q: *mut uint64_t = dest as *mut uint64_t;
    let fresh22 = p;
    p = p.offset(1);
    let fresh23 = q;
    q = q.offset(1);
    *fresh23 = *fresh22;
    let fresh24 = p;
    p = p.offset(1);
    let fresh25 = q;
    q = q.offset(1);
    *fresh25 = *fresh24;
    let fresh26 = p;
    p = p.offset(1);
    let fresh27 = q;
    q = q.offset(1);
    *fresh27 = *fresh26;
    let fresh28 = p;
    p = p.offset(1);
    let fresh29 = q;
    q = q.offset(1);
    *fresh29 = *fresh28;
}
unsafe extern "C" fn flz_hash(mut v: uint32_t) -> uint16_t {
    let mut h: uint32_t =
        (v as libc::c_longlong * 2654435769 as libc::c_longlong >>
             32 as libc::c_int - 14 as libc::c_int) as uint32_t;
    return (h &
                (((1 as libc::c_int) << 14 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint) as uint16_t;
}
unsafe extern "C" fn flz_literals(mut runs: uint32_t, mut src: *const uint8_t,
                                  mut dest: *mut uint8_t) -> *mut uint8_t {
    while runs >= 32 as libc::c_int as libc::c_uint {
        let fresh30 = dest;
        dest = dest.offset(1);
        *fresh30 = (32 as libc::c_int - 1 as libc::c_int) as uint8_t;
        flz_copy256(dest as *mut libc::c_void, src as *const libc::c_void);
        src = src.offset(32 as libc::c_int as isize);
        dest = dest.offset(32 as libc::c_int as isize);
        runs =
            (runs as
                 libc::c_uint).wrapping_sub(32 as libc::c_int as libc::c_uint)
                as uint32_t as uint32_t
    }
    if runs > 0 as libc::c_int as libc::c_uint {
        let fresh31 = dest;
        dest = dest.offset(1);
        *fresh31 =
            runs.wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t;
        flz_copy64(dest, src, runs);
        dest = dest.offset(runs as isize)
    }
    return dest;
}
/* special case of memcpy: at most 32 bytes */
unsafe extern "C" fn flz_smallcopy(mut dest: *mut uint8_t,
                                   mut src: *const uint8_t,
                                   mut count: uint32_t) {
    if count >= 8 as libc::c_int as libc::c_uint {
        let mut p: *const uint64_t =
            src as *const uint64_t; /* because readU32 */
        let mut q: *mut uint64_t = dest as *mut uint64_t;
        while count > 8 as libc::c_int as libc::c_uint {
            let fresh32 = p;
            p = p.offset(1);
            let fresh33 = q;
            q = q.offset(1);
            *fresh33 = *fresh32;
            count =
                (count as
                     libc::c_uint).wrapping_sub(8 as libc::c_int as
                                                    libc::c_uint) as uint32_t
                    as uint32_t;
            dest = dest.offset(8 as libc::c_int as isize);
            src = src.offset(8 as libc::c_int as isize)
        }
    }
    fastlz_memcpy(dest, src, count);
}
unsafe extern "C" fn flz_finalize(mut runs: uint32_t, mut src: *const uint8_t,
                                  mut dest: *mut uint8_t) -> *mut uint8_t {
    while runs >= 32 as libc::c_int as libc::c_uint {
        let fresh34 = dest;
        dest = dest.offset(1);
        *fresh34 = (32 as libc::c_int - 1 as libc::c_int) as uint8_t;
        flz_smallcopy(dest, src, 32 as libc::c_int as uint32_t);
        src = src.offset(32 as libc::c_int as isize);
        dest = dest.offset(32 as libc::c_int as isize);
        runs =
            (runs as
                 libc::c_uint).wrapping_sub(32 as libc::c_int as libc::c_uint)
                as uint32_t as uint32_t
    }
    if runs > 0 as libc::c_int as libc::c_uint {
        let fresh35 = dest;
        dest = dest.offset(1);
        *fresh35 =
            runs.wrapping_sub(1 as libc::c_int as libc::c_uint) as uint8_t;
        flz_smallcopy(dest, src, runs);
        dest = dest.offset(runs as isize)
    }
    return dest;
}
unsafe extern "C" fn flz1_match(mut len: uint32_t, mut distance: uint32_t,
                                mut op: *mut uint8_t) -> *mut uint8_t {
    distance = distance.wrapping_sub(1);
    if (len > (264 as libc::c_int - 2 as libc::c_int) as libc::c_uint) as
           libc::c_int as libc::c_long != 0 {
        while len > (264 as libc::c_int - 2 as libc::c_int) as libc::c_uint {
            let fresh36 = op;
            op = op.offset(1);
            *fresh36 =
                (((7 as libc::c_int) << 5 as libc::c_int) as
                     libc::c_uint).wrapping_add(distance >> 8 as libc::c_int)
                    as uint8_t;
            let fresh37 = op;
            op = op.offset(1);
            *fresh37 =
                (264 as libc::c_int - 2 as libc::c_int - 7 as libc::c_int -
                     2 as libc::c_int) as uint8_t;
            let fresh38 = op;
            op = op.offset(1);
            *fresh38 =
                (distance & 255 as libc::c_int as libc::c_uint) as uint8_t;
            len =
                (len as
                     libc::c_uint).wrapping_sub((264 as libc::c_int -
                                                     2 as libc::c_int) as
                                                    libc::c_uint) as uint32_t
                    as uint32_t
        }
    }
    if len < 7 as libc::c_int as libc::c_uint {
        let fresh39 = op;
        op = op.offset(1);
        *fresh39 =
            (len <<
                 5 as libc::c_int).wrapping_add(distance >> 8 as libc::c_int)
                as uint8_t;
        let fresh40 = op;
        op = op.offset(1);
        *fresh40 = (distance & 255 as libc::c_int as libc::c_uint) as uint8_t
    } else {
        let fresh41 = op;
        op = op.offset(1);
        *fresh41 =
            (((7 as libc::c_int) << 5 as libc::c_int) as
                 libc::c_uint).wrapping_add(distance >> 8 as libc::c_int) as
                uint8_t;
        let fresh42 = op;
        op = op.offset(1);
        *fresh42 =
            len.wrapping_sub(7 as libc::c_int as libc::c_uint) as uint8_t;
        let fresh43 = op;
        op = op.offset(1);
        *fresh43 = (distance & 255 as libc::c_int as libc::c_uint) as uint8_t
    }
    return op;
}
#[no_mangle]
pub unsafe extern "C" fn fastlz1_compress(mut input: *const libc::c_void,
                                          mut length: libc::c_int,
                                          mut output: *mut libc::c_void)
 -> libc::c_int {
    let mut ip: *const uint8_t = input as *const uint8_t;
    let mut ip_start: *const uint8_t = ip;
    let mut ip_bound: *const uint8_t =
        ip.offset(length as isize).offset(-(4 as libc::c_int as isize));
    let mut ip_limit: *const uint8_t =
        ip.offset(length as
                      isize).offset(-(12 as libc::c_int as
                                          isize)).offset(-(1 as libc::c_int as
                                                               isize));
    let mut op: *mut uint8_t = output as *mut uint8_t;
    let mut htab: [uint32_t; 16384] = [0; 16384];
    let mut seq: uint32_t = 0;
    let mut hash: uint32_t = 0;
    /* initializes hash table */
    hash = 0 as libc::c_int as uint32_t;
    while hash < ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_uint {
        htab[hash as usize] = 0 as libc::c_int as uint32_t;
        hash = hash.wrapping_add(1)
    }
    /* we start with literal copy */
    let mut anchor: *const uint8_t = ip;
    ip = ip.offset(2 as libc::c_int as isize);
    /* main loop */
    while (ip < ip_limit) as libc::c_int as libc::c_long != 0 {
        let mut ref_0: *const uint8_t = 0 as *const uint8_t;
        let mut distance: uint32_t = 0;
        let mut cmp: uint32_t = 0;
        loop 
             /* find potential match */
             {
            seq =
                flz_readu32(ip as *const libc::c_void) &
                    0xffffff as libc::c_int as libc::c_uint;
            hash = flz_hash(seq) as uint32_t;
            ref_0 = ip_start.offset(htab[hash as usize] as isize);
            htab[hash as usize] =
                ip.wrapping_offset_from(ip_start) as libc::c_long as uint32_t;
            distance =
                ip.wrapping_offset_from(ref_0) as libc::c_long as uint32_t;
            cmp =
                if (distance < 8192 as libc::c_int as libc::c_uint) as
                       libc::c_int as libc::c_long != 0 {
                    (flz_readu32(ref_0 as *const libc::c_void)) &
                        0xffffff as libc::c_int as libc::c_uint
                } else { 0x1000000 as libc::c_int as libc::c_uint };
            if (ip >= ip_limit) as libc::c_int as libc::c_long != 0 {
                break ;
            }
            ip = ip.offset(1);
            if !(seq != cmp) { break ; }
        }
        if (ip >= ip_limit) as libc::c_int as libc::c_long != 0 { break ; }
        ip = ip.offset(-1);
        if (ip > anchor) as libc::c_int as libc::c_long != 0 {
            op =
                flz_literals(ip.wrapping_offset_from(anchor) as libc::c_long
                                 as uint32_t, anchor, op)
        }
        let mut len: uint32_t =
            flz_cmp(ref_0.offset(3 as libc::c_int as isize),
                    ip.offset(3 as libc::c_int as isize), ip_bound);
        op = flz1_match(len, distance, op);
        /* update the hash at match boundary */
        ip = ip.offset(len as isize);
        seq = flz_readu32(ip as *const libc::c_void);
        hash =
            flz_hash(seq & 0xffffff as libc::c_int as libc::c_uint) as
                uint32_t;
        let fresh44 = ip;
        ip = ip.offset(1);
        htab[hash as usize] =
            fresh44.wrapping_offset_from(ip_start) as libc::c_long as
                uint32_t;
        seq >>= 8 as libc::c_int;
        hash = flz_hash(seq) as uint32_t;
        let fresh45 = ip;
        ip = ip.offset(1);
        htab[hash as usize] =
            fresh45.wrapping_offset_from(ip_start) as libc::c_long as
                uint32_t;
        anchor = ip
    }
    let mut copy: uint32_t =
        (input as
             *mut uint8_t).offset(length as
                                      isize).wrapping_offset_from(anchor) as
            libc::c_long as uint32_t;
    op = flz_finalize(copy, anchor, op);
    return op.wrapping_offset_from(output as *mut uint8_t) as libc::c_long as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fastlz1_decompress(mut input: *const libc::c_void,
                                            mut length: libc::c_int,
                                            mut output: *mut libc::c_void,
                                            mut maxout: libc::c_int)
 -> libc::c_int {
    let mut ip: *const uint8_t = input as *const uint8_t;
    let mut ip_limit: *const uint8_t = ip.offset(length as isize);
    let mut ip_bound: *const uint8_t =
        ip_limit.offset(-(2 as libc::c_int as isize));
    let mut op: *mut uint8_t = output as *mut uint8_t;
    let mut op_limit: *mut uint8_t = op.offset(maxout as isize);
    let fresh46 = ip;
    ip = ip.offset(1);
    let mut ctrl: uint32_t =
        (*fresh46 as libc::c_int & 31 as libc::c_int) as uint32_t;
    loop  {
        if ctrl >= 32 as libc::c_int as libc::c_uint {
            let mut len: uint32_t =
                (ctrl >>
                     5 as
                         libc::c_int).wrapping_sub(1 as libc::c_int as
                                                       libc::c_uint);
            let mut ofs: uint32_t =
                (ctrl & 31 as libc::c_int as libc::c_uint) <<
                    8 as libc::c_int;
            let mut ref_0: *const uint8_t =
                op.offset(-(ofs as
                                isize)).offset(-(1 as libc::c_int as isize));
            if len == (7 as libc::c_int - 1 as libc::c_int) as libc::c_uint {
                if !(ip <= ip_bound) as libc::c_int as libc::c_long != 0 {
                    return 0 as libc::c_int
                }
                let fresh47 = ip;
                ip = ip.offset(1);
                len =
                    (len as
                         libc::c_uint).wrapping_add(*fresh47 as libc::c_uint)
                        as uint32_t as uint32_t
            }
            let fresh48 = ip;
            ip = ip.offset(1);
            ref_0 = ref_0.offset(-(*fresh48 as libc::c_int as isize));
            len =
                (len as
                     libc::c_uint).wrapping_add(3 as libc::c_int as
                                                    libc::c_uint) as uint32_t
                    as uint32_t;
            if !(op.offset(len as isize) <= op_limit) as libc::c_int as
                   libc::c_long != 0 {
                return 0 as libc::c_int
            }
            if !(ref_0 >= output as *mut uint8_t) as libc::c_int as
                   libc::c_long != 0 {
                return 0 as libc::c_int
            }
            fastlz_memmove(op, ref_0, len);
            op = op.offset(len as isize)
        } else {
            ctrl = ctrl.wrapping_add(1);
            if !(op.offset(ctrl as isize) <= op_limit) as libc::c_int as
                   libc::c_long != 0 {
                return 0 as libc::c_int
            }
            if !(ip.offset(ctrl as isize) <= ip_limit) as libc::c_int as
                   libc::c_long != 0 {
                return 0 as libc::c_int
            }
            fastlz_memcpy(op, ip, ctrl);
            ip = ip.offset(ctrl as isize);
            op = op.offset(ctrl as isize)
        }
        if (ip > ip_bound) as libc::c_int as libc::c_long != 0 { break ; }
        let fresh49 = ip;
        ip = ip.offset(1);
        ctrl = *fresh49 as uint32_t
    }
    return op.wrapping_offset_from(output as *mut uint8_t) as libc::c_long as
               libc::c_int;
}
unsafe extern "C" fn flz2_match(mut len: uint32_t, mut distance: uint32_t,
                                mut op: *mut uint8_t) -> *mut uint8_t {
    distance = distance.wrapping_sub(1);
    if distance < 8191 as libc::c_int as libc::c_uint {
        if len < 7 as libc::c_int as libc::c_uint {
            let fresh50 = op;
            op = op.offset(1);
            *fresh50 =
                (len <<
                     5 as
                         libc::c_int).wrapping_add(distance >>
                                                       8 as libc::c_int) as
                    uint8_t;
            let fresh51 = op;
            op = op.offset(1);
            *fresh51 =
                (distance & 255 as libc::c_int as libc::c_uint) as uint8_t
        } else {
            let fresh52 = op;
            op = op.offset(1);
            *fresh52 =
                (((7 as libc::c_int) << 5 as libc::c_int) as
                     libc::c_uint).wrapping_add(distance >> 8 as libc::c_int)
                    as uint8_t;
            len =
                (len as
                     libc::c_uint).wrapping_sub(7 as libc::c_int as
                                                    libc::c_uint) as uint32_t
                    as uint32_t;
            while len >= 255 as libc::c_int as libc::c_uint {
                let fresh53 = op;
                op = op.offset(1);
                *fresh53 = 255 as libc::c_int as uint8_t;
                len =
                    (len as
                         libc::c_uint).wrapping_sub(255 as libc::c_int as
                                                        libc::c_uint) as
                        uint32_t as uint32_t
            }
            let fresh54 = op;
            op = op.offset(1);
            *fresh54 = len as uint8_t;
            let fresh55 = op;
            op = op.offset(1);
            *fresh55 =
                (distance & 255 as libc::c_int as libc::c_uint) as uint8_t
        }
    } else if len < 7 as libc::c_int as libc::c_uint {
        distance =
            (distance as
                 libc::c_uint).wrapping_sub(8191 as libc::c_int as
                                                libc::c_uint) as uint32_t as
                uint32_t;
        let fresh56 = op;
        op = op.offset(1);
        *fresh56 =
            (len <<
                 5 as
                     libc::c_int).wrapping_add(31 as libc::c_int as
                                                   libc::c_uint) as uint8_t;
        let fresh57 = op;
        op = op.offset(1);
        *fresh57 = 255 as libc::c_int as uint8_t;
        let fresh58 = op;
        op = op.offset(1);
        *fresh58 = (distance >> 8 as libc::c_int) as uint8_t;
        let fresh59 = op;
        op = op.offset(1);
        *fresh59 = (distance & 255 as libc::c_int as libc::c_uint) as uint8_t
    } else {
        distance =
            (distance as
                 libc::c_uint).wrapping_sub(8191 as libc::c_int as
                                                libc::c_uint) as uint32_t as
                uint32_t;
        let fresh60 = op;
        op = op.offset(1);
        *fresh60 =
            (((7 as libc::c_int) << 5 as libc::c_int) + 31 as libc::c_int) as
                uint8_t;
        len =
            (len as
                 libc::c_uint).wrapping_sub(7 as libc::c_int as libc::c_uint)
                as uint32_t as uint32_t;
        while len >= 255 as libc::c_int as libc::c_uint {
            let fresh61 = op;
            op = op.offset(1);
            *fresh61 = 255 as libc::c_int as uint8_t;
            len =
                (len as
                     libc::c_uint).wrapping_sub(255 as libc::c_int as
                                                    libc::c_uint) as uint32_t
                    as uint32_t
        }
        let fresh62 = op;
        op = op.offset(1);
        *fresh62 = len as uint8_t;
        let fresh63 = op;
        op = op.offset(1);
        *fresh63 = 255 as libc::c_int as uint8_t;
        let fresh64 = op;
        op = op.offset(1);
        *fresh64 = (distance >> 8 as libc::c_int) as uint8_t;
        let fresh65 = op;
        op = op.offset(1);
        *fresh65 = (distance & 255 as libc::c_int as libc::c_uint) as uint8_t
    }
    return op;
}
#[no_mangle]
pub unsafe extern "C" fn fastlz2_compress(mut input: *const libc::c_void,
                                          mut length: libc::c_int,
                                          mut output: *mut libc::c_void)
 -> libc::c_int {
    let mut ip: *const uint8_t = input as *const uint8_t;
    let mut ip_start: *const uint8_t = ip;
    /* far away, but not yet in the another galaxy... */
    let mut ip_bound: *const uint8_t =
        ip.offset(length as
                      isize).offset(-(4 as libc::c_int as
                                          isize)); /* because readU32 */
    let mut ip_limit: *const uint8_t =
        ip.offset(length as
                      isize).offset(-(12 as libc::c_int as
                                          isize)).offset(-(1 as libc::c_int as
                                                               isize));
    let mut op: *mut uint8_t = output as *mut uint8_t;
    let mut htab: [uint32_t; 16384] = [0; 16384];
    let mut seq: uint32_t = 0;
    let mut hash: uint32_t = 0;
    /* initializes hash table */
    hash = 0 as libc::c_int as uint32_t;
    while hash < ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_uint {
        htab[hash as usize] = 0 as libc::c_int as uint32_t;
        hash = hash.wrapping_add(1)
    }
    /* we start with literal copy */
    let mut anchor: *const uint8_t = ip;
    ip = ip.offset(2 as libc::c_int as isize);
    /* main loop */
    while (ip < ip_limit) as libc::c_int as libc::c_long != 0 {
        let mut ref_0: *const uint8_t = 0 as *const uint8_t;
        let mut distance: uint32_t = 0;
        let mut cmp: uint32_t = 0;
        loop 
             /* find potential match */
             {
            seq =
                flz_readu32(ip as *const libc::c_void) &
                    0xffffff as libc::c_int as libc::c_uint;
            hash = flz_hash(seq) as uint32_t;
            ref_0 = ip_start.offset(htab[hash as usize] as isize);
            htab[hash as usize] =
                ip.wrapping_offset_from(ip_start) as libc::c_long as uint32_t;
            distance =
                ip.wrapping_offset_from(ref_0) as libc::c_long as uint32_t;
            cmp =
                if (distance <
                        (65535 as libc::c_int + 8191 as libc::c_int -
                             1 as libc::c_int) as libc::c_uint) as libc::c_int
                       as libc::c_long != 0 {
                    (flz_readu32(ref_0 as *const libc::c_void)) &
                        0xffffff as libc::c_int as libc::c_uint
                } else { 0x1000000 as libc::c_int as libc::c_uint };
            if (ip >= ip_limit) as libc::c_int as libc::c_long != 0 {
                break ;
            }
            ip = ip.offset(1);
            if !(seq != cmp) { break ; }
        }
        if (ip >= ip_limit) as libc::c_int as libc::c_long != 0 { break ; }
        ip = ip.offset(-1);
        /* far, needs at least 5-byte match */
        if distance >= 8191 as libc::c_int as libc::c_uint {
            if *ref_0.offset(3 as libc::c_int as isize) as libc::c_int !=
                   *ip.offset(3 as libc::c_int as isize) as libc::c_int ||
                   *ref_0.offset(4 as libc::c_int as isize) as libc::c_int !=
                       *ip.offset(4 as libc::c_int as isize) as libc::c_int {
                ip = ip.offset(1);
                continue ;
            }
        }
        if (ip > anchor) as libc::c_int as libc::c_long != 0 {
            op =
                flz_literals(ip.wrapping_offset_from(anchor) as libc::c_long
                                 as uint32_t, anchor, op)
        }
        let mut len: uint32_t =
            flz_cmp(ref_0.offset(3 as libc::c_int as isize),
                    ip.offset(3 as libc::c_int as isize), ip_bound);
        op = flz2_match(len, distance, op);
        /* update the hash at match boundary */
        ip = ip.offset(len as isize);
        seq = flz_readu32(ip as *const libc::c_void);
        hash =
            flz_hash(seq & 0xffffff as libc::c_int as libc::c_uint) as
                uint32_t;
        let fresh66 = ip;
        ip = ip.offset(1);
        htab[hash as usize] =
            fresh66.wrapping_offset_from(ip_start) as libc::c_long as
                uint32_t;
        seq >>= 8 as libc::c_int;
        hash = flz_hash(seq) as uint32_t;
        let fresh67 = ip;
        ip = ip.offset(1);
        htab[hash as usize] =
            fresh67.wrapping_offset_from(ip_start) as libc::c_long as
                uint32_t;
        anchor = ip
    }
    let mut copy: uint32_t =
        (input as
             *mut uint8_t).offset(length as
                                      isize).wrapping_offset_from(anchor) as
            libc::c_long as uint32_t;
    op = flz_finalize(copy, anchor, op);
    /* marker for fastlz2 */
    let ref mut fresh68 = *(output as *mut uint8_t);
    *fresh68 =
        (*fresh68 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int) as
            uint8_t;
    return op.wrapping_offset_from(output as *mut uint8_t) as libc::c_long as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fastlz2_decompress(mut input: *const libc::c_void,
                                            mut length: libc::c_int,
                                            mut output: *mut libc::c_void,
                                            mut maxout: libc::c_int)
 -> libc::c_int {
    let mut ip: *const uint8_t = input as *const uint8_t;
    let mut ip_limit: *const uint8_t = ip.offset(length as isize);
    let mut ip_bound: *const uint8_t =
        ip_limit.offset(-(2 as libc::c_int as isize));
    let mut op: *mut uint8_t = output as *mut uint8_t;
    let mut op_limit: *mut uint8_t = op.offset(maxout as isize);
    let fresh69 = ip;
    ip = ip.offset(1);
    let mut ctrl: uint32_t =
        (*fresh69 as libc::c_int & 31 as libc::c_int) as uint32_t;
    loop  {
        if ctrl >= 32 as libc::c_int as libc::c_uint {
            let mut len: uint32_t =
                (ctrl >>
                     5 as
                         libc::c_int).wrapping_sub(1 as libc::c_int as
                                                       libc::c_uint);
            let mut ofs: uint32_t =
                (ctrl & 31 as libc::c_int as libc::c_uint) <<
                    8 as libc::c_int;
            let mut ref_0: *const uint8_t =
                op.offset(-(ofs as
                                isize)).offset(-(1 as libc::c_int as isize));
            let mut code: uint8_t = 0;
            if len == (7 as libc::c_int - 1 as libc::c_int) as libc::c_uint {
                loop  {
                    if !(ip <= ip_bound) as libc::c_int as libc::c_long != 0 {
                        return 0 as libc::c_int
                    }
                    let fresh70 = ip;
                    ip = ip.offset(1);
                    code = *fresh70;
                    len =
                        (len as
                             libc::c_uint).wrapping_add(code as libc::c_uint)
                            as uint32_t as uint32_t;
                    if !(code as libc::c_int == 255 as libc::c_int) {
                        break ;
                    }
                }
            }
            let fresh71 = ip;
            ip = ip.offset(1);
            code = *fresh71;
            ref_0 = ref_0.offset(-(code as libc::c_int as isize));
            len =
                (len as
                     libc::c_uint).wrapping_add(3 as libc::c_int as
                                                    libc::c_uint) as uint32_t
                    as uint32_t;
            /* match from 16-bit distance */
            if (code as libc::c_int == 255 as libc::c_int) as libc::c_int as
                   libc::c_long != 0 {
                if (ofs ==
                        ((31 as libc::c_int) << 8 as libc::c_int) as
                            libc::c_uint) as libc::c_int as libc::c_long != 0
                   {
                    if !(ip < ip_bound) as libc::c_int as libc::c_long != 0 {
                        return 0 as libc::c_int
                    }
                    let fresh72 = ip;
                    ip = ip.offset(1);
                    ofs =
                        ((*fresh72 as libc::c_int) << 8 as libc::c_int) as
                            uint32_t;
                    let fresh73 = ip;
                    ip = ip.offset(1);
                    ofs =
                        (ofs as
                             libc::c_uint).wrapping_add(*fresh73 as
                                                            libc::c_uint) as
                            uint32_t as uint32_t;
                    ref_0 =
                        op.offset(-(ofs as
                                        isize)).offset(-(8191 as libc::c_int
                                                             as
                                                             isize)).offset(-(1
                                                                                  as
                                                                                  libc::c_int
                                                                                  as
                                                                                  isize))
                }
            }
            if !(op.offset(len as isize) <= op_limit) as libc::c_int as
                   libc::c_long != 0 {
                return 0 as libc::c_int
            }
            if !(ref_0 >= output as *mut uint8_t) as libc::c_int as
                   libc::c_long != 0 {
                return 0 as libc::c_int
            }
            fastlz_memmove(op, ref_0, len);
            op = op.offset(len as isize)
        } else {
            ctrl = ctrl.wrapping_add(1);
            if !(op.offset(ctrl as isize) <= op_limit) as libc::c_int as
                   libc::c_long != 0 {
                return 0 as libc::c_int
            }
            if !(ip.offset(ctrl as isize) <= ip_limit) as libc::c_int as
                   libc::c_long != 0 {
                return 0 as libc::c_int
            }
            fastlz_memcpy(op, ip, ctrl);
            ip = ip.offset(ctrl as isize);
            op = op.offset(ctrl as isize)
        }
        if (ip >= ip_limit) as libc::c_int as libc::c_long != 0 { break ; }
        let fresh74 = ip;
        ip = ip.offset(1);
        ctrl = *fresh74 as uint32_t
    }
    return op.wrapping_offset_from(output as *mut uint8_t) as libc::c_long as
               libc::c_int;
}
/* *
  DEPRECATED.

  This is similar to fastlz_compress_level above, but with the level
  automatically chosen.

  This function is deprecated and it will be completely removed in some future
  version.
*/
#[no_mangle]
pub unsafe extern "C" fn fastlz_compress(mut input: *const libc::c_void,
                                         mut length: libc::c_int,
                                         mut output: *mut libc::c_void)
 -> libc::c_int {
    /* for short block, choose fastlz1 */
    if length < 65536 as libc::c_int {
        return fastlz1_compress(input, length, output)
    }
    /* else... */
    return fastlz2_compress(input, length, output);
}
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
pub unsafe extern "C" fn fastlz_decompress(mut input: *const libc::c_void,
                                           mut length: libc::c_int,
                                           mut output: *mut libc::c_void,
                                           mut maxout: libc::c_int)
 -> libc::c_int {
    /* magic identifier for compression level */
    let mut level: libc::c_int =
        (*(input as *const uint8_t) as libc::c_int >> 5 as libc::c_int) +
            1 as libc::c_int;
    if level == 1 as libc::c_int {
        return fastlz1_decompress(input, length, output, maxout)
    }
    if level == 2 as libc::c_int {
        return fastlz2_decompress(input, length, output, maxout)
    }
    /* unknown level, trigger error */
    return 0 as libc::c_int;
}
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
pub unsafe extern "C" fn fastlz_compress_level(mut level: libc::c_int,
                                               mut input: *const libc::c_void,
                                               mut length: libc::c_int,
                                               mut output: *mut libc::c_void)
 -> libc::c_int {
    if level == 1 as libc::c_int {
        return fastlz1_compress(input, length, output)
    }
    if level == 2 as libc::c_int {
        return fastlz2_compress(input, length, output)
    }
    return 0 as libc::c_int;
}
