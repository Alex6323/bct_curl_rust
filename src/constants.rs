// +------------- GENERAL ----------------
pub const HASH_LENGTH: usize = 243;
pub const STATE_LENGTH: usize = 3 * HASH_LENGTH;
pub const TRANSACTION_TRIT_LENGTH: usize = 8019;
pub const NUM_HASHES: usize = 5_000;

// +-------------- CURL ----------------
pub const NUM_CURL_ROUNDS: usize = 81;
pub const TRUTH_TABLE: [i8; 11] = [1, 0, -1, 2, 1, -1, 0, 2, -1, 1, 0];

// +------------- TROIKA ----------------
pub const NUM_TROIKA_ROUNDS: usize = 24;

// +-------------- BCT ----------------
pub const MAX_BATCH_SIZE_64: usize = 64;
pub const MAX_BATCH_SIZE_128: usize = 128;
pub const HIGH_U64_BITS: u64 = 0xFFFF_FFFF_FFFF_FFFF;
pub const HIGH_I64_BITS: i64 = -1;
pub const HIGH_U128_BITS: u128 = 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF;
