// TODO MAX/MIN

// 255^n
// 255^6 ms = 8 712.57223 years
const MULTIPLIERS: [[u128; 6]; 1] = [
    [1078203909375, 4228250625, 16581375, 65025, 255, 1],
    // [0; 6], //TODO
];

#[allow(dead_code)]
const COARSENESS: [u8; 15] = [
    0b0000_0000_,
    0b0000_0001_,
    0b0000_0010_,
    0b0000_0011_,
    0b0000_0100_,
    0b0000_0101_,
    0b0000_0110_,
    0b0000_0111_,
    0b0000_1000_,
    0b0000_1001_,
    0b0000_1010_,
    0b0000_1100_,
    0b0000_1101_,
    0b0000_1110_,
    0b0000_1111_,
];

#[derive(Debug)]
pub struct TimeHash([u8; 7]);

impl From<u128> for TimeHash {
    fn from(time_millis: u128) -> TimeHash {
        let mut codes: [u8; 7] = [0; 7];

        // codes[0] = MULTIPLIER_256;
        let multiplier: [u128; 6] = MULTIPLIERS[0];
        let mut remaining_timeframe = time_millis;
        for x in 0..6 {
            let frame_code = remaining_timeframe / multiplier[x];
            codes[x + 1] = frame_code as u8;
            remaining_timeframe = remaining_timeframe - multiplier[x] * frame_code;
        }
        let joined_prefix = 0; // TODO implement prefix options
                               // checked_add
        codes[0] = joined_prefix;
        Self(codes)
    }
}

impl TimeHash {
    #[allow(unused_variables)]
    pub fn new(time_millis: u64, config: TimeHashConfig) {
        unimplemented!()
    }
    pub fn as_slice(&self) -> [u8; 7] {
        self.0
    }
}
// ms 10^-3
// ps 10^-12
// as u8 >> 6
#[derive(Copy, Clone)]
pub enum TimeOrigin {
    Millisecond = 0b_00_000000,
    Picosecond = 0b_01_000000,
}

impl TimeOrigin {
    pub fn to_index(self) -> usize {
        (self as u8 >> 6) as usize
    }
}

// 256 blocks/step
// 16 blocks/step
// as u8 >> 4
#[derive(Copy, Clone, Debug)]
pub enum ChunkSize {
    _256 = 0b00_0_00000,
    _16 = 0b00_1_00000,
}

impl ChunkSize {
    pub fn to_u8(self) -> usize {
        (self as u8 >> 6) as usize
    }
}


type Coarseness = u8;

#[derive(Copy, Clone)]
pub struct TimeHashConfig(TimeOrigin, ChunkSize, Coarseness);

// 274941996890625 seconds range for code 0
// 1_599_130_443_882
