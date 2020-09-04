use binary_timehash::{TimeHash, TimeOrigin};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn test() {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let current_timehash = TimeHash::from(current_time);

    // assert_eq!(
    //     current_timehash.as_slice(),
    //     current_timehash.as_slice(),
    //     "rest {:?}",
    //     current_timehash
    // );
	println!("{:?}", current_timehash.as_slice());
	println!("{:?} {:?}", TimeOrigin::to_index(TimeOrigin::Picosecond), TimeOrigin::to_index(TimeOrigin::Millisecond));
}
