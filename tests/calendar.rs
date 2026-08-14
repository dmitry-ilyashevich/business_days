#[allow(unused_imports)]
use business_days::{Calendar, Country};
use chrono::{NaiveDate, Weekday};

#[allow(dead_code)]
fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).unwrap()
}

#[test]
#[cfg(feature = "pl")]
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

#[test]
#[cfg(feature = "us")]
fn us_business_days() {
    let cal = Calendar::new(Country::US).unwrap();

    assert!(cal.is_holiday(date(2026, 7, 3))); // Independence Day 
    assert!(!cal.is_business_day(date(2026, 7, 3))); // Independence Day

    assert!(cal.is_weekend(date(2026, 7, 4))); // Saturday 

    assert_eq!(
        cal.holiday(date(2026, 7, 3)).unwrap().name_en,
        "Independence Day"
    );
    assert_eq!(
        cal.holiday(date(2025, 7, 4)).unwrap().name_en,
        "Independence Day"
    );
}

#[test]
#[cfg(feature = "us")]
fn ua_count_between() {
    let cal = Calendar::new(Country::US).unwrap();

    // Jan 1 Thu = holiday, Jan 2 Fri = work, Jan 3/4 = weekend, Jan 5-7 = work
    assert_eq!(
        cal.business_days_between(date(2026, 1, 1), Some(date(2026, 1, 7)))
            .unwrap(),
        4
    );

    // single working day, inclusive on both ends
    assert_eq!(
        cal.business_days_between(date(2026, 1, 2), Some(date(2026, 1, 2)))
            .unwrap(),
        1
    );
}

#[test]
#[cfg(feature = "ua")]
fn count_defaults_to_today() {
    let cal = Calendar::new(Country::UA).unwrap();

    let today = chrono::Local::now().date_naive();
    let explicit = cal
        .business_days_between(date(2026, 1, 1), Some(today))
        .unwrap();
    let implicit = cal.business_days_between(date(2026, 1, 1), None).unwrap();

    assert_eq!(explicit, implicit);
}

#[test]
#[cfg(feature = "us")]
fn count_errors() {
    let cal = Calendar::new(Country::US).unwrap();

    assert!(
        cal.business_days_between(date(2026, 1, 7), Some(date(2026, 1, 1)))
            .is_err()
    );
    assert!(
        cal.business_days_between(date(1999, 1, 1), Some(date(2026, 1, 1)))
            .is_err()
    );
    assert!(
        cal.business_days_between(date(2026, 1, 1), Some(date(2090, 1, 1)))
            .is_err()
    );
}

#[test]
#[cfg(feature = "ua")]
fn with_years_restricts_coverage() {
    let cal = Calendar::with_years(Country::UA, 2020..2022).unwrap();
    assert_eq!(cal.covered_years(), 2020..=2021);
    assert!(cal.business_days_between(date(2022, 1, 1), None).is_err());
}

#[test]
#[cfg(all(feature = "us", feature = "eg", feature = "dz"))]
fn weekend_days_per_country() {
    assert_eq!(Country::US.weekend(), &[Weekday::Sat, Weekday::Sun]);
    assert_eq!(Country::EG.weekend(), &[Weekday::Fri, Weekday::Sat]);
    assert_eq!(Country::DZ.weekend(), &[Weekday::Fri, Weekday::Sat]);
}

#[test]
#[cfg(feature = "eg")]
fn egypt_friday_is_not_business_day() {
    let cal = Calendar::new(Country::EG).unwrap();

    assert!(cal.is_weekend(date(2026, 7, 3))); // Friday
    assert!(!cal.is_business_day(date(2026, 7, 3)));
    assert!(cal.is_business_day(date(2026, 7, 5))); // Sunday is a workday
}

#[test]
#[cfg(feature = "gb")]
fn gb_substitute_day_is_in_data() {
    let cal = Calendar::new(Country::GB).unwrap();

    // Boxing Day 2026 falls on Saturday; the API reports the substitute
    // Monday as the holiday - no shifting logic in this crate.
    assert!(cal.is_holiday(date(2026, 12, 28)));
    assert!(!cal.is_business_day(date(2026, 12, 28)));
    assert!(!cal.is_holiday(date(2026, 12, 26)));
}

#[test]
#[cfg(feature = "us")]
fn add_business_days() {
    let cal = Calendar::new(Country::US).unwrap();

    // Jan 1 Thu = New Year, Jan 2 Fri = work, Jan 3/4 = weekend, Jan 5-6 = work
    assert_eq!(
        cal.add_business_days(date(2026, 1, 1), 0).unwrap(),
        date(2026, 1, 2)
    );
    assert_eq!(
        cal.add_business_days(date(2026, 1, 1), 1).unwrap(),
        date(2026, 1, 5)
    );
    assert_eq!(
        cal.add_business_days(date(2026, 1, 1), 2).unwrap(),
        date(2026, 1, 6)
    );
    assert_eq!(
        cal.add_business_days(date(2026, 1, 1), 3).unwrap(),
        date(2026, 1, 7)
    );
}

#[test]
#[cfg(feature = "us")]
fn substract_business_days() {
    let cal = Calendar::new(Country::US).unwrap();

    // Jan 1 Thu = New Year, Jan 2 Fri = work, Jan 3/4 = weekend, Jan 5-7 = work
    assert_eq!(
        cal.subtract_business_days(date(2026, 1, 7), 1).unwrap(),
        date(2026, 1, 6)
    );
    assert_eq!(
        cal.subtract_business_days(date(2026, 1, 7), 2).unwrap(),
        date(2026, 1, 5)
    );
    assert_eq!(
        cal.subtract_business_days(date(2026, 1, 7), 3).unwrap(),
        date(2026, 1, 2)
    );
    assert_eq!(
        cal.subtract_business_days(date(2026, 1, 7), 5).unwrap(),
        date(2025, 12, 30)
    );
    assert_eq!(
        cal.subtract_business_days(date(2026, 1, 1), 0).unwrap(),
        date(2025, 12, 31)
    );
}

#[test]
#[cfg(feature = "us")]
fn roll_forward_to_next_business_day() {
    let cal = Calendar::new(Country::US).unwrap();

    // Jan 1 Thu = New Year, Jan 2 Fri = work, Jan 3/4 = weekend, Jan 5-7 = work
    assert_eq!(
        cal.roll_forward(date(2026, 1, 1)).unwrap(),
        date(2026, 1, 2)
    );
    assert_eq!(
        cal.roll_forward(date(2026, 1, 2)).unwrap(),
        date(2026, 1, 2)
    );
    assert_eq!(
        cal.roll_forward(date(2026, 1, 3)).unwrap(),
        date(2026, 1, 5)
    );
    assert_eq!(
        cal.roll_forward(date(2026, 1, 4)).unwrap(),
        date(2026, 1, 5)
    );
}

#[test]
#[cfg(feature = "us")]
fn roll_backward_to_next_business_day() {
    let cal = Calendar::new(Country::US).unwrap();

    // Jan 1 Thu = New Year, Jan 2 Fri = work, Jan 3/4 = weekend, Jan 5-7 = work
    assert_eq!(
        cal.roll_backward(date(2026, 1, 1)).unwrap(),
        date(2025, 12, 31)
    );
    assert_eq!(
        cal.roll_backward(date(2026, 1, 2)).unwrap(),
        date(2026, 1, 2)
    );
    assert_eq!(
        cal.roll_backward(date(2026, 1, 3)).unwrap(),
        date(2026, 1, 2)
    );
    assert_eq!(
        cal.roll_backward(date(2026, 1, 4)).unwrap(),
        date(2026, 1, 2)
    );
}
