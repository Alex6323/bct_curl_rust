use super::types::Trit;

//uint16_t perm[729];
//uint16_t plutLeft[243];
//uint16_t plutRight[243];

/// Evaluates the Troika hash function on the input.
pub fn troika(out_buf: &mut [Trit], in_buf: &[Trit]) {}

/// Evaluates the Troika hash function on the input with a variable number of rounds of
/// the permutation.
pub fn troika_var_rounds(out_buf: &mut [Trit], in_buf: &[u8], num_rounds: usize) {
    let out_len = out_buf.len();
    let in_len = in_buf.len();
}

#[inline]
fn simd_export_value(nr: u32, t: &Trit) -> u8 {
    //#[cfg(not(simd_size_128))]
    let a = ((t.lo >> nr) & 0x1) | ((((t.lo ^ t.hi) >> nr) & 0x1) << 1);

    unimplemented!()
    //#[cfg(simd_size_128)]
    //return (_mm_movepi64_pi64(((t.lo >> nr) & 0x1) | ((((t.lo ^ t.hi) >> nr) & 0x1) <<
    // 1))[0] & 0xff) as u8;
}

fn sub_trytes(state: &mut [Trit]) {}

fn shift_rows(state: &mut [Trit]) {}

fn shift_lanes(state: &mut [Trit]) {}

fn add_column_parity(state: &mut [Trit]) {}

fn add_round_constant(state: &mut [Trit], round: usize) {}

fn troika_permutation(state: &mut [Trit], num_rounds: usize) {}
