use crate::{bct_types::*, constants::*};

pub struct BCTCurl64 {
    hash_length: usize,
    num_rounds: usize,
    state_length: usize,
    state: Trits64,
}

impl BCTCurl64 {
    pub fn new(hash_length: usize, num_rounds: usize) -> Self {
        BCTCurl64 {
            hash_length,
            num_rounds,
            state_length: 3 * hash_length,
            state: Trits64::from(
                vec![HIGH_U64_BITS; 3 * hash_length],
                vec![HIGH_U64_BITS; 3 * hash_length],
            ),
        }
    }

    pub fn reset(&mut self) {
        for i in 0..self.state_length {
            self.state.lo[i] = HIGH_U64_BITS;
            self.state.hi[i] = HIGH_U64_BITS;
        }
    }

    // * Determines number of inputted trits
    // * loops through the input trits in chunks of '243', copies a chunk into the state and transforms it.
    pub fn absorb(&mut self, trits: Trits64) {
        assert_eq!(trits.lo.len(), trits.hi.len());

        let mut length = trits.lo.len();
        let mut offset = 0;

        loop {
            let chunk_length = {
                if length < self.hash_length {
                    length
                } else {
                    self.hash_length
                }
            };

            self.state.lo[0..chunk_length]
                .copy_from_slice(&trits.lo[offset..offset + chunk_length]);

            self.state.hi[0..chunk_length]
                .copy_from_slice(&trits.hi[offset..offset + chunk_length]);

            self.transform();

            offset += chunk_length;

            if length > chunk_length {
                length -= chunk_length;
            } else {
                break;
            }
        }
    }

    pub fn squeeze(&mut self, trit_count: usize) -> Trits64 {
        let mut result = Trits64::from(vec![0; trit_count], vec![0; trit_count]);

        let hash_count = trit_count / self.hash_length;

        for i in 0..hash_count {
            let offset = i * self.hash_length;

            result.lo[offset..offset + self.hash_length]
                .copy_from_slice(&self.state.lo[0..self.hash_length]);

            result.hi[offset..offset + self.hash_length]
                .copy_from_slice(&self.state.hi[0..self.hash_length]);

            self.transform();
        }

        let last = trit_count - hash_count * self.hash_length;
        let offset = trit_count - last;

        result.lo[offset..offset + last].copy_from_slice(&self.state.lo[0..last]);
        result.hi[offset..offset + last].copy_from_slice(&self.state.hi[0..last]);

        if trit_count % self.hash_length != 0 {
            self.transform();
        }

        result
    }

    fn transform(&mut self) {
        let mut scratch_pad_lo = vec![0u64; self.state_length];
        let mut scratch_pad_hi = vec![0u64; self.state_length];
        let mut scratch_pad_index = 0;

        for _ in 0..self.num_rounds {
            scratch_pad_lo.copy_from_slice(&self.state.lo);
            scratch_pad_hi.copy_from_slice(&self.state.hi);

            for state_index in 0..self.state_length {
                let a = scratch_pad_lo[scratch_pad_index];
                let b = scratch_pad_hi[scratch_pad_index];

                if scratch_pad_index < 365 {
                    scratch_pad_index += 364;
                } else {
                    scratch_pad_index -= 365;
                }

                let d = b ^ scratch_pad_lo[scratch_pad_index];
                self.state.lo[state_index] = !(d & a);
                self.state.hi[state_index] = (a ^ scratch_pad_hi[scratch_pad_index]) | d;
            }
        }
    }
}

#[cfg(test)]
mod bct_curl_tests {
    use super::*;
    use crate::bct_mux::*;
    use crate::convert::*;

    const MAINNET_TRYTES_1: &str = "TLFCFY9IMZVINTAZRCUWTKAFENIBIFOGKWDZQIRTYSVVHTSIIZZ9RLUYVTLXEHACXIUFJJQNFRJYMGGYDWOBNMTPFE9CGVVTREVUJKIXRHSOPFAXMNEMHEW9ZE9HVFEDEORKWGLNECZ9MXLDHPBAOMO9ZMSZJCZLAWWZKOLHBASHYNMCBCPZOXOLLVMFZVCTUDQZSIUSITRDHHXGAOVTOMSKDTZXLSCNHNXJNVGOTZPJDRHOBUAPIAIGLCETVDWSOPEKAOWBNUIEUTTLPFQLRYVRJQJOCBVOZEK9TQMJQUPEZKLHIVMO9TRIUBQNXJYIXFUWFUYWDIIDBQXRYULR9RXPSLTRFY9IIMQBLGOXUZJAKFSEJCSTYP9SWRFCNTMDMRFFWCVZTNFYLFZISPCQ99OSTMJBNLYCQLKWETRLJEOEBJZBO9ZUZMGQIRCCLBANSVYABGKMQCKWIWHHH9FGKGIURCJDKTIQBFENQCYWAX9WHNQ9OKGIWILNFJGMERJNBHDPNFCASDKZLOXLALOSMUFXYKKCDKWVX9PBOVXMAICVTHBLPWPFWJWYBTLJLXNOHREEFTJDLTYPPFMZ9MTPBHNXQL9MXRLGMKRN9EJYZMDZEOZOMKVYWKORKIBKDYZTCPOHYIADIVJWCHRVWCE9DSSHIEEINJYQWBFBTOCZOBL9LLFKWIFAJT9ZQKEUZTBARTEYUBYQOKMRMKWLTJOPVKIDXIUWQFLVWBTAYNOREZSCKAGRGVRLQUBUGKKHLL9YBFMGUMNSUMAXMCRQOQHBYJJRBMFQIUPZEBXFMHYJMAMAHUMMBLRDPBIOMJ9OCHBSBIFX9YSXPPVDMUCICHCSYRWUXXUEROHXGGEJBFJE9S9QGAQ9YOPIZOKGXRXMMFBLGVMC9QXJZTI99TATFJDJORMGJPAQGQICFHYAMWEUKWYYKIGTWYPNC9ZPQEKWAOZVCBIPZUTZUKJXFPWTQUKWIYJBULBJEJZGYEHVYUHFROLQYYPI9WCXHHWEITITPTXMTBWLJRAYV9LZK9FVGBOQRSWEFRMWBKBHAYWETHDTAAPOPPHFOX9PYQAXDVMWXGW9HDTLSINGRWGODCBNVXXYVDKJ9OROIZAULXMZUEVSDPWUJC9FEQAWMDOI9TALZAHX9ZHYSQEJOSZTHZPKWMZBTWUKNJUJNTZRWEYVWUAXVEP9NSZVYHLHZWDDTCQQTCDHTQPZXTM9ERHNNEORYBUKIRJPZORWXJDRRURZCBYLMFZKSZZVJIWXBXSKJMKUAFYKRQKVIGJJGYLXKFWZEIU9JJXRQSOFDLGXELTVBXKPDLKRLJTGVOD9QGIVVWS9EZAMBPDIEABEJJKTYQZVOD9TIGXPDJGJBRLHXCKKFFVQXFPQNKLMOMOJUDNFZCYEP9CQVNQKRYLCMCFNM9JIE9XUCDBX9ABNHZTSRROFYZCXDRLRBMYYRWUEWHC9QGGHBIQVBISISOZWXGXKQWSOASERXWNQXHWUGXDKIVDDWZZIRIERRSEOMEREYYCO9QIXKQOZQZALPBNQCBJWPV9BYDGYTDJPHXFZQ9CQZIDZTORKIABS9LFWOPWISFESVOTWIBTGDFIZBDOAJO9DJVAIQVUYEAWPRETXYWFMMUUUEUMWPGTWEUSZHJUCYGZDCSGVZGNTJBWGHGYZEOTOVIYAODKWJJLJFZGIKVGUYXRGAFMOFDM9SHSWVSDKAJGEVCORATXJHEGLYTVCGCTXZVUFVLZ9CYFCA9MM9STIZHKTGYJUACFVEGSZYJBNRWTRO9JUWZWOSPGJYIRTQSD9EPHONGYDWUQXYRHGXUSVGIAPVGOLLFQTQOYSOMHAOCNVKLPGRKIEVZGCFVWLTBEMM9QMUML9RVYCMOFIUCNTTALZKSGIPVNLFUGDPTHVGKDUIOZMKAEPYSYZTNFTMWJY99VGIM9YHI9WIVVJAANTHPKT9HOWWZSYRDMVJCSKASOZOOPAUOMMSOWNUTTGREQWPQDKRGGSODHKPFUIXKLVDFJSOQH9ZYMREQNXHHPOEISKPGTNIEBKV9SEFTKZZZVXQAYFPYTDMJVUULL9YNMITHTRB9GKILOFJCCYXKMPIYNNOXTVNLDKTODGEADIRIUXHNGVAAIEFYG9BE9BRNAZUABPF9BVODCZGPXBLBVJIXYLLYDVDUKVYGIWETMSKYXGYMXSXGKPDZMG9NOFIMSKFKIHTQSAVGIWERREF9MEAOCDE99FXRR9FDCKOZOJBTOZEVLLCASBONUMPDVD9XWSHEGZ9999999999999999999999999999999999999999999999VPRPPZD99A99999999J99999999KOJZIA9PSFRKG9ZUOJO9PGDIEFPGPSDKVPVBSXDIOOXAPZHKLJHEULIJKYRTDXOJKTRFYYSABGTBRKVCBBZZSWTVHQSQGJKQAHLINBNNLFTQERSITF9BAJCODBNLLQEQZETPQBGWFYCOBUARDAGTCGQCGOUBLA9999QPBMLSSKBO9ILX9QKYCAXNHLK9KFUJYO99GOO99VYROHOVXACRKYPFVY9JRSHJIKFGBHOCXQFPMZZ9999999999999999999999999999999HKJSFUCME999999999MMMMMMMMMCVMNOI9PFCHLRVXSUEOCRLTRMUF";
    const MAINNET_HASH_1: &str =
        "MGPBAHYHKSQMMXXONAOOEDQS9RFEKMOOJUCGXSFYLXBHQFWIHMJGFJWDSZTGKHNBCSENCXSPQOSZ99999";

    // NOTE: FAILS ATM!!!
    #[test]
    fn bct_curl_works() {
        let tx_trits = from_tryte_string(MAINNET_TRYTES_1);
        let mut bct_curl = BCTCurl64::new(HASH_LENGTH, NUM_ROUNDS);

        // add those trits a few times
        let mut mux = BCTMultiplexer::default();
        mux.add(&tx_trits);
        mux.add(&tx_trits);

        let trits64 = mux.extract64();

        bct_curl.absorb(trits64);

        let hash_trits64 = bct_curl.squeeze(HASH_LENGTH);

        let demux = BCTDemultiplexer64::from(hash_trits64);
        let hash_trits1 = demux.get(0);
        let hash_trits2 = demux.get(1);
        let hash_trits3 = demux.get(2);
        //println!("{:?}", hash_trits3);
        //println!("{}", hash_trits.len());

        let tryte_string1 = string_from_trytes(&trytes_from_trits(&hash_trits1));
        let tryte_string2 = string_from_trytes(&trytes_from_trits(&hash_trits2));
        let tryte_string3 = string_from_trytes(&trytes_from_trits(&hash_trits3));
        println!("{}", tryte_string1);
        println!("{}", tryte_string2);
        println!("{}", tryte_string3);
        assert_eq!(MAINNET_HASH_1, tryte_string1);
        assert_eq!(MAINNET_HASH_1, tryte_string2);
        assert_ne!(MAINNET_HASH_1, tryte_string3);
    }

}
