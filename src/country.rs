use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Country {
    /// Poland
    PL,
}

impl Country {
    pub const ALL: [Country; 1] = [Country::PL];

    pub fn code(&self) -> &'static str {
        match self {
            Country::PL => "PL",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Country::PL => "Poland",
        }
    }

    pub fn weekend(&self) -> &'static [chrono::Weekday] {
        use chrono::Weekday::*;

        #[allow(clippy::match_single_binding)]
        match self {
            _ => &[Sat, Sun],
        }
    }
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

impl FromStr for Country {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PL" => Ok(Country::PL),
            other => Err(anyhow::anyhow!("Unknown country code: {other}")),
        }
    }
}
