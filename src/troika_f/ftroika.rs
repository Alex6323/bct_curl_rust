/*
use super::constants::*;
use super::types::{
    Trit,
    Tryte,
    T27,
};

pub fn ftroika(out_buf: &mut [Trit], in_buf: &[Trit]) {
    ftroika_var_rounds(out_buf, in_buf, NUM_ROUNDS)
}

pub fn ftroika_var_rounds(out_buf: &mut [Trit], in_buf: &[Trit], num_rounds: usize) {
    let out_len = out_buf.len();
    let in_len = in_buf.len();
    let mut state = [0u8; STATE_SIZE];

    ftroika_absorb(&mut state, TROIKA_RATE, in_buf, in_len, num_rounds);
    ftroika_squeeze(out_buf, out_len, TROIKA_RATE, &mut state, num_rounds);
}

pub fn ftroika(trit_t *out, unsigned long long outlen, const trit_t *in, unsigned long long inlen);

pub fn ftroika_var_rounds(trit_t *out, unsigned long long outlen, const trit_t *in, unsigned long long inlen,
                        unsigned long long num_rounds);

fn ftroika_sub_trytes(state: &mut [T27]) {
    for col in 0..COLUMNS {
        let x = T27 {
            p: A | E | F | J | M | P | S | U | Z,
            n: B | C | G | K | N | Q | T | V | X,
        };

        let y = T27 {
            p: C | F | M | Q | R | S | T | W | Y,
            n: ZERO | A | B | D | H | N | P | U | X,
        };

        let z = T27 {
            p: B | D | E | L | P | Q | S | V | W,
            n: A | G | H | M | N | O | T | Y | Z,
        };

        state[ROWS * col + 2] = t27_clean(x);
        state[ROWS * col + 1] = t27_clean(y);
        state[ROWS * col] = t27_clean(z);
    }
}

fn ftroika_shift_rows(state: &mut [T27]) {}

fn ftroika_shift_lanes(state: &mut [T27]) {}

fn ftroika_add_column_parity(state: &mut [T27]) {}

fn ftroika_print_round_constants() {}

fn ftroika_add_round_constant(state: &mut [T27], round: usize) {}

fn ftroika_permutation(state: &mut [T27], num_rounds: usize) {}

fn ftroika_nullify_state(state: &mut [T27]) {}

fn ftroika_nullify_rate(state: &mut [T27]) {}

fn ftroika_nullify_capacity(state: &mut [T27]) {}

fn ftroika_trits_to_rate(state: &mut [T27], trits: &[Trit], len: usize) {}

fn ftroika_rate_to_trits(state: &[T27], trits: &[Trit], len: usize) {}

fn ftroika_trytes_to_state(state: &mut [T27], trytes: &[Tryte], len: usize) -> usize {
    unimplemented!()
}

fn ftroika_compare_states(state: &mut [T27], other: &mut [T27]) -> usize {
    unimplemented!()
}

fn ftroika_increase_state(state: &mut [T27]) {}

fn ftroika_absorb(
    state: &mut [T27],
    rate: usize,
    message: &[Trit],
    message_length: usize,
    num_rounds: usize,
) {

}

fn ftroika_squeeze(
    hash: &mut [Trit],
    hash_length: usize,
    rate: usize,
    state: &mut [T27],
    num_rounds: usize,
) {

}

fn ftroika243_repeated(out_buf: &mut [Trit], in_buf: &[Trit], repeat: usize) {}

#[cfg(test)]
mod ftroika_tests {
    use super::*;
    use bytes::BytesMut;

    const VECTOR1_HASH: [u8; 243] = [
        0, 0, 2, 0, 0, 0, 2, 0, 2, 1, 0, 2, 2, 2, 0, 2, 0, 1, 0, 0, 1, 2, 2, 0, 1, 1, 1,
        0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 2, 0, 2, 2, 2, 1, 1, 2, 2, 1, 1, 0, 2, 1,
        1, 0, 0, 2, 1, 1, 0, 1, 2, 0, 2, 1, 0, 1, 1, 0, 1, 1, 0, 1, 2, 0, 1, 0, 1, 2, 0,
        1, 2, 1, 0, 2, 0, 2, 0, 1, 0, 1, 1, 1, 0, 0, 2, 2, 1, 1, 1, 0, 2, 0, 2, 2, 1, 2,
        0, 0, 1, 2, 2, 2, 1, 0, 2, 0, 2, 0, 2, 1, 0, 0, 2, 0, 0, 0, 2, 0, 1, 2, 2, 0, 0,
        2, 1, 1, 2, 2, 0, 0, 2, 1, 2, 0, 2, 0, 0, 1, 2, 0, 0, 1, 0, 1, 0, 2, 0, 1, 2, 2,
        1, 2, 0, 0, 0, 1, 0, 1, 1, 2, 0, 1, 0, 1, 0, 2, 1, 1, 2, 0, 0, 2, 1, 0, 0, 2, 1,
        0, 2, 0, 0, 0, 0, 0, 2, 1, 0, 0, 1, 2, 0, 2, 0, 0, 1, 1, 2, 2, 0, 0, 2, 2, 1, 0,
        2, 2, 1, 1, 1, 0, 0, 2, 1, 1, 1, 0, 0, 0, 0, 0, 1, 2, 1, 2, 2, 2, 2, 0, 0, 0, 2,
    ];

    const VECTOR2_HASH: [u8; 243] = [
        2, 0, 2, 0, 0, 2, 1, 1, 1, 1, 1, 0, 1, 2, 0, 0, 1, 1, 1, 0, 1, 2, 2, 1, 2, 2, 2,
        1, 2, 0, 0, 2, 2, 1, 1, 1, 0, 1, 2, 2, 0, 1, 2, 0, 2, 1, 2, 1, 2, 1, 2, 0, 1, 0,
        0, 0, 0, 0, 1, 0, 2, 0, 2, 0, 2, 1, 2, 2, 2, 0, 1, 0, 2, 1, 2, 1, 2, 1, 2, 1, 0,
        2, 1, 0, 2, 0, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 0, 1, 0, 0, 0, 2, 1, 0, 0, 1, 2, 1,
        1, 1, 0, 0, 0, 1, 1, 2, 1, 2, 1, 2, 0, 0, 0, 2, 2, 2, 1, 2, 1, 2, 0, 2, 0, 0, 2,
        2, 1, 0, 0, 0, 2, 2, 2, 0, 2, 2, 0, 2, 2, 2, 2, 1, 0, 0, 2, 2, 1, 0, 1, 2, 1, 1,
        2, 0, 0, 1, 1, 1, 2, 1, 2, 1, 0, 2, 2, 0, 1, 1, 2, 0, 2, 2, 1, 1, 0, 2, 1, 1, 2,
        0, 2, 0, 0, 1, 1, 1, 0, 2, 0, 0, 0, 0, 2, 1, 0, 1, 2, 2, 1, 1, 0, 2, 2, 2, 1, 1,
        0, 0, 2, 1, 1, 2, 2, 0, 0, 2, 1, 2, 0, 1, 2, 2, 1, 1, 2, 0, 2, 2, 1, 2, 1, 1, 1,
    ];

    const VECTOR3_HASH: [u8; 243] = [
        1, 2, 0, 2, 2, 0, 1, 2, 1, 2, 1, 2, 0, 2, 0, 2, 1, 1, 0, 1, 2, 2, 0, 2, 2, 2, 1,
        1, 2, 1, 2, 1, 2, 2, 2, 1, 2, 1, 1, 0, 2, 2, 1, 1, 2, 2, 2, 2, 2, 0, 1, 2, 1, 2,
        0, 0, 1, 2, 2, 1, 0, 1, 1, 2, 0, 2, 2, 1, 1, 0, 2, 0, 0, 2, 0, 0, 0, 0, 2, 0, 0,
        1, 0, 0, 0, 1, 2, 0, 2, 1, 2, 2, 2, 0, 1, 1, 2, 1, 1, 1, 1, 1, 2, 0, 2, 2, 1, 0,
        1, 0, 2, 2, 0, 2, 2, 1, 1, 1, 2, 0, 1, 0, 2, 2, 1, 1, 2, 2, 2, 0, 0, 0, 0, 0, 2,
        2, 1, 0, 2, 0, 2, 1, 2, 1, 0, 0, 1, 2, 2, 1, 0, 1, 0, 0, 2, 2, 0, 0, 1, 1, 0, 1,
        0, 2, 1, 0, 1, 0, 0, 0, 0, 0, 2, 1, 2, 2, 1, 0, 1, 1, 2, 2, 0, 0, 0, 2, 1, 0, 0,
        0, 1, 2, 2, 2, 1, 0, 2, 0, 0, 1, 0, 1, 1, 2, 0, 0, 1, 2, 2, 2, 0, 2, 0, 1, 1, 2,
        1, 0, 0, 2, 1, 1, 0, 2, 0, 2, 2, 1, 1, 2, 1, 1, 0, 1, 1, 0, 1, 1, 0, 2, 2, 1, 2,
    ];

    #[test]
    fn vector1_test() {
        let trits = [0u8; 243];
        let mut trits = BytesMut::from(&trits[..]);
        ftroika(&mut trits, &[0]);

        assert_eq!(VECTOR1_HASH.len(), trits.len());
        (0..trits.len()).for_each(|i| assert_eq!(trits[i], VECTOR1_HASH[i]));
    }

    #[test]
    fn vector2_test() {
        let trits = [0u8; 243];
        let mut trits = BytesMut::from(&trits[..]);
        let input = &[0, 0];
        ftroika(&mut trits, input);

        assert_eq!(VECTOR2_HASH.len(), trits.len());
        (0..trits.len()).for_each(|i| assert_eq!(trits[i], VECTOR2_HASH[i]));
    }

    #[test]
    fn vector3_test() {
        let trits = [0u8; 243];
        let mut trits = BytesMut::from(&trits[..]);
        let input = &[
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 2,
        ];
        ftroika(&mut trits, input);

        assert_eq!(VECTOR3_HASH.len(), trits.len());
        (0..trits.len()).for_each(|i| assert_eq!(trits[i], VECTOR3_HASH[i]));
    }
}
*/
