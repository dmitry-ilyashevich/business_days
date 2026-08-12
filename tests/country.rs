#[allow(unused_imports)]
use business_days::Country;
#[allow(unused_imports)]
use chrono::Weekday::*;

#[test]
#[cfg(feature = "pl")]
fn weekend_days_per_country() {
    assert_eq!(Country::PL.weekend(), &[Sat, Sun]);
}
