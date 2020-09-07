use binary_timehash::{Dta};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn test() {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let current_timehash = Dta::<5>::new(154, 134).unwrap();
    // assert_eq!(
    //     current_timehash.as_slice(),
    //     current_timehash.as_slice(),
    //     "rest {:?}",
    //     current_timehash
    // );
	println!("{:?} string: {}", current_timehash.as_bytes(), current_timehash);
	// println!("{:?} {:?}", TimeOrigin::to_index(TimeOrigin::Picosecond), TimeOrigin::to_index(TimeOrigin::Millisecond));
}
