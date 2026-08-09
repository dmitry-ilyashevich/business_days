use std::collections::BTreeMap;
use std::ops::{Range, RangeInclusive};

use anyhow::{Result, bail};
use chrono::{Datelike, NaiveDate, Weekday};

use crate::country::Country;
use crate::holiday::{Holiday, Year};

pub struct Calendar {
    country: Country,
    weekend: &'static [Weekday],
    holidays: BTreeMap<NaiveDate, Holiday>,
    years: RangeInclusive<Year>,
}

impl Calendar {
    pub fn new(country: Country) -> Result<Self> {
        Self::build(country, None)
    }

    /// Create a calendar for a specific country and a range of years.
    pub fn with_years(country: Country, years: Range<Year>) -> Result<Self> {
        Self::build(country, Some(&years))
    }

    pub fn build(country: Country, years: Option<&Range<Year>>) -> Result<Self> {
        let per_year = crate::data::build(country, years)?;

        let (Some(&first_year), Some(&last_year)) = (per_year.keys().min(), per_year.keys().max())
        else {
            bail!("No holidays found for country {country:?} in the requested years");
        };

        Ok(Self {
            country,
            weekend: country.weekend(),
            holidays: per_year.into_values().flatten().collect(),
            years: first_year..=last_year,
        })
    }

    /// Get the country this calendar is for.
    pub fn country(&self) -> Country {
        self.country
    }

    /// Years this calendar has holidays data for.
    pub fn covered_years(&self) -> RangeInclusive<Year> {
        self.years.clone()
    }

    /// Get the holiday for a given date, if it exists.
    pub fn holiday(&self, date: NaiveDate) -> Option<&Holiday> {
        self.holidays.get(&date)
    }

    /// Check if a given date is a holiday.
    pub fn is_holiday(&self, date: NaiveDate) -> bool {
        self.holidays.contains_key(&date)
    }

    /// Check if a given date is a weekend day.
    pub fn is_weekend(&self, date: NaiveDate) -> bool {
        self.weekend.contains(&date.weekday())
    }

    /// Check if a given date is a business day (not a holiday and not a weekend).
    pub fn is_business_day(&self, date: NaiveDate) -> bool {
        !self.is_holiday(date) && !self.is_weekend(date)
    }
}
