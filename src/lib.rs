pub mod calendar;
pub mod country;
pub mod data;

mod holiday;

pub use anyhow::Result;
pub use calendar::Calendar;
pub use country::Country;
pub use holiday::{Holiday, HolidayPerYearMap, Year};

/// Build a map of holidays for a given country and optional range of years.
pub fn build(country: Country, years: Option<&std::ops::Range<Year>>) -> Result<HolidayPerYearMap> {
    data::build(country, years)
}
