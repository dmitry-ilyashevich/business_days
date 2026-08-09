use business_days::{Calendar, Country};
use chrono::NaiveDate;

fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).unwrap()
}

#[test]
fn pl_business_days() {
    let cal = Calendar::new(Country::PL).unwrap();

    assert!(cal.is_holiday(date(2026, 1, 1))); // New Year's Day
    assert!(cal.is_holiday(date(2026, 5, 1))); // Labor Day
    assert!(cal.is_holiday(date(2026, 12, 25))); // Christmas Day

    assert!(!cal.is_holiday(date(2026, 1, 2))); // Not a holiday
    assert!(!cal.is_holiday(date(2026, 5, 2))); // Not a holiday

    assert!(cal.is_weekend(date(2026, 1, 3))); // Saturday
    assert!(cal.is_weekend(date(2026, 1, 4))); // Sunday

    assert!(!cal.is_weekend(date(2026, 1, 5))); // Monday
}
