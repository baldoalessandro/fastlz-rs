use crate::consts::MAX_L2_DISTANCE;

fn fastlz1_decompress(
    input: &[u8],
    output: &mut [u8]
) -> Option<usize> {
    let mut dest = 0;

    let mut iter = input.iter();

    // The first instruction is always a literal run,
    // but the compression level info might mess up
    // the first code opcode_0.
    // In order to avoid this issue, we're going to 
    // make a new iterator where we change the first
    // opcode_0 without mutating the input array.
    // to change the instruction into a literal run
    let mut opcode_0 = iter.next().map(|&x| x & 31);

    while let Some(opcode) = opcode_0 {
        let op_type = opcode >> 5;
        let op_data = opcode & 31;

        match op_type {
            0b000 => {
                // literal run
                let run = 1 + op_data as usize;

                output[dest..(dest + run)].copy_from_slice(&iter.as_slice()[..run]);
                iter.advance_by(run).ok()?;
                dest += run;
            },

            0b111 => {
                // long match
                let opcode_1 = *iter.next()?;
                let opcode_2 = *iter.next()?;

                let ofs = ((op_data as usize) << 8) + (opcode_2 as usize);

                let len = 9 + (opcode_1 as usize);

                let ref_0 = dest - ofs - 1;

                output.copy_within(ref_0..(ref_0 + len), dest);
                dest += len;
            },

            _ => {
                // short match
                let opcode_1 = *iter.next()?;

                let len = 2 + (op_type as usize);
                let ofs = ((op_data as usize) << 8) + (opcode_1 as usize);

                let ref_0 = dest - ofs - 1;

                output.copy_within(ref_0..(ref_0 + len), dest);
                dest += len;
            }
        }

        opcode_0 = iter.next().copied();
    };

    Some(dest)
}

fn fastlz2_decompress(
    input: &[u8],
    output: &mut [u8]
) -> Option<usize> {
    let mut dest = 0;

    let mut iter = input.iter();

    // The first instruction is always a literal run,
    // but the compression level info might mess up
    // the first code opcode_0.
    // In order to avoid this issue, we're going to 
    // make a new iterator where we change the first
    // opcode_0 without mutating the input array.
    // to change the instruction into a literal run
    let mut opcode_0 = iter.next().map(|&x| x & 31);

    while let Some(opcode) = opcode_0 {
        let op_type = opcode >> 5;
        let op_data = opcode & 31;

        match op_type {
            0b000 => {
                // literal run
                let run = 1 + op_data as usize;

                output[dest..(dest + run)].copy_from_slice(&iter.as_slice()[..run]);
                iter.advance_by(run).ok()?;
                dest += run;
            },

            0b111 => {
                // long match
                let mut len = 9;

                for &nn in iter.by_ref() {
                    len += nn as usize;
                    if nn != 255 { break; }
                }

                let mut ofs = (op_data as usize) << 8;
                ofs += *iter.next()? as usize;

                if ofs == MAX_L2_DISTANCE {
                    // match from 16-bit distance
                    ofs += (*iter.next()? as usize) << 8;
                    ofs += *iter.next()? as usize;
                }
    
                let ref_0 = dest - ofs - 1;
                output.copy_within(ref_0..(ref_0 + len), dest);
                dest += len;
            },

            _ => {
                // short match
                let len = 2 + op_type as usize;

                let mut ofs = (op_data as usize) << 8;
                ofs += *iter.next()? as usize;

                if ofs == MAX_L2_DISTANCE {
                    // match from 16-bit distance
                    ofs += (*iter.next()? as usize) << 8;
                    ofs += *iter.next()? as usize;
                }

                let ref_0 = dest - ofs - 1;
                output.copy_within(ref_0..(ref_0 + len), dest);
                dest += len;
            },
        }

        opcode_0 = iter.next().copied();
    };

    Some(dest)
}

/// Decompress a block of compressed data and returns the size of the
/// decompressed block. If error occurs, e.g. the compressed data is
/// corrupted or the output buffer is not large enough, then 0 (zero)
/// will be returned instead.
///
/// The input buffer and the output buffer can not overlap.
///
/// Decompression is memory safe and guaranteed not to write the output buffer
/// more than what is specified in maxout.
///
/// Note that the decompression will always work, regardless of the
/// compression level specified in fastlz_compress_level (when
/// producing the compressed block).
///
pub fn decompress(
    input: &[u8],
    output: &mut [u8]
) -> Option<usize> {
    // magic identifier for compression level
    let level = input.first().map(|&x| (x >> 5) + 1);

    match level {
        Some(i) => decompress_level(i, input, output),
        None => None,
    }
}

fn decompress_level(
    level: u8,
    input: &[u8],
    output: &mut [u8]
) -> Option<usize> {
    match level {
        1 => fastlz1_decompress(input, output),
        2 => fastlz2_decompress(input, output),
        // unknown level, trigger error
        _ => None
    }
}
