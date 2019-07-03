/*
// trits are stored as two bits, one inside p, the other inside n, both have the
// same 'index' names 'p' (positive) and 'n' (negative) result from using these
// functions in a lib for balanced ternary before so i just left them here
pub struct T27 {
    p: u32,
    n: u32,
}

pub type Trit = u8;
pub type Tryte = u8;

#[inline]
pub fn t27_clean(a: T27) -> T27 {
    T27 {
        p: a.p & 0x07ff_ffff,
        n: a.n & 0x07ff_ffff,
    }
}
*/
