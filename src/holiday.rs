use std::collections::{BTreeMap, HashMap};

use chrono::NaiveDate;

use crate::country::Country;

pub type Year = i32;

pub struct Holiday {
    pub date: NaiveDate,
    pub country: Country,
    pub country_name: &'static str,
    pub name: &'static str,
    pub name_en: &'static str,
}

pub type HolidayPerYearMap = HashMap<Year, BTreeMap<NaiveDate, Holiday>>;

/// Convert year, month, day to NaiveDate, returning an error if the date is invalid.
pub(crate) fn from_ymd_res(year: Year, month: u32, day: u32) -> anyhow::Result<NaiveDate> {
    NaiveDate::from_ymd_opt(year, month, day)
        .ok_or_else(|| anyhow::anyhow!("invalid date: {year}-{month:02}-{day:02}"))
}
