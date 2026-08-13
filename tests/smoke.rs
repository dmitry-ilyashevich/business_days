use business_days::{Country, build};
use chrono::NaiveDate;

#[test]
#[cfg(feature = "ua")]
fn ua_new_year_2026() {
    let map = build(Country::UA, None).unwrap();
    let date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
    let holiday = &map[&2026][&date];

    assert_eq!(holiday.name, "Новий Рік");
    assert_eq!(holiday.name_en, "New Year's Day");
    assert_eq!(holiday.country, Country::UA);
    assert_eq!(holiday.country_name, "Ukraine");
}

#[test]
#[cfg(feature = "us")]
fn us_covers_2000_through_2031() {
    let map = build(Country::US, None).unwrap();

    assert!(map.contains_key(&2000));
    assert!(map.contains_key(&2031));
}

#[test]
#[cfg(feature = "us")]
fn year_range_filter() {
    let map = build(Country::US, Some(&(2020..2022))).unwrap();

    assert!(map.contains_key(&2020));
    assert!(map.contains_key(&2021));
    assert!(!map.contains_key(&2022));
    assert!(!map.contains_key(&2019));
}

#[test]
#[cfg(feature = "us")]
fn country_roundtrip() {
    let country: Country = "US".parse().unwrap();

    assert_eq!(country, Country::US);
    assert_eq!(country.code(), "US");
    assert_eq!(country.name(), "United States");
    assert_eq!(country.to_string(), "US");

    assert!(Country::ALL.contains(&country));
    assert!("XX".parse::<Country>().is_err());
}
