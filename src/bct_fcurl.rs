use crate::constants::*;
use scoped_pool::Pool;
use std::sync::atomic::{AtomicUsize, Ordering};

// TODO: process rest chunk
pub fn bct_fcurl_64_par(transactions: &Vec<Vec<i8>>, num_rounds: usize) -> Vec<[i8; HASH_LENGTH]> {
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
                    bct_fcurl_64(&transactions[offset..offset + chunk_length], num_rounds);
                // TODO: create output vector and actually return something
            })
        }
    });

    // TEMP
    vec![]
}

pub fn bct_fcurl_64(transactions: &[Vec<i8>], num_rounds: usize) -> Vec<[i8; HASH_LENGTH]> {
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

        let mut trits_lo = [0u64; TRANSACTION_TRIT_LENGTH];
        let mut trits_hi = [0u64; TRANSACTION_TRIT_LENGTH];

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

        // Run FCurl
        let mut state_lo = [HIGH_U64_BITS; STATE_LENGTH];
        let mut state_hi = [HIGH_U64_BITS; STATE_LENGTH];

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
    state_lo: &mut [u64; STATE_LENGTH],
    state_hi: &mut [u64; STATE_LENGTH],
    num_rounds: usize,
) {
    let mut scratch_lo = [0u64; STATE_LENGTH];
    let mut scratch_hi = [0u64; STATE_LENGTH];

    scratch_lo.copy_from_slice(state_lo);
    scratch_hi.copy_from_slice(state_hi);

    let mut s1_lo = state_lo.as_mut_ptr();
    let mut s1_hi = state_hi.as_mut_ptr();

    let mut s2_hi = scratch_hi.as_mut_ptr();
    let mut s2_lo = scratch_lo.as_mut_ptr();

    let mut lo: *mut u64;
    let mut hi: *mut u64;

    for _ in 0..num_rounds {
        let a = *s2_lo.offset(0);
        let b = *s2_hi.offset(0);
        let d = b ^ *s2_lo.offset(364);

        *s1_lo.offset(0) = !(d & a);
        *s1_hi.offset(0) = (a ^ *s2_hi.offset(364)) | d;

        for i in 0..364 {
            let (x, y, z) = (364 - i, 729 - (i + 1), 364 - (i + 1));
            //let (x, y, z) = (i + 1, i + 365, i);

            let a = *s2_lo.offset(x);
            let b = *s2_hi.offset(x);
            let d = b ^ (*s2_lo.offset(y));

            *s1_lo.offset(2 * i + 1) = !(d & a);
            *s1_hi.offset(2 * i + 1) = (a ^ *s2_hi.offset(y)) | d;
            //*s1_lo.offset(2 * (363 - i) + 1) = !(d & a);
            //*s1_hi.offset(2 * (363 - i) + 1) = (a ^ *s2_hi.offset(y)) | d;

            let a = *s2_lo.offset(y);
            let b = *s2_hi.offset(y);
            let d = b ^ (*s2_lo.offset(z));

            *s1_lo.offset(2 * i + 2) = !(d & a);
            *s1_hi.offset(2 * i + 2) = (a ^ *s2_hi.offset(z)) | d;
            //*s1_lo.offset(2 * (363 - i) + 2) = !(d & a);
            //*s1_hi.offset(2 * (363 - i) + 2) = (a ^ *s2_hi.offset(z)) | d;
        }

        lo = s1_lo;
        hi = s1_hi;

        s1_lo = s2_lo;
        s1_hi = s2_hi;

        s2_lo = lo;
        s2_hi = hi;
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

    #[test]
    fn bct_fcurl_works() {
        let mut transactions = vec![];
        let tx_trits = from_tryte_string(MAINNET_TRYTES_1);
        transactions.push(tx_trits);

        let hash_trits64 = bct_fcurl_64(&transactions, NUM_ROUNDS);
        let hash_trits1 = hash_trits64[0];

        //println!("{:?}", hash_trits1);
        //println!("{}", hash_trits1.len());

        let tryte_string1 = string_from_trytes(&trytes_from_trits(&hash_trits1));

        println!("{}", tryte_string1);

        assert_eq!(MAINNET_HASH_1, tryte_string1);
    }

}
