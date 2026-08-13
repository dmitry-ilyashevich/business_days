# business_days

Rust library with embedded public-holiday data for >200 countries, generated from the [nagerholidays.com](https://nagerholidays.com/) API, years 2000..=2031 (this range can be changed).

## Usage

```rust
use business_days::{Calendar, Country};
use chrono::NaiveDate;

let cal = Calendar::new(Country::UA)?;
let date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();

cal.is_holiday(date);                      // true - New Year's Day
cal.is_business_day(date);                 // false
cal.holiday(date);                         // Some(&Holiday { name: "Новий Рік", name_en: "New Year's Day", ... })
cal.business_days_between(date, None)?;    // business days from `date` through today, inclusive
```

Weekly rest days come from Unicode CLDR week data and differ per country (Sat/Sun for most, Fri/Sat for Egypt or Algeria, Sun for Uganda, etc.); see `Country::weekend()`. Observed/substitute days ("holiday falls on a weekend -> next workday is off") are already encoded in the upstream data as separate dates, so the calendar does no shifting of its own.

The raw per-year data is also available:

```rust
use business_days::{build, Country};

let map = build(Country::UA, Some(&(2020..2031)))?; // HashMap<Year, BTreeMap<NaiveDate, Holiday>>
let holidays_2026 = &map[&2026];
```

Only nationwide public holidays are included (regional and Bank/School/ Observance entries are filtered out). Each holiday carries its name in the country's local language (`Holiday::name`) and in English (`Holiday::name_en`).

Each country is behind a cargo feature named after its lowercase ISO code (`ua`, `us`, `de`, ...). The default `all` feature enables every country; to compile only what you need:

```toml
business_days = { version = "0.1", default-features = false, features = ["ua", "pl"] }
```

## Regenerating the data

`src/country.rs`, `src/data/` and the feature list in `Cargo.toml` are generated - do not edit them by hand. Raw API responses and the CLDR `weekData.json` are cached in `builder/cache/` (gitignored, but you can use builder to refetch and recreate).

```sh
cargo run -p builder                       # fetch missing years, regenerate everything
cargo run -p builder -- --refresh          # also re-fetch current & future years
cargo run -p builder -- --countries UA,PL  # limit to specific countries
cargo run -p builder -- --end-year 2033    # extend the horizon (default: current year + 5)
cargo run -p builder -- --help             # print help with all possible arguments
```

Run `--refresh` once a year to pick up the next future year and any upstream corrections, then review the diff and commit.

## Need a different year range?

The published crate only embeds years 2000 through current year + 5. That range is baked in at build time by `builder`, so a crate consuming `business_days` from crates.io can't extend it just by calling `build()` with a wider range - years outside the embedded window simply return no data.

If you need years the published release doesn't cover (further into the future, or further back before 2000, if upstream has it), regenerate the data yourself and point your app at that local copy instead of the crates.io version:

1. Clone this repository:
   ```sh
   git clone https://github.com/dmitry-ilyashevich/business_days
   ```
2. Regenerate the data for the range and/or countries you need:
   ```sh
   cargo run -p builder -- --end-year 2040
   ```
3. In your app's `Cargo.toml`, replace the crates.io dependency with a path (or git) dependency pointing at your clone:
   ```toml
   business_days = { path = "../business_days" }
   # or, if you pushed the regenerated data to your own fork:
   business_days = { git = "https://github.com/<you>/business_days", branch = "custom-range" }
   ```
4. Build your app as usual. `path`/`git` dependencies are compiled from source, so your app picks up the regenerated `src/data/` rather than whatever was last published.

Since step 3 points at a directory instead of a registry version, `cargo` picks up any further changes you make there the next time you build - no need to bump a version or republish anything to iterate locally.
