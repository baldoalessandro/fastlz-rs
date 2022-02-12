use crate::consts::{
    MAX_COPY,
    MAX_LEN,
    MAX_L2_DISTANCE,

    HASH_LOG,
    HASH_MASK
};

pub fn flz_cmp(
    p: &[u8],
    q: &[u8]
) -> usize {
    p.iter()
     .zip(q.iter())
     .take_while(|(&p, &q)| p == q)
     .count()
}

pub unsafe fn flz_cmp_unsafe(
    mut p: *const u8,
    mut q: *const u8,
    r: *const u8
) -> usize {
    let mut count: usize = 0;

    while (q < r) && (*p != *q) {
        p = p.add(1);
        q = q.add(1);
        count += 1;
    }

    count
}

pub fn flz_readu32(p: &[u8]) -> u32 {
    u32::from_ne_bytes([p[0], p[1], p[2], p[3]])
}

pub unsafe fn flz_readu32_unsafe(ptr: *const u8) -> u32 {
    *(ptr as *const u32)
}

pub fn flz_readu24(p: &[u8]) -> u32 {
    u32::from_ne_bytes([p[0], p[1], p[2], 0])
}

pub fn flz_hash(v: u32) -> u16 {
    let h = (v as u64 * 2654435769u64) >> (32 - HASH_LOG);
    (h as u16) & HASH_MASK
}

pub fn flz_literals(
    input: &[u8],
    output: &mut [u8]
) -> usize {
    let mut dest = 0;

    for run in input.chunks(MAX_COPY) {
        let run_len = run.len();
        output[dest] = (run_len - 1) as u8;
        dest += 1;
        output[dest..(dest + run_len)].copy_from_slice(run);
        dest += run_len;
    }

    dest
}

pub unsafe fn flz_literals_unsafe(
    mut runs: usize,
    mut src: *const u8,
    mut dest: *mut u8
) -> *mut u8 {
    while runs >= MAX_COPY {
        *dest = (MAX_COPY - 1) as u8;
        dest = dest.add(1);
        dest.copy_from(src, MAX_COPY); // flz_copy256
        src = src.add(MAX_COPY);
        dest = dest.add(MAX_COPY);
        runs -= MAX_COPY;
    }

    if runs > 0 {
        *dest = (runs - 1) as u8;
        dest = dest.add(1);
        // Note: flz_copy64 seems to have an overflow bug which causes rust to
        // error out. As such, we're taking a different approach here.
        dest.copy_from(src, runs);
        dest = dest.add(runs);
    }

    dest
}

pub fn flz_finalize(
    input: &[u8],
    output: &mut [u8]
) -> usize {
    let mut dest = 0;

    for run in input.chunks(MAX_COPY) {
        let run_len = run.len();
        output[dest] = (run_len - 1) as u8;
        dest += 1;
        output[dest..(dest + run_len)].copy_from_slice(run);
        dest += run_len;
    }

    dest
}

pub unsafe fn flz_finalize_unsafe(
    mut runs: usize,
    mut src: *const u8,
    mut dest: *mut u8
) -> *mut u8 {
    while runs >= MAX_COPY {
        *dest = (MAX_COPY - 1) as u8;
        dest = dest.add(1);
        dest.copy_from_nonoverlapping(src, MAX_COPY); // flz_smallcopy
        src = src.add(MAX_COPY);
        dest = dest.add(MAX_COPY);
        runs -= MAX_COPY;
    }

    if runs > 0 {
        *dest = (runs - 1) as u8;
        dest = dest.add(1);
        dest.copy_from_nonoverlapping(src, runs); // flz_smallcopy
        dest = dest.add(runs);
    }

    dest
}

pub fn flz1_match(
    mut len: usize,
    mut distance: u32,
    output: &mut [u8]
) -> usize {
    let mut op = 0;

    distance -= 1;

    if fastlz_unlikely!(len > (MAX_LEN - 2)) {
        while len > (MAX_LEN - 2) {
            output[op] = (7 << 5) + (distance >> 8) as u8;
            op += 1;
            output[op] = (MAX_LEN - 2 - 7 - 2) as u8;
            op += 1;
            output[op] = (distance & 255) as u8;
            op += 1;
            len -= MAX_LEN - 2;
        }
    }

    let len = len as u8;
    if len < 7 {
        output[op] = (len << 5) + (distance >> 8) as u8;
        op += 1;
        output[op] = (distance & 255) as u8;
        op += 1;
    } else {
        output[op] = (7 << 5) + (distance >> 8) as u8;
        op += 1;
        output[op] = len - 7;
        op += 1;
        output[op] = (distance & 255) as u8;
        op += 1;
    }

    op
}

pub unsafe fn flz1_match_unsafe(
    mut len: usize,
    mut distance: u32,
    mut op: *mut u8
) -> *mut u8 {
    distance -= 1;

    if fastlz_unlikely!(len > (MAX_LEN - 2)) {
        while len > (MAX_LEN - 2) {
            *op = ((7 << 5) + (distance >> 8)) as u8;
            op = op.add(1);
            *op = (MAX_LEN - 2 - 7 - 2) as u8;
            op = op.add(1);
            *op = (distance & 255) as u8;
            op = op.add(1);
            len -= MAX_LEN - 2;
        }
    }

    let len = len as u8;
    if len < 7 {
        *op = (len << 5) + (distance >> 8) as u8;
        op = op.add(1);
        *op = (distance & 255) as u8;
        op = op.add(1);
    } else {
        *op = (7 << 5) + (distance >> 8) as u8;
        op = op.add(1);
        *op = len - 7;
        op = op.add(1);
        *op = (distance & 255) as u8;
        op = op.add(1);
    }

    op
}

pub fn flz2_match(
    mut len: usize,
    mut distance: u32,
    output: &mut [u8]
) -> usize {
    let mut op = 0;

    distance -= 1;

    if distance < (MAX_L2_DISTANCE as u32) {
        if len < 7 {
            output[op] = (len << 5) as u8 + (distance >> 8) as u8;
            op += 1;
            output[op] = (distance & 255) as u8;
            op += 1;
        } else {
            output[op] = (7 << 5) + (distance >> 8) as u8;
            op += 1;
            len -= 7;
            while len >= 255 {
                output[op] = 255;
                op += 1;
                len -= 255;
            }
            output[op] = len as u8;
            op += 1;
            output[op] = (distance & 255) as u8;
            op += 1;
        }
    } else {
        // far away, but not yet in the another galaxy...
        distance -= MAX_L2_DISTANCE as u32;

        if len < 7 {  
            output[op] = (len << 5) as u8 + 31;
            op += 1;
            output[op] = 255;
            op += 1;
            output[op] = (distance >> 8) as u8;
            op += 1;
            output[op] = (distance & 255) as u8;
            op += 1;
        } else {
            output[op] = (7 << 5) + 31;
            op += 1;
            len -= 7;
            while len >= 255 {
                output[op] = 255;
                op += 1;
                len -= 255;
            }
            output[op] = len as u8;
            op += 1;
            output[op] = 255;
            op += 1;
            output[op] = (distance >> 8) as u8;
            op += 1;
            output[op] = (distance & 255) as u8;
            op += 1;
        }
    }

    op
}


pub unsafe fn flz2_match_unsafe(
    mut len: u32,
    mut distance: u32,
    mut op: *mut u8
) -> *mut u8 {
    distance -= 1;

    if (distance as usize) < MAX_L2_DISTANCE {
        if len < 7 {
            *op = (len << 5) as u8 + (distance >> 8) as u8;
            op = op.add(1);
            *op = (distance & 255) as u8;
            op = op.add(1);
        } else {
            *op = (7 << 5) + (distance >> 8) as u8;
            op = op.add(1);
            len -= 7;
            while len >= 255 {
                *op = 255;
                op = op.add(1);
                len -= 255;
            }
            *op = len as u8;
            op = op.add(1);
            *op = (distance & 255) as u8;
            op = op.add(1);
        }
    } else {
        // far away, but not yet in the another galaxy...
        distance -= MAX_L2_DISTANCE as u32;

        if len < 7 {  
            *op = (len << 5) as u8 + 31;
            op = op.add(1);
            *op = 255;
            op = op.add(1);
            *op = (distance >> 8) as u8;
            op = op.add(1);
            *op = (distance & 255) as u8;
            op = op.add(1);
        } else {
            *op = (7 << 5) + 31;
            op = op.add(1);
            len -= 7;
            while len >= 255 {
                *op = 255;
                op = op.add(1);
                len -= 255;
            }
            *op = len as u8;
            op = op.add(1);
            *op = 255;
            op = op.add(1);
            *op = (distance >> 8) as u8;
            op = op.add(1);
            *op = (distance & 255) as u8;
            op = op.add(1);
        }
    }

    op
}
