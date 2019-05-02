use std::hash::Hasher;
use twox_hash::XxHash;

pub fn xxhash(trits: &[i8]) -> u64 {
    let mut xxhash = XxHash::default();
    trits.iter().for_each(|trit| {
        xxhash.write_i8(*trit);
    });
    xxhash.finish()
}
