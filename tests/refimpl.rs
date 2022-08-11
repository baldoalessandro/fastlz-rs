pub fn ref_level1_decompress(
    input: &[u8],
    length: usize,
    output: &mut [u8]
) {
    let mut src = 0;
    let mut dest = 0;

    while src < length {
        let type_0 = input[src] >> 5;
        if type_0 == 0 {
            // literal run
            let run = 1 + (input[src] as usize);
            src += 1;
            for _ in 0..run {
                output[dest] = input[src];
                src += 1;
                dest += 1;
            }
        } else if type_0 < 7 {
            // short match
            let ofs =
                256 *
                    ((input[src] & 31) as usize) +
                    (input[src + 1] as usize);
            let len = 2 + ((input[src] >> 5) as usize);
            src += 2;
            let mut ref_ = dest - ofs - 1;
            for _ in 0..len {
                output[dest] = output[ref_];
                ref_ += 1;
                dest += 1;
            }
        } else {
            // long match
            let ofs =
                256 *
                    ((input[src] & 31) as usize) +
                    (input[src + 2] as usize);
            let len = 9 + (input[src + 1] as usize);
            src += 3;
            let mut ref_ = dest - ofs - 1;
            for _ in 0..len {
                output[dest] = output[ref_];
                ref_ += 1;
                dest += 1;
            }
        }
    };
}

pub fn ref_level2_decompress(
    input: &[u8],
    length: usize,
    output: &mut [u8]
) {
    let mut src = 0;
    let mut dest = 0;

    while src < length {
        let type_0 = input[src] >> 5;
        if type_0 == 0 {
            // literal run
            let run = 1 + (input[src] as usize);
            src += 1;
            for _ in 0..run {
                output[dest] = input[src];
                src += 1;
                dest += 1;
            }
        } else {
            let mut next = 2;
            let mut len = 2 + ((input[src] >> 5) as usize);
            if len == 9 {
                // long match
                next += 1;
                len += input[src + 1] as usize;
                if len == (9 + 255) {
                    // Gamma code for match length
                    let mut nn = input[src + 1] as usize;
                    while nn == 255 {
                        nn = input[src + next - 1] as usize;
                        next += 1;
                        len += nn;
                    }
                }
            }
            let mut ofs =
                256 *
                    ((input[src] & 31) as usize) +
                    (input[src + next - 1] as usize);
            if ofs == 8191 {
                // match from 16-bit distance
                ofs +=
                    256 *
                        (input[src + next] as usize) +
                        (input[src + next + 1] as usize);
                next += 2;
            }
            src += next;
            let mut ref_0 = dest - ofs - 1;
            for _ in 0..len {
                output[dest] = output[ref_0];
                ref_0 += 1;
                dest += 1;
            }
        }
    };
}
