pub const T27_NUM_COLUMNS: usize = 9;
pub const T27_NUM_SLICES: usize = 27;
pub const T27_NUM_ROWS: usize = 3;
pub const T27_SLICE_SIZE: usize = T27_NUM_COLUMNS * T27_NUM_ROWS;

pub const COLUMNS: usize = 9;
pub const ROWS: usize = 3;
pub const SLICES: usize = 27;
pub const SLICE_SIZE: usize = COLUMNS * ROWS;
pub const STATE_SIZE: usize = COLUMNS * ROWS * SLICES;
pub const NUM_SBOXES: usize = SLICES * ROWS * COLUMNS / 3;

pub const NUM_ROUNDS: usize = 24;
pub const TROIKA_RATE: usize = 243;

pub const PADDING: u8 = 0x1;

//#define _1(a) (a->p)
//#define _2(a) (a->n)
//#define _0(a) (~a->p & ~a->n)

//#define ZERO (_0(a) & _0(b) & _0(c))  // 0
//#define A (_1(a) & _0(b) & _0(c))     // 1
//#define B (_2(a) & _0(b) & _0(c))     // 2
//#define C (_0(a) & _1(b) & _0(c))     // 3
//#define D (_1(a) & _1(b) & _0(c))     // 4
//#define E (_2(a) & _1(b) & _0(c))     // 5
//#define F (_0(a) & _2(b) & _0(c))     // 6
//#define G (_1(a) & _2(b) & _0(c))     // 7
//#define H (_2(a) & _2(b) & _0(c))     // 8
//#define I (_0(a) & _0(b) & _1(c))     // 9
//#define J (_1(a) & _0(b) & _1(c))     // 10
//#define K (_2(a) & _0(b) & _1(c))     // 11
//#define L (_0(a) & _1(b) & _1(c))     // 12
//#define M (_1(a) & _1(b) & _1(c))     // 13
//#define N (_2(a) & _1(b) & _1(c))     // 14
//#define O (_0(a) & _2(b) & _1(c))     // 15
//#define P (_1(a) & _2(b) & _1(c))     // 16
//#define Q (_2(a) & _2(b) & _1(c))     // 17
//#define R (_0(a) & _0(b) & _2(c))     // 18
//#define S (_1(a) & _0(b) & _2(c))     // 19
//#define T (_2(a) & _0(b) & _2(c))     // 20
//#define U (_0(a) & _1(b) & _2(c))     // 21
//#define V (_1(a) & _1(b) & _2(c))     // 22
//#define W (_2(a) & _1(b) & _2(c))     // 23
//#define X (_0(a) & _2(b) & _2(c))     // 24
//#define Y (_1(a) & _2(b) & _2(c))     // 25
//#define Z (_2(a) & _2(b) & _2(c))     // 26
