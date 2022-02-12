use crate::consts::{
    HASH_SIZE,

    MAX_L1_DISTANCE,
    MAX_L2_DISTANCE,
    MAX_FARDISTANCE
};

use crate::utils::{
    flz_readu24,
    flz_hash,
    flz_literals,
    flz_cmp,
    flz1_match,
    flz2_match,
    flz_finalize
};

pub fn fastlz1_compress(
    input: &[u8],
    output: &mut [u8]
) -> Option<usize> {
    let mut ip = 0;
    let ip_len = input.len();
    let ip_bound = ip_len - 4; // because readU32
    let ip_limit = ip_len - 12 - 1;

    let mut op = 0;

    // hash table is already initalized
    let mut htab: [usize; HASH_SIZE] = [0; HASH_SIZE];

    // we start with literal copy
    let mut anchor = 0;
    ip += 2;

    // main loop
    while ip < ip_limit {
        // find potential match
        let seq = flz_readu24(&input[ip..]);
        let hash = flz_hash(seq) as usize;
        let ref_0 = htab[hash];
        htab[hash] = ip;

        let distance = ip - ref_0;

        let cmp = match distance < MAX_L1_DISTANCE {
            true => flz_readu24(&input[ref_0..]),
            false => 0x1000000,
        };

        if seq != cmp {
            ip += 1;
            continue;
        }

        if ip > anchor {
            op += flz_literals(&input[anchor..ip], &mut output[op..]);
        }

        let len = flz_cmp(&input[ref_0+3..], &input[ip+3..ip_bound]);
        op += flz1_match(len, distance as u32, &mut output[op..]);

        // update the hash at match boundary
        ip += len;

        let seq = flz_readu24(&input[ip..]);
        let hash = flz_hash(seq) as usize;
        htab[hash] = ip;
        ip += 1;

        let seq = flz_readu24(&input[ip..]);
        let hash = flz_hash(seq) as usize;
        htab[hash] = ip;
        ip += 1;

        anchor = ip
    }

    op += flz_finalize(&input[anchor..], &mut output[op..]);

    Some(op)
}

pub fn fastlz2_compress(
    input: &[u8],
    output: &mut [u8]
) -> Option<usize> {
    let mut ip = 0;
    let ip_end = input.len();
    let ip_bound = ip_end - 4; // because readU32
    let ip_limit = ip_end - 12 - 1;

    let mut op = 0;

    // hash table is already initalized
    let mut htab: [usize; HASH_SIZE] = [0; HASH_SIZE];
 
    // we start with literal copy
    let mut anchor = 0;
    ip += 2;

    // main loop
    while ip < ip_limit {
        // find potential match
        let seq = flz_readu24(&input[ip..]);
        let hash = flz_hash(seq) as usize;
        let ref_0 = htab[hash];
        htab[hash] = ip;

        let distance = ip - ref_0;

        let cmp = match fastlz_likely!(distance < MAX_FARDISTANCE) {
            true => flz_readu24(&input[ref_0..]),
            false => 0x1000000,
        };

        if seq != cmp {
            ip += 1;
            continue;
        }

        // far, needs at least 5-byte match
        if distance >= MAX_L2_DISTANCE {
            if input[ref_0+3..(ref_0+3+2)] != input[(ip+3)..(ip+3+2)] {
                ip += 1;
                continue;
            }
        }

        if fastlz_likely!(ip > anchor) {
            op += flz_literals(&input[anchor..ip], &mut output[op..]);
        }
    
        let len = flz_cmp(&input[ref_0+3..], &input[ip+3..ip_bound]);
        op += flz2_match(len, distance as u32, &mut output[op..]);

        // update the hash at match boundary
        ip += len;

        let seq = flz_readu24(&input[ip..]);
        let hash = flz_hash(seq) as usize;
        htab[hash] = ip;
        ip += 1;

        let seq = flz_readu24(&input[ip..]);
        let hash = flz_hash(seq) as usize;
        htab[hash] = ip;
        ip += 1;

        anchor = ip;
    }

    op += flz_finalize(&input[anchor..], &mut output[op..]);

    // marker for fastlz2
    output[0] |= 1 << 5;

    Some(op)
}

/// Compress a block of data in the input buffer and returns the size of
/// compressed block. The size of input buffer is specified by length. The
/// minimum input buffer size is 16.
///
/// The output buffer must be at least 5% larger than the input buffer
/// and can not be smaller than 66 bytes.
///
/// If the input is not compressible, the return value might be larger than
/// length (input buffer size).
///
/// The input buffer and the output buffer can not overlap.
///
/// Compression level can be specified in parameter level. At the moment,
/// only level 1 and level 2 are supported.
/// Level 1 is the fastest compression and generally useful for short data.
/// Level 2 is slightly slower but it gives better compression ratio.
///
/// Note that the compressed data, regardless of the level, can always be
/// decompressed using the function fastlz_decompress below.
///
pub fn compress(
    level: u8,
    input: &[u8],
    output: &mut [u8]
) -> Option<usize> {
    match level {
        1 => fastlz1_compress(input, output),
        2 => fastlz2_compress(input, output),
        // unknown level, trigger error
        _ => None
    }
}
