/// Batches up to 64 trit arrays.
pub struct Batch64Hasher {
    hash_length: usize,
    num_rounds: usize,
}

impl Batch64Hasher {
    pub fn from(hash_length: usize, num_rounds: usize) -> Self {
        Batch64Hasher {
            hash_length,
            num_rounds,
        }
        //this.reqQueue = new ArrayBlockingQueue<>(MAX_BATCH_SIZE * 2);
    }
}

/// Batches up to 128 trit arrays.
pub struct Batch128Hasher {
    hash_length: usize,
    num_rounds: usize,
}
