use std::collections::BTreeMap;
use std::ops::{Range, RangeInclusive};

use anyhow::{Context, Result, bail};
use chrono::{Datelike, NaiveDate, Weekday};

use crate::country::Country;
use crate::holiday::{Holiday, Year};

/// Business-day calendar for one country.
///
/// Combines the embedded public-holiday data (date.nager.at) with the
/// country's weekly rest days (Unicode CLDR week data - Sat/Sun for most
/// countries, e.g. Fri/Sat for Egypt).
///
/// Observed holidays are already part of the data: where a country moves a
/// weekend holiday to the next working day (US "in lieu" days, UK substitute
/// bank holidays, …), the API reports the substitute date as the holiday, so
/// no extra shifting is done here.
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

    /// Number of business days from `start` to `end`, inclusive.
    ///
    /// `end` defaults to today if not provided. Returns an error if
    /// `start` is after `end` or the range is not fully within [`covered_years()`](Self::covered_years).
    pub fn business_days_between(&self, start: NaiveDate, end: Option<NaiveDate>) -> Result<u32> {
        let end = end.unwrap_or_else(|| chrono::Local::now().naive_local().date());

        if start > end {
            bail!("start date {start} is after end date {end}");
        }

        if !self.covered_years().contains(&start.year())
            || !self.covered_years().contains(&end.year())
        {
            bail!(
                "date range {start} to {end} is not fully within covered years {:?}",
                self.covered_years()
            );
        }

        let mut count = 0;
        let mut day = start;
        while day <= end {
            if self.is_business_day(day) {
                count += 1;
            }
            day = day.succ_opt().context("date out of range")?;
        }

        Ok(count)
    }

    /// Roll a date forward to the next business day.
    ///
    /// Returns an error if the date is not within [`covered_years()`](Self::covered_years).
    pub fn roll_forward(&self, mut day: NaiveDate) -> Result<NaiveDate> {
        while !self.is_business_day(day) {
            day = day.succ_opt().context("date out of range")?;
        }
        Ok(day)
    }

    /// Roll a date backward to the previous business day.
    ///
    /// Returns an error if the date is not within [`covered_years()`](Self::covered_years).
    pub fn roll_backward(&self, mut day: NaiveDate) -> Result<NaiveDate> {
        while !self.is_business_day(day) {
            day = day.pred_opt().context("date out of range")?;
        }
        Ok(day)
    }

    /// Add a number of business days to a given start date.
    ///
    /// Returns an error if the start date is not within [`covered_years()`](Self::covered_years).
    pub fn add_business_days(&self, start: NaiveDate, days: u32) -> Result<NaiveDate> {
        let start_business_day = self.roll_forward(start)?;

        if !self.covered_years().contains(&start_business_day.year()) {
            bail!(
                "start date {start_business_day} is not within covered years {:?}",
                self.covered_years()
            );
        }

        let mut count = 0;
        let mut day = start_business_day;
        while count < days {
            day = day.succ_opt().context("date out of range")?;
            if self.is_business_day(day) {
                count += 1;
            }
        }

        Ok(day)
    }

    /// Subtract a number of business days from a given start date.
    ///
    /// Returns an error if the start date is not within [`covered_years()`](Self::covered_years).
    pub fn subtract_business_days(&self, start: NaiveDate, days: u32) -> Result<NaiveDate> {
        let start_business_day = self.roll_backward(start)?;

        if !self.covered_years().contains(&start_business_day.year()) {
            bail!(
                "start_business_day date {start_business_day} is not within covered years {:?}",
                self.covered_years()
            );
        }

        let mut count = 0;
        let mut day = start_business_day;
        while count < days {
            day = day.pred_opt().context("date out of range")?;

            if self.is_business_day(day) {
                count += 1;
            }
        }

        Ok(day)
    }
}
