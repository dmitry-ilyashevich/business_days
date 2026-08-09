use business_days::Country;
use chrono::Weekday::*;

#[test]
fn weekend_days_per_country() {
    assert_eq!(Country::PL.weekend(), &[Sat, Sun]);
}
