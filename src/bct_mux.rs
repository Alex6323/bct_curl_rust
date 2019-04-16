use crate::bct_types::*;

#[derive(Default)]
pub struct BCTMultiplexer {
    trinaries: Vec<Vec<i8>>,
}

impl BCTMultiplexer {
    pub fn reset(&mut self) {
        self.trinaries.clear();
    }

    pub fn add(&mut self, trits: &[i8]) {
        self.trinaries.push(Vec::from(trits));
    }

    pub fn get(&self, index: usize) -> &Vec<i8> {
        assert!(index < self.trinaries.len());
        &self.trinaries[index]
    }

    pub fn len(&self) -> usize {
        self.trinaries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.trinaries.is_empty()
    }

    pub fn extract64(&self) -> Trits64 {
        let trinaries_count = self.trinaries.len();
        let trits_count = self.trinaries[0].len();

        let mut result = Trits64 {
            lo: vec![0; trits_count],
            hi: vec![0; trits_count],
        };

        for i in 0..trits_count {
            for j in 0..trinaries_count {
                match self.trinaries[j][i] {
                    0 => {
                        result.lo[i] |= 0x1 << j;
                        result.hi[i] |= 0x1 << j;
                    }
                    1 => result.hi[i] |= 0x1 << j,
                    -1 => result.lo[i] |= 0x1 << j,
                    _ => panic!("unexpected trit value"),
                }
            }
        }

        result
    }

    pub fn extract128(&self) -> Trits128 {
        let trinaries_count = self.trinaries.len();
        let trits_count = self.trinaries[0].len();

        let mut result = Trits128 {
            lo: vec![0; trits_count],
            hi: vec![0; trits_count],
        };

        for i in 0..trits_count {
            for j in 0..trinaries_count {
                match self.trinaries[j][i] {
                    0 => {
                        result.lo[i] |= 0x1 << j;
                        result.hi[i] |= 0x1 << j;
                    }
                    1 => result.hi[i] |= 0x1 << j,
                    -1 => result.lo[i] |= 0x1 << j,
                    _ => panic!("unexpected trit value"),
                }
            }
        }

        result
    }
}

pub struct BCTDemultiplexer64 {
    trits: Trits64,
}

impl BCTDemultiplexer64 {
    pub fn from(trits: Trits64) -> Self {
        BCTDemultiplexer64 { trits }
    }

    pub fn get(&self, index: usize) -> Vec<i8> {
        let length = self.trits.lo.len();
        let mut result = vec![0i8; length];

        for i in 0..length {
            let lo = (self.trits.lo[i] >> index) & 0x1;
            let hi = (self.trits.hi[i] >> index) & 0x1;

            match (lo, hi) {
                (1, 0) => result[i] = -1,
                (0, 1) => result[i] = 1,
                (_, _) => result[i] = 0,
            }
        }
        result
    }
}

pub struct BCTDemultiplexer128 {
    trits: Trits128,
}

impl BCTDemultiplexer128 {
    pub fn from(trits: Trits128) -> Self {
        BCTDemultiplexer128 { trits }
    }

    pub fn get(&self, index: usize) -> Vec<i8> {
        let length = self.trits.lo.len();
        let mut result = vec![0i8; length];

        for i in 0..length {
            let lo = (self.trits.lo[i] >> index) & 0x1;
            let hi = (self.trits.hi[i] >> index) & 0x1;

            match (lo, hi) {
                (1, 0) => result[i] = -1,
                (0, 1) => result[i] = 1,
                (_, _) => result[i] = 0,
            }
        }
        result
    }
}

#[cfg(test)]
mod bct_mux_tests {
    use super::*;
    use crate::constants::*;
    use crate::convert::*;

    const MAINNET_TRYTES_1: &str = "TLFCFY9IMZVINTAZRCUWTKAFENIBIFOGKWDZQIRTYSVVHTSIIZZ9RLUYVTLXEHACXIUFJJQNFRJYMGGYDWOBNMTPFE9CGVVTREVUJKIXRHSOPFAXMNEMHEW9ZE9HVFEDEORKWGLNECZ9MXLDHPBAOMO9ZMSZJCZLAWWZKOLHBASHYNMCBCPZOXOLLVMFZVCTUDQZSIUSITRDHHXGAOVTOMSKDTZXLSCNHNXJNVGOTZPJDRHOBUAPIAIGLCETVDWSOPEKAOWBNUIEUTTLPFQLRYVRJQJOCBVOZEK9TQMJQUPEZKLHIVMO9TRIUBQNXJYIXFUWFUYWDIIDBQXRYULR9RXPSLTRFY9IIMQBLGOXUZJAKFSEJCSTYP9SWRFCNTMDMRFFWCVZTNFYLFZISPCQ99OSTMJBNLYCQLKWETRLJEOEBJZBO9ZUZMGQIRCCLBANSVYABGKMQCKWIWHHH9FGKGIURCJDKTIQBFENQCYWAX9WHNQ9OKGIWILNFJGMERJNBHDPNFCASDKZLOXLALOSMUFXYKKCDKWVX9PBOVXMAICVTHBLPWPFWJWYBTLJLXNOHREEFTJDLTYPPFMZ9MTPBHNXQL9MXRLGMKRN9EJYZMDZEOZOMKVYWKORKIBKDYZTCPOHYIADIVJWCHRVWCE9DSSHIEEINJYQWBFBTOCZOBL9LLFKWIFAJT9ZQKEUZTBARTEYUBYQOKMRMKWLTJOPVKIDXIUWQFLVWBTAYNOREZSCKAGRGVRLQUBUGKKHLL9YBFMGUMNSUMAXMCRQOQHBYJJRBMFQIUPZEBXFMHYJMAMAHUMMBLRDPBIOMJ9OCHBSBIFX9YSXPPVDMUCICHCSYRWUXXUEROHXGGEJBFJE9S9QGAQ9YOPIZOKGXRXMMFBLGVMC9QXJZTI99TATFJDJORMGJPAQGQICFHYAMWEUKWYYKIGTWYPNC9ZPQEKWAOZVCBIPZUTZUKJXFPWTQUKWIYJBULBJEJZGYEHVYUHFROLQYYPI9WCXHHWEITITPTXMTBWLJRAYV9LZK9FVGBOQRSWEFRMWBKBHAYWETHDTAAPOPPHFOX9PYQAXDVMWXGW9HDTLSINGRWGODCBNVXXYVDKJ9OROIZAULXMZUEVSDPWUJC9FEQAWMDOI9TALZAHX9ZHYSQEJOSZTHZPKWMZBTWUKNJUJNTZRWEYVWUAXVEP9NSZVYHLHZWDDTCQQTCDHTQPZXTM9ERHNNEORYBUKIRJPZORWXJDRRURZCBYLMFZKSZZVJIWXBXSKJMKUAFYKRQKVIGJJGYLXKFWZEIU9JJXRQSOFDLGXELTVBXKPDLKRLJTGVOD9QGIVVWS9EZAMBPDIEABEJJKTYQZVOD9TIGXPDJGJBRLHXCKKFFVQXFPQNKLMOMOJUDNFZCYEP9CQVNQKRYLCMCFNM9JIE9XUCDBX9ABNHZTSRROFYZCXDRLRBMYYRWUEWHC9QGGHBIQVBISISOZWXGXKQWSOASERXWNQXHWUGXDKIVDDWZZIRIERRSEOMEREYYCO9QIXKQOZQZALPBNQCBJWPV9BYDGYTDJPHXFZQ9CQZIDZTORKIABS9LFWOPWISFESVOTWIBTGDFIZBDOAJO9DJVAIQVUYEAWPRETXYWFMMUUUEUMWPGTWEUSZHJUCYGZDCSGVZGNTJBWGHGYZEOTOVIYAODKWJJLJFZGIKVGUYXRGAFMOFDM9SHSWVSDKAJGEVCORATXJHEGLYTVCGCTXZVUFVLZ9CYFCA9MM9STIZHKTGYJUACFVEGSZYJBNRWTRO9JUWZWOSPGJYIRTQSD9EPHONGYDWUQXYRHGXUSVGIAPVGOLLFQTQOYSOMHAOCNVKLPGRKIEVZGCFVWLTBEMM9QMUML9RVYCMOFIUCNTTALZKSGIPVNLFUGDPTHVGKDUIOZMKAEPYSYZTNFTMWJY99VGIM9YHI9WIVVJAANTHPKT9HOWWZSYRDMVJCSKASOZOOPAUOMMSOWNUTTGREQWPQDKRGGSODHKPFUIXKLVDFJSOQH9ZYMREQNXHHPOEISKPGTNIEBKV9SEFTKZZZVXQAYFPYTDMJVUULL9YNMITHTRB9GKILOFJCCYXKMPIYNNOXTVNLDKTODGEADIRIUXHNGVAAIEFYG9BE9BRNAZUABPF9BVODCZGPXBLBVJIXYLLYDVDUKVYGIWETMSKYXGYMXSXGKPDZMG9NOFIMSKFKIHTQSAVGIWERREF9MEAOCDE99FXRR9FDCKOZOJBTOZEVLLCASBONUMPDVD9XWSHEGZ9999999999999999999999999999999999999999999999VPRPPZD99A99999999J99999999KOJZIA9PSFRKG9ZUOJO9PGDIEFPGPSDKVPVBSXDIOOXAPZHKLJHEULIJKYRTDXOJKTRFYYSABGTBRKVCBBZZSWTVHQSQGJKQAHLINBNNLFTQERSITF9BAJCODBNLLQEQZETPQBGWFYCOBUARDAGTCGQCGOUBLA9999QPBMLSSKBO9ILX9QKYCAXNHLK9KFUJYO99GOO99VYROHOVXACRKYPFVY9JRSHJIKFGBHOCXQFPMZZ9999999999999999999999999999999HKJSFUCME999999999MMMMMMMMMCVMNOI9PFCHLRVXSUEOCRLTRMUF";
    const MAINNET_HASH_1: &str =
        "MGPBAHYHKSQMMXXONAOOEDQS9RFEKMOOJUCGXSFYLXBHQFWIHMJGFJWDSZTGKHNBCSENCXSPQOSZ99999";

    #[test]
    fn bct_multiplexer_works() {
        let tx_trits = from_tryte_string(MAINNET_TRYTES_1);

        // add those trits a few times
        let mut mux = BCTMultiplexer::default();
        mux.add(&tx_trits);
        mux.add(&tx_trits);
        mux.add(&tx_trits);
        assert_eq!(mux.get(0).len(), tx_trits.len());
        assert_eq!(mux.get(1).len(), tx_trits.len());
        assert_eq!(mux.get(2).len(), tx_trits.len());

        let trits64 = mux.extract64();
        // TODO: add assert_eq!
    }
}
