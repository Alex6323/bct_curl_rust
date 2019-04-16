// hi lo trit
// 0  0   ?
// 0  1   -1
// 1  0   1
// 1  1   0

#[derive(Default)]
pub struct Trit64 {
    pub lo: u64,
    pub hi: u64,
}

#[derive(Default)]
pub struct Trit128 {
    pub lo: u128,
    pub hi: u128,
}

#[derive(Default)]
pub struct Trits64 {
    pub lo: Vec<u64>,
    pub hi: Vec<u64>,
}

impl Trits64 {
    pub fn from(lo: Vec<u64>, hi: Vec<u64>) -> Self {
        Trits64 { lo, hi }
    }
}

#[derive(Default)]
pub struct Trits128 {
    pub lo: Vec<u128>,
    pub hi: Vec<u128>,
}

impl Trits128 {
    pub fn from(lo: Vec<u128>, hi: Vec<u128>) -> Self {
        Trits128 { lo, hi }
    }
}

#[cfg(test)]
mod types_tests {
    use super::*;

    #[test]
    fn default_trit64() {
        let trit = Trit64::default();

        assert_eq!(0, trit.lo);
        assert_eq!(0, trit.hi);
    }

    #[test]
    fn default_trit128() {
        let trit = Trit128::default();

        assert_eq!(0, trit.lo);
        assert_eq!(0, trit.hi);
    }

    #[test]
    fn default_trits64() {
        let trits = Trits64::default();

        assert_eq!(0, trits.lo.len());
        assert_eq!(0, trits.hi.len());
    }

    #[test]
    fn default_trits128() {
        let trits = Trits128::default();

        assert_eq!(0, trits.lo.len());
        assert_eq!(0, trits.hi.len());
    }
}
