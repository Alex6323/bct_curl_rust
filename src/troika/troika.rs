use super::constants::*;
use super::luts::*;

pub fn troika(out_buf: &mut [Trit], in_buf: &[u8]) {
    troika_var_rounds(out_buf, in_buf, NUM_ROUNDS)
}

pub fn troika_var_rounds(out_buf: &mut [Trit], in_buf: &[u8], num_rounds: usize) {
    let out_len = out_buf.len();
    let in_len = in_buf.len();
    let mut state = [0u8; STATE_SIZE];

    troika_absorb(&mut state, TROIKA_RATE, in_buf, in_len, num_rounds);
    troika_squeeze(out_buf, out_len, TROIKA_RATE, &mut state, num_rounds);
}

fn sub_trytes(state: &mut [Trit]) {
    for sbox_idx in 0..NUM_SBOXES {
        let sbox_input =
            9 * state[3 * sbox_idx] + 3 * state[3 * sbox_idx + 1] + state[3 * sbox_idx + 2];

        let mut sbox_output = SBOX_LOOKUP[sbox_input as usize];

        state[3 * sbox_idx + 2] = sbox_output % 3;
        sbox_output /= 3;
        state[3 * sbox_idx + 1] = sbox_output % 3;
        sbox_output /= 3;
        state[3 * sbox_idx] = sbox_output % 3;
    }
}

fn shift_rows(state: &mut [Trit]) {
    let mut new_state = [0u8; STATE_SIZE];

    for slice in 0..SLICES {
        for row in 0..ROWS {
            for col in 0..COLUMNS {
                let old_idx = SLICE_SIZE * slice + COLUMNS * row + col;
                let new_idx = SLICE_SIZE * slice
                    + COLUMNS * row
                    + (col + 3 * SHIFT_ROWS_PARAM[row]) % COLUMNS;
                new_state[new_idx] = state[old_idx];
            }
        }
    }

    state.copy_from_slice(&new_state[..]);
}

fn shift_lanes(state: &mut [Trit]) {
    let mut new_state = [0u8; STATE_SIZE];

    for slice in 0..SLICES {
        for row in 0..ROWS {
            for col in 0..COLUMNS {
                let old_idx = SLICE_SIZE * slice + COLUMNS * row + col;
                let new_slice = (slice + SHIFT_LANES_PARAM[col + COLUMNS * row]) % SLICES;
                let new_idx = SLICE_SIZE * new_slice + COLUMNS * row + col;
                new_state[new_idx] = state[old_idx];
            }
        }
    }

    state.copy_from_slice(&new_state[..]);
}

fn add_column_parity(state: &mut [Trit]) {
    let mut parity = [0u8; SLICES * COLUMNS];

    // First compute parity for each column
    for slice in 0..SLICES {
        for col in 0..COLUMNS {
            let mut col_sum = 0;
            for row in 0..ROWS {
                col_sum += state[SLICE_SIZE * slice + COLUMNS * row + col];
            }
            parity[COLUMNS * slice + col] = col_sum % 3;
        }
    }

    // Add parity
    for slice in 0..SLICES {
        for row in 0..ROWS {
            for col in 0..COLUMNS {
                let idx = SLICE_SIZE * slice + COLUMNS * row + col;
                //let sum_to_add = parity[(col - 1 + 9) % 9 + COLUMNS * slice]
                //+ parity[(col + 1) % 9 + COLUMNS * ((slice + 1) % SLICES)];
                let sum_to_add = parity[(col + 8) % 9 + COLUMNS * slice]
                    + parity[(col + 1) % 9 + COLUMNS * ((slice + 1) % SLICES)];
                state[idx] = (state[idx] + sum_to_add) % 3;
            }
        }
    }
}

fn add_round_constant(state: &mut [Trit], round: usize) {
    for slice in 0..SLICES {
        for col in 0..COLUMNS {
            let idx = SLICE_SIZE * slice + col;
            state[idx] = (state[idx] + ROUND_CONSTANTS[round][slice * COLUMNS + col]) % 3;
        }
    }
}

fn troika_permutation(state: &mut [Trit], num_rounds: usize) {
    assert!(num_rounds <= NUM_ROUNDS);

    //print_troika_state(state);
    for round in 0..num_rounds {
        sub_trytes(state);
        shift_rows(state);
        shift_lanes(state);
        add_column_parity(state);
        add_round_constant(state, round);
    }
    //print_troika_state(state);
}

fn troika_absorb(
    state: &mut [Trit],
    rate: usize,
    message: &[Trit],
    message_length: usize,
    num_rounds: usize,
) {
    let mut message_offset = 0;
    let mut message_length = message_length;

    while message_length >= rate {
        // Copy message block over the state
        for trit_idx in 0..rate {
            //state[trit_idx] = TROIKA_TROIKAFY!(message[message_offset + trit_idx], Trit);
            state[trit_idx] = message[message_offset + trit_idx];
        }
        troika_permutation(state, num_rounds);
        message_length -= rate;
        message_offset += rate;
    }

    // Pad last block
    let mut last_block = vec![0u8; rate];

    // Copy over last incomplete message block
    let mut trit_idx = 0;
    for _ in 0..message_length {
        //last_block[trit_idx] = TROIKA_TROIKAFY!(message[trit_idx], Trit);
        last_block[trit_idx] = message[trit_idx];
        trit_idx += 1;
    }

    // Apply padding
    last_block[trit_idx] = PADDING;

    // Insert last message block
    for trit_idx in 0..rate {
        state[trit_idx] = last_block[trit_idx];
    }
}

fn troika_squeeze(
    hash: &mut [Trit],
    hash_length: usize,
    rate: usize,
    state: &mut [Trit],
    num_rounds: usize,
) {
    let mut hash_offset = 0;
    let mut hash_length = hash_length;

    while hash_length >= rate {
        troika_permutation(state, num_rounds);
        // Extract rate output
        for trit_idx in 0..rate {
            //hash[trit_idx] = TROIKA_IOTAFY(state[trit_idx], Trit);
            hash[hash_offset + trit_idx] = state[trit_idx];
        }
        hash_offset += rate;
        hash_length -= rate;
    }

    // Check if there is a last incomplete block
    if hash_length % rate == 0 {
        troika_permutation(state, num_rounds);
        for trit_idx in 0..hash_length {
            //hash[trit_idx] = TROIKA_IOTAFY(state[trit_idx], Trit);
            hash[trit_idx] = state[trit_idx];
        }
    }
}

#[cfg(test)]
mod troika_tests {
    use super::*;
    use bytes::BytesMut;

    const VECTOR1_HASH: [u8; 243] = [
        0, 0, 2, 0, 0, 0, 2, 0, 2, 1, 0, 2, 2, 2, 0, 2, 0, 1, 0, 0, 1, 2, 2, 0, 1, 1, 1, 0, 0, 1,
        1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 2, 2, 2, 1, 1, 2, 2, 1, 1, 0, 2, 1, 1, 0, 0, 2, 1, 1,
        0, 1, 2, 0, 2, 1, 0, 1, 1, 0, 1, 1, 0, 1, 2, 0, 1, 0, 1, 2, 0, 1, 2, 1, 0, 2, 0, 2, 0, 1,
        0, 1, 1, 1, 0, 0, 2, 2, 1, 1, 1, 0, 2, 0, 2, 2, 1, 2, 0, 0, 1, 2, 2, 2, 1, 0, 2, 0, 2, 0,
        2, 1, 0, 0, 2, 0, 0, 0, 2, 0, 1, 2, 2, 0, 0, 2, 1, 1, 2, 2, 0, 0, 2, 1, 2, 0, 2, 0, 0, 1,
        2, 0, 0, 1, 0, 1, 0, 2, 0, 1, 2, 2, 1, 2, 0, 0, 0, 1, 0, 1, 1, 2, 0, 1, 0, 1, 0, 2, 1, 1,
        2, 0, 0, 2, 1, 0, 0, 2, 1, 0, 2, 0, 0, 0, 0, 0, 2, 1, 0, 0, 1, 2, 0, 2, 0, 0, 1, 1, 2, 2,
        0, 0, 2, 2, 1, 0, 2, 2, 1, 1, 1, 0, 0, 2, 1, 1, 1, 0, 0, 0, 0, 0, 1, 2, 1, 2, 2, 2, 2, 0,
        0, 0, 2,
    ];

    const VECTOR2_HASH: [u8; 243] = [
        2, 0, 2, 0, 0, 2, 1, 1, 1, 1, 1, 0, 1, 2, 0, 0, 1, 1, 1, 0, 1, 2, 2, 1, 2, 2, 2, 1, 2, 0,
        0, 2, 2, 1, 1, 1, 0, 1, 2, 2, 0, 1, 2, 0, 2, 1, 2, 1, 2, 1, 2, 0, 1, 0, 0, 0, 0, 0, 1, 0,
        2, 0, 2, 0, 2, 1, 2, 2, 2, 0, 1, 0, 2, 1, 2, 1, 2, 1, 2, 1, 0, 2, 1, 0, 2, 0, 1, 1, 1, 2,
        2, 2, 1, 1, 1, 1, 0, 1, 0, 0, 0, 2, 1, 0, 0, 1, 2, 1, 1, 1, 0, 0, 0, 1, 1, 2, 1, 2, 1, 2,
        0, 0, 0, 2, 2, 2, 1, 2, 1, 2, 0, 2, 0, 0, 2, 2, 1, 0, 0, 0, 2, 2, 2, 0, 2, 2, 0, 2, 2, 2,
        2, 1, 0, 0, 2, 2, 1, 0, 1, 2, 1, 1, 2, 0, 0, 1, 1, 1, 2, 1, 2, 1, 0, 2, 2, 0, 1, 1, 2, 0,
        2, 2, 1, 1, 0, 2, 1, 1, 2, 0, 2, 0, 0, 1, 1, 1, 0, 2, 0, 0, 0, 0, 2, 1, 0, 1, 2, 2, 1, 1,
        0, 2, 2, 2, 1, 1, 0, 0, 2, 1, 1, 2, 2, 0, 0, 2, 1, 2, 0, 1, 2, 2, 1, 1, 2, 0, 2, 2, 1, 2,
        1, 1, 1,
    ];

    const VECTOR3_HASH: [u8; 243] = [
        1, 2, 0, 2, 2, 0, 1, 2, 1, 2, 1, 2, 0, 2, 0, 2, 1, 1, 0, 1, 2, 2, 0, 2, 2, 2, 1, 1, 2, 1,
        2, 1, 2, 2, 2, 1, 2, 1, 1, 0, 2, 2, 1, 1, 2, 2, 2, 2, 2, 0, 1, 2, 1, 2, 0, 0, 1, 2, 2, 1,
        0, 1, 1, 2, 0, 2, 2, 1, 1, 0, 2, 0, 0, 2, 0, 0, 0, 0, 2, 0, 0, 1, 0, 0, 0, 1, 2, 0, 2, 1,
        2, 2, 2, 0, 1, 1, 2, 1, 1, 1, 1, 1, 2, 0, 2, 2, 1, 0, 1, 0, 2, 2, 0, 2, 2, 1, 1, 1, 2, 0,
        1, 0, 2, 2, 1, 1, 2, 2, 2, 0, 0, 0, 0, 0, 2, 2, 1, 0, 2, 0, 2, 1, 2, 1, 0, 0, 1, 2, 2, 1,
        0, 1, 0, 0, 2, 2, 0, 0, 1, 1, 0, 1, 0, 2, 1, 0, 1, 0, 0, 0, 0, 0, 2, 1, 2, 2, 1, 0, 1, 1,
        2, 2, 0, 0, 0, 2, 1, 0, 0, 0, 1, 2, 2, 2, 1, 0, 2, 0, 0, 1, 0, 1, 1, 2, 0, 0, 1, 2, 2, 2,
        0, 2, 0, 1, 1, 2, 1, 0, 0, 2, 1, 1, 0, 2, 0, 2, 2, 1, 1, 2, 1, 1, 0, 1, 1, 0, 1, 1, 0, 2,
        2, 1, 2,
    ];

    #[test]
    fn vector1_test() {
        let trits = [0u8; 243];
        let mut trits = BytesMut::from(&trits[..]);
        troika(&mut trits, &[0]);

        assert_eq!(VECTOR1_HASH.len(), trits.len());
        (0..trits.len()).for_each(|i| assert_eq!(trits[i], VECTOR1_HASH[i]));
    }

    #[test]
    fn vector2_test() {
        let trits = [0u8; 243];
        let mut trits = BytesMut::from(&trits[..]);
        let input = &[0, 0];
        troika(&mut trits, input);

        assert_eq!(VECTOR2_HASH.len(), trits.len());
        (0..trits.len()).for_each(|i| assert_eq!(trits[i], VECTOR2_HASH[i]));
    }

    #[test]
    fn vector3_test() {
        let trits = [0u8; 243];
        let mut trits = BytesMut::from(&trits[..]);
        let input = &[
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
        ];
        troika(&mut trits, input);

        assert_eq!(VECTOR3_HASH.len(), trits.len());
        (0..trits.len()).for_each(|i| assert_eq!(trits[i], VECTOR3_HASH[i]));
    }
}
