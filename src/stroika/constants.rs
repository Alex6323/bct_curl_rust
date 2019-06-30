pub const NUM_ROUNDS: usize =  24;
pub const TROIKA_RATE: usize = 243;

pub const COLUMNS: usize = 9;
pub const ROWS: usize =  3;
pub const SLICES: usize =  27;
pub const SLICE_SIZE: usize = COLUMNS*ROWS;
pub const STATE_SIZE: usize = COLUMNS*ROWS*SLICES;
pub const NUM_SBOXES: usize = SLICES*ROWS*COLUMNS/3;

//#define PADDING 0x1

/*
#[cfg(simd_size_128)]
pub const PADDING: Trit = Trit {~0, ~0, ~0, ~0};

#[cfg(simd_size_64)]
pub const PADDING: Trit = Trit {~0ul, ~0ul};

#[cfg(simd_size_32)]
pub const PADDING: Trit = Trit { hi: ~0, lo: ~0};
*/
