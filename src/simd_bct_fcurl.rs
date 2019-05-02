use std::sync::atomic::{AtomicUsize, Ordering};

use scoped_pool::Pool;
use simdeez::avx2::*;
use simdeez::scalar::*;
use simdeez::sse2::*;
use simdeez::sse41::*;
use simdeez::*;

use crate::constants::*;

pub fn simd_bct_fcurl_64(transactions: &[Vec<i8>], num_rounds: usize) -> Vec<[i8; HASH_LENGTH]> {
    let mut offset = 0;
    let mut length = transactions.len();
    let mut hashes = vec![[0i8; HASH_LENGTH]; length];

    loop {
        let chunk_size = {
            if length < MAX_BATCH_SIZE_64 {
                length
            } else {
                MAX_BATCH_SIZE_64
            }
        };

        let mut trits_lo = [0i64; TRANSACTION_TRIT_LENGTH];
        let mut trits_hi = [0i64; TRANSACTION_TRIT_LENGTH];

        // Interlace transactions
        for i in 0..TRANSACTION_TRIT_LENGTH {
            for j in 0..chunk_size {
                match transactions[j][i] {
                    0 => {
                        trits_lo[i] |= 0x1 << j;
                        trits_hi[i] |= 0x1 << j;
                    }
                    1 => trits_hi[i] |= 0x1 << j,
                    -1 => trits_lo[i] |= 0x1 << j,
                    _ => panic!("unexpected trit value"),
                }
            }
        }

        // Run SIMD-FCurl
        let mut state_lo = [HIGH_I64_BITS; STATE_LENGTH];
        let mut state_hi = [HIGH_I64_BITS; STATE_LENGTH];

        for i in (0..TRANSACTION_TRIT_LENGTH).step_by(HASH_LENGTH) {
            state_lo[0..HASH_LENGTH].copy_from_slice(&trits_lo[i..i + HASH_LENGTH]);
            state_hi[0..HASH_LENGTH].copy_from_slice(&trits_hi[i..i + HASH_LENGTH]);
            unsafe {
                transform(&mut state_lo, &mut state_hi, num_rounds);
            }
        }

        // Deinterlace transactions
        for i in 0..HASH_LENGTH {
            for j in 0..chunk_size {
                let lo = (state_lo[i] >> j) & 0x1;
                let hi = (state_hi[i] >> j) & 0x1;

                match (lo, hi) {
                    (1, 0) => hashes[offset + j][i] = -1,
                    (0, 1) => hashes[offset + j][i] = 1,
                    (_, _) => hashes[offset + j][i] = 0,
                }
            }
        }
        offset += chunk_size;

        if length > chunk_size {
            length -= chunk_size;
        } else {
            break;
        }
    }
    hashes
}

#[inline]
unsafe fn transform(
    state_lo: &mut [i64; STATE_LENGTH],
    state_hi: &mut [i64; STATE_LENGTH],
    num_rounds: usize,
) {
    let mut scratch_lo = [0i64; STATE_LENGTH];
    let mut scratch_hi = [0i64; STATE_LENGTH];
    scratch_lo.copy_from_slice(state_lo);
    scratch_hi.copy_from_slice(state_hi);

    let mut s1_lo = state_lo.as_mut_ptr();
    let mut s1_hi = state_hi.as_mut_ptr();
    let mut s2_hi = scratch_hi.as_mut_ptr();
    let mut s2_lo = scratch_lo.as_mut_ptr();

    let mut lo: *mut i64;
    let mut hi: *mut i64;

    for _ in 0..num_rounds {
        let a = *s2_lo.offset(0);
        let b = *s2_hi.offset(0);
        let d = b ^ *s2_lo.offset(364);
        *s1_lo.offset(0) = !(d & a);
        *s1_hi.offset(0) = (a ^ *s2_hi.offset(364)) | d;

        inner_loop_runtime_select(s2_lo, s2_hi, s1_lo, s1_hi);

        // Swap scratchpad and state
        lo = s1_lo;
        hi = s1_hi;
        s1_lo = s2_lo;
        s1_hi = s2_hi;
        s2_lo = lo;
        s2_hi = hi;
    }
}

// Rust implementation of this C-macro for in-place swapping:
// #define SWAP(a, b) (((a) ^= (b)), ((b) ^= (a)), ((a) ^= (b)))
macro_rules! swap {
    ($a:expr, $b:expr) => {
        $a ^= $b;
        $b ^= $a;
        $a ^= $b;
    };
}

// NOTE: currently doesn't produce the correct hash, or in other words: it doesn't work
// The swap macro invocations align the result data, so the compiler can copy larger
// chunks back into the state. Unfortunately it had no effect performance-wise :(
simd_runtime_generate!(
    fn inner_loop(s2_lo: *const i64, s2_hi: *const i64, s1_lo: *mut i64, s1_hi: *mut i64) {
        let step = S::VI64_WIDTH;

        let mut er: Vec<i64> = Vec::with_capacity(2 * step);
        let mut fr: Vec<i64> = Vec::with_capacity(2 * step);

        er.set_len(2 * step);
        fr.set_len(2 * step);

        // Inner loop
        for i in (0..364).step_by(step) {
            //let (x, y, z) = (364 - i, 729 - (i + 1), 364 - (i + 1));
            let (x, y, z) = (i + 1, i + 365, i);

            // Load SIMD vectors
            let a = S::loadu_epi64(&*s2_lo.offset(x));
            let b = S::loadu_epi64(&*s2_hi.offset(x));
            let c = S::loadu_epi64(&*s2_lo.offset(y));
            let k = S::loadu_epi64(&*s2_hi.offset(y));
            let p = S::loadu_epi64(&*s2_lo.offset(z));
            let q = S::loadu_epi64(&*s2_hi.offset(z));

            // Perform SIMD operations
            let d = b ^ c;
            let e = !(d & a);
            let f = (a ^ k) | d;
            let m = k ^ p;
            let n = !(m & c);
            let o = (c ^ q) | m;

            // Store SIMD vectors
            S::storeu_epi64(&mut er[0], e);
            S::storeu_epi64(&mut er[step], n);
            S::storeu_epi64(&mut fr[0], f);
            S::storeu_epi64(&mut fr[step], o);

            // Align data so it can be copied into the state as a contiguous chunk
            swap!(er[1], er[4]);
            swap!(fr[1], fr[4]);
            swap!(er[2], er[4]);
            swap!(fr[2], fr[4]);
            swap!(er[3], er[5]);
            swap!(fr[3], fr[5]);
            swap!(er[6], er[5]);
            swap!(fr[6], fr[5]);

            // Write back to state
            for k in 0..(2 * step) {
                *s1_lo.offset((363 - i) + k as isize + 1) = er[2 * step - k - 1];
                *s1_hi.offset((363 - i) + k as isize + 1) = fr[2 * step - k - 1];
            }
        }
    }
);

// Just for comparision (this is scalar code), produces the correct hash
unsafe fn inner_loop_basic(s2_lo: *const i64, s2_hi: *const i64, s1_lo: *mut i64, s1_hi: *mut i64) {
    for i in 0..364 {
        let (x, y, z) = (i + 1, i + 365, i);

        let a = *s2_lo.offset(x);
        let b = *s2_hi.offset(x);
        let c = *s2_lo.offset(y);
        let k = *s2_hi.offset(y);
        let p = *s2_lo.offset(z);
        let q = *s2_hi.offset(z);

        let d = b ^ c;
        let e = !(d & a);
        let f = (a ^ k) | d;
        let m = k ^ p;
        let n = !(m & c);
        let o = (c ^ q) | m;

        *s1_lo.offset(2 * (363 - i) + 1) = e;
        *s1_lo.offset(2 * (363 - i) + 2) = n;
        *s1_hi.offset(2 * (363 - i) + 1) = f;
        *s1_hi.offset(2 * (363 - i) + 2) = o;
    }
}

// Just for comparision (manually vectorized code); produces the correct hash
unsafe fn inner_loop_u64x4(s2_lo: *const i64, s2_hi: *const i64, s1_lo: *mut i64, s1_hi: *mut i64) {
    //
    for i in (0..364).step_by(4) {
        let (x, y, z) = (i + 1, i + 365, i);

        let a = i64x4::new(s2_lo.offset(x));
        let b = i64x4::new(s2_hi.offset(x));
        let c = i64x4::new(s2_lo.offset(y));
        let k = i64x4::new(s2_hi.offset(y));
        let p = i64x4::new(s2_lo.offset(z));
        let q = i64x4::new(s2_hi.offset(z));

        let d = b.xor(&c);
        let e = d.and(&a).not();
        let f = a.xor(&k).or(&d);
        let m = k.xor(&p);
        let n = m.and(&c).not();
        let o = c.xor(&q).or(&m);

        *s1_lo.offset(2 * (363 - i) + 1) = e.0;
        *s1_lo.offset(2 * (363 - i) + 2) = n.0;
        *s1_hi.offset(2 * (363 - i) + 1) = f.0;
        *s1_hi.offset(2 * (363 - i) + 2) = o.0;

        *s1_lo.offset(2 * (362 - i) + 1) = e.1;
        *s1_lo.offset(2 * (362 - i) + 2) = n.1;
        *s1_hi.offset(2 * (362 - i) + 1) = f.1;
        *s1_hi.offset(2 * (362 - i) + 2) = o.1;

        *s1_lo.offset(2 * (361 - i) + 1) = e.2;
        *s1_lo.offset(2 * (361 - i) + 2) = n.2;
        *s1_hi.offset(2 * (361 - i) + 1) = f.2;
        *s1_hi.offset(2 * (361 - i) + 2) = o.2;

        *s1_lo.offset(2 * (360 - i) + 1) = e.3;
        *s1_lo.offset(2 * (360 - i) + 2) = n.3;
        *s1_hi.offset(2 * (360 - i) + 1) = f.3;
        *s1_hi.offset(2 * (360 - i) + 2) = o.3;
    }
}

// TODO: process rest chunk
pub fn simd_bct_fcurl_64_par(
    transactions: &Vec<Vec<i8>>,
    num_rounds: usize,
) -> Vec<[i8; HASH_LENGTH]> {
    let num_threads = num_cpus::get();
    let pool = Pool::new(num_threads);
    let chunk_length = transactions.len() / num_threads;
    let index = AtomicUsize::new(0);

    pool.scoped(|scope| {
        for _ in 0..num_threads {
            scope.execute(|| {
                let i = index.fetch_add(1, Ordering::SeqCst);
                //
                let offset = i * chunk_length;
                let hash_trits =
                    simd_bct_fcurl_64(&transactions[offset..offset + chunk_length], num_rounds);
                // TODO: create output vector and actually return something
            })
        }
    });

    // TEMP
    vec![]
}

// TEMP: just one step before using actual SIMD type
struct i64x4(i64, i64, i64, i64);

impl i64x4 {
    pub unsafe fn new(p: *const i64) -> Self {
        i64x4(*p.offset(0), *p.offset(1), *p.offset(2), *p.offset(3))
    }

    pub fn not(&self) -> Self {
        i64x4(!self.0, !self.1, !self.2, !self.3)
    }

    pub fn xor(&self, right: &Self) -> Self {
        i64x4(
            self.0 ^ right.0,
            self.1 ^ right.1,
            self.2 ^ right.2,
            self.3 ^ right.3,
        )
    }

    pub fn and(&self, right: &Self) -> Self {
        i64x4(
            self.0 & right.0,
            self.1 & right.1,
            self.2 & right.2,
            self.3 & right.3,
        )
    }

    pub fn or(&self, right: &Self) -> Self {
        i64x4(
            self.0 | right.0,
            self.1 | right.1,
            self.2 | right.2,
            self.3 | right.3,
        )
    }
}

#[cfg(test)]
mod bct_fcurl_tests {
    use super::*;
    use crate::constants::*;
    use crate::convert::*;

    const MAINNET_TRYTES_1: &str = "TLFCFY9IMZVINTAZRCUWTKAFENIBIFOGKWDZQIRTYSVVHTSIIZZ9RLUYVTLXEHACXIUFJJQNFRJYMGGYDWOBNMTPFE9CGVVTREVUJKIXRHSOPFAXMNEMHEW9ZE9HVFEDEORKWGLNECZ9MXLDHPBAOMO9ZMSZJCZLAWWZKOLHBASHYNMCBCPZOXOLLVMFZVCTUDQZSIUSITRDHHXGAOVTOMSKDTZXLSCNHNXJNVGOTZPJDRHOBUAPIAIGLCETVDWSOPEKAOWBNUIEUTTLPFQLRYVRJQJOCBVOZEK9TQMJQUPEZKLHIVMO9TRIUBQNXJYIXFUWFUYWDIIDBQXRYULR9RXPSLTRFY9IIMQBLGOXUZJAKFSEJCSTYP9SWRFCNTMDMRFFWCVZTNFYLFZISPCQ99OSTMJBNLYCQLKWETRLJEOEBJZBO9ZUZMGQIRCCLBANSVYABGKMQCKWIWHHH9FGKGIURCJDKTIQBFENQCYWAX9WHNQ9OKGIWILNFJGMERJNBHDPNFCASDKZLOXLALOSMUFXYKKCDKWVX9PBOVXMAICVTHBLPWPFWJWYBTLJLXNOHREEFTJDLTYPPFMZ9MTPBHNXQL9MXRLGMKRN9EJYZMDZEOZOMKVYWKORKIBKDYZTCPOHYIADIVJWCHRVWCE9DSSHIEEINJYQWBFBTOCZOBL9LLFKWIFAJT9ZQKEUZTBARTEYUBYQOKMRMKWLTJOPVKIDXIUWQFLVWBTAYNOREZSCKAGRGVRLQUBUGKKHLL9YBFMGUMNSUMAXMCRQOQHBYJJRBMFQIUPZEBXFMHYJMAMAHUMMBLRDPBIOMJ9OCHBSBIFX9YSXPPVDMUCICHCSYRWUXXUEROHXGGEJBFJE9S9QGAQ9YOPIZOKGXRXMMFBLGVMC9QXJZTI99TATFJDJORMGJPAQGQICFHYAMWEUKWYYKIGTWYPNC9ZPQEKWAOZVCBIPZUTZUKJXFPWTQUKWIYJBULBJEJZGYEHVYUHFROLQYYPI9WCXHHWEITITPTXMTBWLJRAYV9LZK9FVGBOQRSWEFRMWBKBHAYWETHDTAAPOPPHFOX9PYQAXDVMWXGW9HDTLSINGRWGODCBNVXXYVDKJ9OROIZAULXMZUEVSDPWUJC9FEQAWMDOI9TALZAHX9ZHYSQEJOSZTHZPKWMZBTWUKNJUJNTZRWEYVWUAXVEP9NSZVYHLHZWDDTCQQTCDHTQPZXTM9ERHNNEORYBUKIRJPZORWXJDRRURZCBYLMFZKSZZVJIWXBXSKJMKUAFYKRQKVIGJJGYLXKFWZEIU9JJXRQSOFDLGXELTVBXKPDLKRLJTGVOD9QGIVVWS9EZAMBPDIEABEJJKTYQZVOD9TIGXPDJGJBRLHXCKKFFVQXFPQNKLMOMOJUDNFZCYEP9CQVNQKRYLCMCFNM9JIE9XUCDBX9ABNHZTSRROFYZCXDRLRBMYYRWUEWHC9QGGHBIQVBISISOZWXGXKQWSOASERXWNQXHWUGXDKIVDDWZZIRIERRSEOMEREYYCO9QIXKQOZQZALPBNQCBJWPV9BYDGYTDJPHXFZQ9CQZIDZTORKIABS9LFWOPWISFESVOTWIBTGDFIZBDOAJO9DJVAIQVUYEAWPRETXYWFMMUUUEUMWPGTWEUSZHJUCYGZDCSGVZGNTJBWGHGYZEOTOVIYAODKWJJLJFZGIKVGUYXRGAFMOFDM9SHSWVSDKAJGEVCORATXJHEGLYTVCGCTXZVUFVLZ9CYFCA9MM9STIZHKTGYJUACFVEGSZYJBNRWTRO9JUWZWOSPGJYIRTQSD9EPHONGYDWUQXYRHGXUSVGIAPVGOLLFQTQOYSOMHAOCNVKLPGRKIEVZGCFVWLTBEMM9QMUML9RVYCMOFIUCNTTALZKSGIPVNLFUGDPTHVGKDUIOZMKAEPYSYZTNFTMWJY99VGIM9YHI9WIVVJAANTHPKT9HOWWZSYRDMVJCSKASOZOOPAUOMMSOWNUTTGREQWPQDKRGGSODHKPFUIXKLVDFJSOQH9ZYMREQNXHHPOEISKPGTNIEBKV9SEFTKZZZVXQAYFPYTDMJVUULL9YNMITHTRB9GKILOFJCCYXKMPIYNNOXTVNLDKTODGEADIRIUXHNGVAAIEFYG9BE9BRNAZUABPF9BVODCZGPXBLBVJIXYLLYDVDUKVYGIWETMSKYXGYMXSXGKPDZMG9NOFIMSKFKIHTQSAVGIWERREF9MEAOCDE99FXRR9FDCKOZOJBTOZEVLLCASBONUMPDVD9XWSHEGZ9999999999999999999999999999999999999999999999VPRPPZD99A99999999J99999999KOJZIA9PSFRKG9ZUOJO9PGDIEFPGPSDKVPVBSXDIOOXAPZHKLJHEULIJKYRTDXOJKTRFYYSABGTBRKVCBBZZSWTVHQSQGJKQAHLINBNNLFTQERSITF9BAJCODBNLLQEQZETPQBGWFYCOBUARDAGTCGQCGOUBLA9999QPBMLSSKBO9ILX9QKYCAXNHLK9KFUJYO99GOO99VYROHOVXACRKYPFVY9JRSHJIKFGBHOCXQFPMZZ9999999999999999999999999999999HKJSFUCME999999999MMMMMMMMMCVMNOI9PFCHLRVXSUEOCRLTRMUF";
    const MAINNET_HASH_1: &str =
        "MGPBAHYHKSQMMXXONAOOEDQS9RFEKMOOJUCGXSFYLXBHQFWIHMJGFJWDSZTGKHNBCSENCXSPQOSZ99999";

    // Deactivated for now because reasons
    //#[test]
    fn simd_bct_fcurl_works() {
        let mut transactions = vec![];
        let tx_trits = from_tryte_string(MAINNET_TRYTES_1);
        transactions.push(tx_trits);

        let hash_trits64 = simd_bct_fcurl_64(&transactions, NUM_ROUNDS);
        let hash_trits1 = hash_trits64[0];

        //println!("{:?}", hash_trits1);
        //println!("{}", hash_trits1.len());

        let tryte_string1 = string_from_trytes(&trytes_from_trits(&hash_trits1));

        println!("{}", tryte_string1);

        assert_eq!(MAINNET_HASH_1, tryte_string1);
    }

}
