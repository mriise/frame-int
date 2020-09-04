use std::time::{SystemTime, UNIX_EPOCH};
// 255^n
// 255^6 ms = 8 712.57223 years
const MULTIPLIERS: [[u64; 6]; 2] = [
    [1078203909375, 4228250625, 16581375, 65025, 255, 1],
    [0; 6], //TODO
];
// const MULTIPLIER_16: [u64; 4] = [];

// ms 10^-3
// ps 10^-12
// const TIME_ORIGIN: [u8; 2] = [0b_00_000000, 0b_01_000000];

// 256 blocks/step
// 16 blocks/step
// const MULTIPLIER_SCALE: [u8; 2] = [0b00_00_0000, 0b00_01_0000];

fn main() {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Clock may have gone backwards");

    fn encode(time_millis: u64, multiplier: usize) -> [u8; 7] {
        let mut codes: [u8; 7] = [0; 7];
        // codes[0] = MULTIPLIER_256;
        let multiplier: [u64; 6] = MULTIPLIERS[multiplier];
        let mut remaining_timeframe = time_millis;
        for x in 0..6 {
            let frame_code = remaining_timeframe / multiplier[x];
            codes[x+1] = frame_code as u8;
            remaining_timeframe = remaining_timeframe - multiplier[x] * frame_code;
        }
        let joined_prefix = 0; // TODO implement prefix options
        // checked_add
        codes[0] = joined_prefix;
        codes
    }
}

// #[derive(Copy)]
// struct TimeHash {
//     slice: [u8; 7]
// }

// impl From<64> for TimeHash {
//     fn from
// }
// 274941996890625 seconds range for code 0
// 1_599_130_443_882
