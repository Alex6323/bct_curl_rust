use super::types::Trit;

pub const NUM_MAX_ROUNDS: usize = 24;
pub const TROIKA_RATE: usize = 243;

pub const COLUMNS: usize = 9;
pub const ROWS: usize = 3;
pub const SLICES: usize = 27;
pub const SLICESIZE: usize = COLUMNS * ROWS;
pub const STATE_SIZE: usize = COLUMNS * ROWS * SLICES;
pub const STATESIZE: usize = COLUMNS * ROWS * SLICES;
pub const RATESIZE: usize = SLICESIZE * 9;
pub const NUM_SBOXES: usize = COLUMNS * ROWS * SLICES / 3;
pub const MUXSIZE: usize = 8064;
pub const STRIT_SIZE: usize = 64;
pub const STRIT_BASE_SIZE: usize = 64;

pub const PADDING: Trit = 1;
