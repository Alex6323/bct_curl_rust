use super::types::Trit;

pub const NUM_ROUNDS: usize = 24;
pub const TROIKA_RATE: usize = 243;

pub const COLUMNS: usize = 9;
pub const ROWS: usize = 3;
pub const SLICES: usize = 27;

pub const SLICE_SIZE: usize = COLUMNS * ROWS;
pub const STATE_SIZE: usize = COLUMNS * ROWS * SLICES;
pub const NUM_SBOXES: usize = STATE_SIZE / 3;

pub const PADDING: Trit = 0x1;
