use packed_simd::u64x8;

pub type SIMD_T = u64x8;

pub struct Trit {
	pub hi: SIMD_T ,
	pub lo: SIMD_T ,
}

pub type Tryte = u8; /* Stores 0,...,26 in a byte. */