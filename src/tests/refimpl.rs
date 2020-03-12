use ::libc;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
#[no_mangle]
pub unsafe extern "C" fn REF_Level1_decompress(mut input: *const uint8_t,
                                               mut length: libc::c_int,
                                               mut output: *mut uint8_t) {
    let mut src: libc::c_int = 0 as libc::c_int;
    let mut dest: libc::c_int = 0 as libc::c_int;
    while src < length {
        let mut type_0: libc::c_int =
            *input.offset(src as isize) as libc::c_int >> 5 as libc::c_int;
        if type_0 == 0 as libc::c_int {
            /* literal run */
            let mut run: libc::c_int =
                1 as libc::c_int + *input.offset(src as isize) as libc::c_int;
            src = src + 1 as libc::c_int;
            while run > 0 as libc::c_int {
                *output.offset(dest as isize) = *input.offset(src as isize);
                src = src + 1 as libc::c_int;
                dest = dest + 1 as libc::c_int;
                run = run - 1 as libc::c_int
            }
        } else if type_0 < 7 as libc::c_int {
            /* short match */
            let mut ofs: libc::c_int =
                256 as libc::c_int *
                    (*input.offset(src as isize) as libc::c_int &
                         31 as libc::c_int) +
                    *input.offset((src + 1 as libc::c_int) as isize) as
                        libc::c_int;
            let mut len: libc::c_int =
                2 as libc::c_int +
                    (*input.offset(src as isize) as libc::c_int >>
                         5 as libc::c_int);
            src = src + 2 as libc::c_int;
            let mut ref_0: libc::c_int = dest - ofs - 1 as libc::c_int;
            while len > 0 as libc::c_int {
                *output.offset(dest as isize) =
                    *output.offset(ref_0 as isize);
                ref_0 = ref_0 + 1 as libc::c_int;
                dest = dest + 1 as libc::c_int;
                len = len - 1 as libc::c_int
            }
        } else {
            /* long match */
            let mut ofs_0: libc::c_int =
                256 as libc::c_int *
                    (*input.offset(src as isize) as libc::c_int &
                         31 as libc::c_int) +
                    *input.offset((src + 2 as libc::c_int) as isize) as
                        libc::c_int;
            let mut len_0: libc::c_int =
                9 as libc::c_int +
                    *input.offset((src + 1 as libc::c_int) as isize) as
                        libc::c_int;
            src = src + 3 as libc::c_int;
            let mut ref_1: libc::c_int = dest - ofs_0 - 1 as libc::c_int;
            while len_0 > 0 as libc::c_int {
                *output.offset(dest as isize) =
                    *output.offset(ref_1 as isize);
                ref_1 = ref_1 + 1 as libc::c_int;
                dest = dest + 1 as libc::c_int;
                len_0 = len_0 - 1 as libc::c_int
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn REF_Level2_decompress(mut input: *const uint8_t,
                                               mut length: libc::c_int,
                                               mut output: *mut uint8_t) {
    let mut src: libc::c_int = 0 as libc::c_int;
    let mut dest: libc::c_int = 0 as libc::c_int;
    while src < length {
        let mut type_0: libc::c_int =
            *input.offset(src as isize) as libc::c_int >> 5 as libc::c_int;
        if type_0 == 0 as libc::c_int {
            /* literal run */
            let mut run: libc::c_int =
                1 as libc::c_int + *input.offset(src as isize) as libc::c_int;
            src = src + 1 as libc::c_int;
            while run > 0 as libc::c_int {
                *output.offset(dest as isize) = *input.offset(src as isize);
                src = src + 1 as libc::c_int;
                dest = dest + 1 as libc::c_int;
                run = run - 1 as libc::c_int
            }
        } else {
            let mut next: libc::c_int = 2 as libc::c_int;
            let mut len: libc::c_int =
                2 as libc::c_int +
                    (*input.offset(src as isize) as libc::c_int >>
                         5 as libc::c_int);
            if len == 9 as libc::c_int {
                /* long match */
                next = next + 1 as libc::c_int;
                len =
                    len +
                        *input.offset((src + 1 as libc::c_int) as isize) as
                            libc::c_int;
                if len == 9 as libc::c_int + 255 as libc::c_int {
                    /* Gamma code for match length */
                    let mut nn: libc::c_int =
                        *input.offset((src + 1 as libc::c_int) as isize) as
                            libc::c_int;
                    while nn == 255 as libc::c_int {
                        nn =
                            *input.offset((src + next - 1 as libc::c_int) as
                                              isize) as libc::c_int;
                        next = next + 1 as libc::c_int;
                        len += nn
                    }
                }
            }
            let mut ofs: libc::c_int =
                256 as libc::c_int *
                    (*input.offset(src as isize) as libc::c_int &
                         31 as libc::c_int) +
                    *input.offset((src + next - 1 as libc::c_int) as isize) as
                        libc::c_int;
            if ofs == 8191 as libc::c_int {
                /* match from 16-bit distance */
                ofs +=
                    256 as libc::c_int *
                        *input.offset((src + next) as isize) as libc::c_int +
                        *input.offset((src + next + 1 as libc::c_int) as
                                          isize) as libc::c_int;
                next = next + 2 as libc::c_int
            }
            src = src + next;
            let mut ref_0: libc::c_int = dest - ofs - 1 as libc::c_int;
            while len > 0 as libc::c_int {
                *output.offset(dest as isize) =
                    *output.offset(ref_0 as isize);
                ref_0 = ref_0 + 1 as libc::c_int;
                dest = dest + 1 as libc::c_int;
                len = len - 1 as libc::c_int
            }
        }
    };
}
