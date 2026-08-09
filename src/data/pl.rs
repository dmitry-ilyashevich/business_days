use super::*;

pub fn build(years: Option<&Range<Year>>) -> Result<HolidayPerYearMap> {
    let mut map = HashMap::new();

    build_year(
        years,
        2026,
        [
            (from_ymd_res(2026, 1, 1)?, "nowy rok", "new year's day"),
            (from_ymd_res(2026, 1, 6)?, "święto trzech króli", "epiphany"),
            (from_ymd_res(2026, 4, 5)?, "wielkanoc", "easter sunday"),
            (
                from_ymd_res(2026, 4, 6)?,
                "drugi dzień wielkanocy",
                "easter monday",
            ),
            (from_ymd_res(2026, 5, 1)?, "święto pracy", "may day"),
            (
                from_ymd_res(2026, 5, 3)?,
                "święto konstytucji 3 maja",
                "constitution day",
            ),
            (from_ymd_res(2026, 5, 24)?, "zielone świątki", "pentecost"),
            (from_ymd_res(2026, 6, 4)?, "boże ciało", "corpus christi"),
            (
                from_ymd_res(2026, 8, 15)?,
                "wniebowzięcie najświętszej maryi panny",
                "assumption day",
            ),
            (
                from_ymd_res(2026, 11, 1)?,
                "wszystkich świętych",
                "all saints' day",
            ),
            (
                from_ymd_res(2026, 11, 11)?,
                "narodowe święto niepodległości",
                "independence day",
            ),
            (
                from_ymd_res(2026, 12, 24)?,
                "wigilia bożego narodzenia",
                "christmas eve",
            ),
            (
                from_ymd_res(2026, 12, 25)?,
                "boże narodzenie",
                "christmas day",
            ),
            (
                from_ymd_res(2026, 12, 26)?,
                "drugi dzień bożego narodzenia",
                "st. stephen's day",
            ),
        ],
        &mut map,
        Country::PL,
        "Poland",
    );

    Ok(map)
}
