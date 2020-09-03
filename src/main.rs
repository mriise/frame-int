use std::time::{SystemTime, UNIX_EPOCH};
// 255^n
// 255^6 ms = 8 712.57223 years
const MULTIPLIERS: [u64; 6] = [1078203909375, 4228250625, 16581375, 65025, 255, 1];

fn main() {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Clock may have gone backwards");

    let encoded = encode(since_the_epoch.as_millis() as u64);

    println!("{:?}", encoded);
  

    fn encode(time_millis: u64) -> [u8; 6] {

        let mut codes: [u8; 6] = [0; 6];

        let mut remaining_timeframe = time_millis;
        for x in 0..6 {
            let frame_code = remaining_timeframe / (MULTIPLIERS[x]);
            // println!("frame: {} remaining: {} multiplier: {}", frame_code, remaining_timeframe, multipliers[x]);
            codes[x] = frame_code as u8;
            remaining_timeframe = remaining_timeframe - MULTIPLIERS[x] * frame_code;
        }
        codes
    }
}
// 274941996890625
// 1_599_130_443_882
