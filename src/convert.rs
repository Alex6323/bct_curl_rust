use lazy_static::*;
use std::collections::HashMap;
use std::iter::FromIterator;

pub fn from_tryte_string(tryte_string: &str) -> Vec<i8> {
    let bytes = tryte_string.as_bytes();

    let mut trits = vec![0i8; tryte_string.len() * 3];

    bytes.iter().enumerate().for_each(|(i, c)| {
        trits[(i * 3)..(i * 3) + 3]
            .copy_from_slice(&TRYTE_TO_TRIT_TRIPLET[*ASCII_TO_TRYTE.get(&c).unwrap()][..]);
    });

    trits
}

const TRYTE_TO_ASCII: [u8; 27] = [
    57, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87,
    88, 89, 90,
];

lazy_static! {
    pub static ref ASCII_TO_TRYTE: HashMap<u8, usize> =
        { HashMap::from_iter(TRYTE_TO_ASCII.iter().enumerate().map(|(v, &k)| (k, v))) };
}

const TRYTE_TO_TRIT_TRIPLET: [[i8; 3]; 27] = [
    [0, 0, 0],    // 9 => 0
    [1, 0, 0],    // A => 1
    [-1, 1, 0],   // B => 2
    [0, 1, 0],    // C => 3
    [1, 1, 0],    // D => 4
    [-1, -1, 1],  // E => 5
    [0, -1, 1],   // F => 6
    [1, -1, 1],   // G => 7
    [-1, 0, 1],   // H => 8
    [0, 0, 1],    // I => 9
    [1, 0, 1],    // J => 10
    [-1, 1, 1],   // K => 11
    [0, 1, 1],    // L => 12
    [1, 1, 1],    // M => 13
    [-1, -1, -1], // N => -13
    [0, -1, -1],  // O => -12
    [1, -1, -1],  // P => -11
    [-1, 0, -1],  // Q => -10
    [0, 0, -1],   // R => -9
    [1, 0, -1],   // S => -8
    [-1, 1, -1],  // T => -7
    [0, 1, -1],   // U => -6
    [1, 1, -1],   // V => -5
    [-1, -1, 0],  // W => -4
    [0, -1, 0],   // X => -3
    [1, -1, 0],   // Y => -2
    [-1, 0, 0],   // Z => -1
];

pub fn trytes_from_trits(trits: &[i8]) -> Vec<u8> {
    assert!(trits.len() % 3 == 0);
    let mut trytes = vec![TRYTE_TO_ASCII[0]; trits.len() / 3];
    let mut index;

    for (i, t) in trytes.iter_mut().enumerate() {
        index = trits[i * 3] + 3 * trits[i * 3 + 1] + 9 * trits[i * 3 + 2];
        index = if index < 0 { index + 27 } else { index };
        *t = TRYTE_TO_ASCII[index as usize];
    }

    trytes
}

pub fn string_from_trytes(trytes: &[u8]) -> String {
    String::from_utf8(trytes.to_vec()).unwrap()
}
