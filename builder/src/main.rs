use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use chrono::Datelike;
use clap::Parser;
use serde::{Deserialize, Serialize};
use tera::{Context as TeraContext, Tera};

// V4 have not holidays names in local languages, so we use V3 for fetching
// holidays and V4 for fetching countries list
const API_V3_URL: &str = "https://nagerholidays.com/api/v3";
const API_V4_URL: &str = "https://nagerholidays.com/api/v4";
const WEEKDATA_URL: &str = "https://raw.githubusercontent.com/unicode-org/cldr-json/main/cldr-json/cldr-core/supplemental/weekData.json";

const MAX_RETRIES: u32 = 5;

const START_YEAR: u32 = 2000;
const FUTURE_YEARS: u32 = 5;

const REQUEST_DELAY: Duration = Duration::from_millis(150); // milliseconds

/// Two-letter country codes that collide with Rust keywords need `r#` module names.
const RUST_KEYWORDS: &[&str] = &["as", "do", "fn", "if", "in"];

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct CountryInfo {
    country_code: String,
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiHoliday {
    date: String,
    local_name: Option<String>,
    name: String,
    #[serde(default)]
    global: bool,
    #[serde(default)]
    types: Option<Vec<String>>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Comma-separated list of country codes to fetch and generate (default: all).
    #[arg(short, long, value_delimiter = ',')]
    countries: Option<Vec<String>>,

    // Override the default end_year (default: current year + FUTURE_YEARS).
    #[arg(short, long)]
    end_year: Option<u32>,

    // Re-fetch all data, ignoring any cached files.
    #[arg(short, long, default_value_t = false)]
    force: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let builder_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cache = builder_dir.join("cache");

    let current_year = chrono::Utc::now().year() as u32;
    let end_year = args.end_year.unwrap_or(current_year + FUTURE_YEARS);

    let countries = load_countries(&cache, args.force)?;
    let week_data = load_week_data(&cache, args.force)?;

    if let Some(selected_countries) = &args.countries {
        for country_code in selected_countries {
            if !countries.iter().any(|c| &c.country_code == country_code) {
                bail!("Country code {} is not available", country_code);
            }
        }
    }
    println!("{} countries available", countries.len());
    println!(
        "Loading {} countries",
        args.countries.as_ref().map_or(countries.len(), |c| c.len())
    );

    let countries_to_fetch: Vec<&CountryInfo> = if let Some(selected_countries) = &args.countries {
        countries
            .iter()
            .filter(|c| selected_countries.contains(&c.country_code))
            .collect()
    } else {
        countries.iter().collect()
    };

    // Fetch countries info
    for country in &countries_to_fetch {
        if let Some(selected_countries) = &args.countries {
            if !selected_countries.contains(&country.country_code) {
                continue;
            }
        }

        let dir = cache.join(&country.country_code);
        fs::create_dir_all(&dir).context("Failed to create cache directory")?;
        let (mut fetched, mut cached) = (0u32, 0u32);
        for year in START_YEAR..=end_year {
            let path = dir.join(format!("{year}.json"));
            if path.exists() && !args.force {
                cached += 1;
                continue;
            }

            let url = format!(
                "{API_V3_URL}/PublicHolidays/{year}/{country_code}",
                country_code = country.country_code
            );
            match get_with_retry(&url) {
                Ok(Some(body)) => {
                    // Validate before caching so a truncated response never poisons the cache.
                    serde_json::from_str::<Vec<ApiHoliday>>(&body)
                        .with_context(|| format!("unparsable response from {url}"))?;

                    fs::write(&path, body).context("Failed to write cache file")?;
                    fetched += 1;
                }
                Ok(None) => {
                    fs::write(&path, "[]").context("Failed to write empty cache file")?;
                    continue;
                }
                Err(e) => {
                    eprintln!(
                        "Failed to fetch holidays for {} in {}: {}",
                        country.country_code, year, e
                    );
                }
            }

            sleep(REQUEST_DELAY); // Avoid hitting the API too quickly
        }
        println!(
            "Country {}: fetched {} years, cached {} years",
            country.country_code, fetched, cached
        );
    }

    // Codegen holidays data for library
    let root_dir = builder_dir
        .parent()
        .context("Failed to find root directory")?
        .to_path_buf();
    let src_dir = root_dir.join("src");
    let data_dir = root_dir.join("src/data");
    fs::create_dir_all(&data_dir).context("Failed to create data directory")?;

    let mut generated = 0u32;
    for country in &countries {
        if let Some(selected_countries) = &args.countries {
            if !selected_countries.contains(&country.country_code) {
                continue;
            }
        }

        if generate_country(&cache, &data_dir, country)? {
            generated += 1;
        }
    }
    println!("Generated data for {generated} countries");

    generate_country_enums(builder_dir, &src_dir, &countries_to_fetch, &week_data)
        .context("Failed to generate enums")?;

    generate_mod_file(builder_dir, &src_dir, &countries_to_fetch)
        .context("Failed to generate mod.rs")?;

    update_cargo_toml(&root_dir, &countries_to_fetch).context("Failed to update Cargo.toml")?;

    Ok(())
}

fn load_countries(cache: &Path, refresh: bool) -> Result<Vec<CountryInfo>> {
    let path = cache.join("countries.json");
    let raw_body = if path.exists() && !refresh {
        fs::read_to_string(&path)?
    } else {
        let body = get_with_retry(&format!("{API_V4_URL}/Countries/Available"))?
            .context("Failed to fetch countries from API")?;
        body
    };

    let mut countries: Vec<CountryInfo> =
        serde_json::from_str(&raw_body).context("Failed to parse countries JSON")?;
    countries.sort_by(|a, b| a.country_code.cmp(&b.country_code));

    Ok(countries)
}

// Weekend days of week per country: country code -> list of weekend days
// e.g., "US" -> ["Sat", "Sun"]. Key '001' is the default for countries
// not listed in the CLDR data.
#[derive(Debug)]
struct WeekData(BTreeMap<String, Vec<&'static str>>);

fn load_week_data(cache: &Path, refresh: bool) -> Result<WeekData> {
    let path = cache.join("week_data.json");
    let raw_body = if path.exists() && !refresh {
        fs::read_to_string(&path)?
    } else {
        let body = get_with_retry(WEEKDATA_URL)?.context("Failed to fetch week data from API")?;
        fs::write(&path, &body).context("Failed to write week data cache file")?;
        body
    };

    const DAYS: [&str; 7] = ["sun", "mon", "tue", "wed", "thu", "fri", "sat"];
    const CHRONO_DAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    let day_to_index: BTreeMap<&str, usize> =
        DAYS.iter().enumerate().map(|(i, &d)| (d, i)).collect();

    let json: serde_json::Value =
        serde_json::from_str(&raw_body).context("Failed to parse week data JSON")?;

    let week = &json["supplemental"]["weekData"];
    let starts = week["weekendStart"]
        .as_object()
        .context("Missing weekendStart data")?;
    let ends = week["weekendEnd"]
        .as_object()
        .context("Missing weekendEnd data")?;
    let default_start = starts["001"]
        .as_str()
        .context("Missing default weekendStart")?;
    let default_end = ends["001"].as_str().context("Missing default weekendEnd")?;

    let mut week_data = BTreeMap::new();
    let mut codes: Vec<&String> = starts.keys().chain(ends.keys()).collect();
    codes.sort();
    codes.dedup();
    for code in codes {
        let start = starts
            .get(code)
            .and_then(|v| v.as_str())
            .unwrap_or(default_start);
        let end = ends
            .get(code)
            .and_then(|v| v.as_str())
            .unwrap_or(default_end);

        // Generate the list of weekend days for this country code
        let mut days = Vec::new();
        let mut index = *day_to_index
            .get(start)
            .context(format!("Invalid weekendStart day: {}", start))?;
        let end_index = *day_to_index
            .get(end)
            .context(format!("Invalid weekendEnd day: {}", end))?;

        loop {
            days.push(CHRONO_DAYS[index]);

            if index == end_index {
                break;
            }

            index = (index + 1) % 7; // Wrap around the week
        }

        week_data.insert(code.clone(), days);
    }

    Ok(WeekData(week_data))
}

fn is_retryable_status(status: reqwest::StatusCode) -> bool {
    matches!(
        status,
        reqwest::StatusCode::TOO_MANY_REQUESTS
            | reqwest::StatusCode::INTERNAL_SERVER_ERROR
            | reqwest::StatusCode::BAD_GATEWAY
            | reqwest::StatusCode::SERVICE_UNAVAILABLE
            | reqwest::StatusCode::GATEWAY_TIMEOUT
    )
}

fn should_retry(error: &reqwest::Error) -> bool {
    if error.is_timeout() || error.is_connect() || error.is_request() {
        return true;
    }

    if let Some(status) = error.status() {
        return is_retryable_status(status);
    }

    false
}

fn get_with_retry(url: &str) -> Result<Option<String>> {
    let mut delay = Duration::from_secs(1);

    for attempt in 1..=MAX_RETRIES {
        match reqwest::blocking::get(url) {
            Ok(response) if response.status() == reqwest::StatusCode::NO_CONTENT => {
                return Ok(None);
            }

            Ok(response) => return Ok(Some(response.text()?)),

            Err(e) => {
                if attempt == MAX_RETRIES || !should_retry(&e) {
                    return Err(anyhow::anyhow!(
                        "Request to {} failed after {} attempts: {}",
                        url,
                        MAX_RETRIES,
                        e
                    ));
                }

                eprintln!("Retrying {} failed with: {}", url, e);
                sleep(delay);
                delay *= 2; // Exponential backoff
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    Ok(None)
}

// Generate Rust code for a specific country based on cached holiday data.
fn generate_country(cache: &Path, data_dir: &Path, country: &CountryInfo) -> Result<bool> {
    let dir = cache.join(&country.country_code);
    if !dir.exists() {
        return Ok(false);
    }

    let mut holidays_by_year = BTreeMap::new();
    for year in START_YEAR..=chrono::Utc::now().year() as u32 + FUTURE_YEARS {
        let path = dir.join(format!("{year}.json"));
        if !path.exists() {
            continue;
        }

        let raw_body = fs::read_to_string(&path).with_context(|| {
            format!(
                "Failed to read cache file for {} in {}",
                country.country_code, year
            )
        })?;
        let holidays: Vec<ApiHoliday> = serde_json::from_str(&raw_body).with_context(|| {
            format!(
                "Failed to parse holidays JSON for {} in {}",
                country.country_code, year
            )
        })?;
        // Filter holidays to include only global public holidays
        holidays_by_year.insert(
            year,
            holidays
                .into_iter()
                .filter(|h| {
                    h.global
                        && h.types
                            .as_ref()
                            .map_or(true, |t| t.contains(&"Public".to_string()))
                })
                .collect::<Vec<_>>(),
        );
    }

    if holidays_by_year.is_empty() {
        return Ok(false);
    }

    let output_path = data_dir.join(format!("{}.rs", country.country_code.to_lowercase()));
    let mut output = String::new();

    output.push_str(&format!("//! {}\n", country.name));
    output.push_str("//\n");
    output.push_str("// This file is auto-generated. Do not edit manually.\n\n");
    output.push_str("use super::*;\n\n");
    output.push_str("pub fn build(years: Option<&Range<Year>>) -> Result<HolidayPerYearMap> {\n");
    output.push_str("    let mut map = HashMap::new();\n\n");

    for (year, holidays) in &holidays_by_year {
        output.push_str(&format!("    build_year(\n"));
        output.push_str(&format!("        years,\n"));
        output.push_str(&format!("        {year},\n"));
        output.push_str(&format!("        [\n"));

        for holiday in holidays {
            let local_name = holiday.local_name.as_deref().unwrap_or(&holiday.name);
            let (year, month, day) = parse_date(&holiday.date).with_context(|| {
                format!(
                    "Failed to parse date {} for {} in {}",
                    holiday.date, holiday.name, country.country_code
                )
            })?;

            output.push_str(&format!(
                "            (from_ymd_res({}, {}, {})?, \"{}\", \"{}\"),\n",
                year,
                month,
                day,
                local_name.replace("\"", "\\\""),
                holiday.name.replace("\"", "\\\"")
            ));
        }
        output.push_str(&format!("        ],\n"));
        output.push_str(&format!("        &mut map,\n"));
        output.push_str(&format!("        Country::{},\n", country.country_code));
        output.push_str(&format!("        \"{}\",\n", country.name));
        output.push_str(&format!("    );\n\n"));
    }

    output.push_str("    Ok(map)\n");
    output.push_str("}\n");

    fs::write(&output_path, output).with_context(|| {
        format!(
            "Failed to write generated file for {}",
            country.country_code
        )
    })?;

    rustfmt_file(&output_path).with_context(|| {
        format!(
            "Failed to format generated file for {}",
            country.country_code
        )
    })?;

    Ok(true)
}

fn parse_date(date_str: &str) -> Result<(u32, u32, u32)> {
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() != 3 {
        bail!("Invalid date format: {}", date_str);
    }

    let year = parts[0].parse::<u32>().context("Invalid year")?;
    let month = parts[1].parse::<u32>().context("Invalid month")?;
    let day = parts[2].parse::<u32>().context("Invalid day")?;

    Ok((year, month, day))
}

fn rustfmt_file(path: &Path) -> Result<()> {
    let status = Command::new("rustfmt")
        .arg(path)
        .status()
        .context("Failed to run rustfmt")?;

    if !status.success() {
        bail!("rustfmt failed on {}", path.display());
    }

    Ok(())
}

#[derive(Serialize)]
struct WeekDataEntry<'a> {
    country_code: &'a str,
    days: &'a [&'static str],
}

fn generate_country_enums(
    builder_dir: &Path,
    src_dir: &Path,
    countries: &Vec<&CountryInfo>,
    week_data: &WeekData,
) -> Result<()> {
    let output_path = src_dir.join("country.rs");
    let template_path = builder_dir.join("templates/country.rs.tera");
    let mut tera = Tera::new();
    tera.add_template_file(&template_path, Some("country.rs.tera"))
        .with_context(|| format!("Failed to load template {}", template_path.display()))?;

    let mut context = TeraContext::new();
    context.insert("countries_count", &countries.len());
    context.insert("countries", &countries);

    let country_codes: Vec<&String> = countries.iter().map(|c| &c.country_code).collect();
    let mut filtered_week_data: Vec<WeekDataEntry> = week_data
        .0
        .iter()
        .filter(|(k, _)| country_codes.contains(k))
        .map(|(code, days)| WeekDataEntry {
            country_code: code,
            days,
        })
        .collect();
    filtered_week_data.sort_by(|a, b| a.country_code.cmp(b.country_code));
    context.insert("week_data", &filtered_week_data);

    let rendered = tera
        .render("country.rs.tera", &context)
        .context("Failed to render country.rs template")?;
    fs::write(&output_path, rendered).context("Failed to write country.rs file")?;
    rustfmt_file(&output_path).with_context(|| {
        format!(
            "Failed to format generated country.rs file at {}",
            output_path.display()
        )
    })?;

    Ok(())
}

#[derive(Serialize)]
struct CountryCodeEntry<'a> {
    country_code: &'a str,
    country_code_ident: String,
}

fn generate_mod_file(
    builder_dir: &Path,
    src_dir: &Path,
    countries: &Vec<&CountryInfo>,
) -> Result<()> {
    let output_path = src_dir.join("data/mod.rs");
    let template_path = builder_dir.join("templates/mod.rs.tera");
    let mut tera = Tera::new();
    tera.add_template_file(&template_path, Some("mod.rs.tera"))
        .with_context(|| format!("Failed to load template {}", template_path.display()))?;

    let mut context = TeraContext::new();
    let mut country_codes_ident: Vec<CountryCodeEntry> = countries
        .iter()
        .map(|c| CountryCodeEntry {
            country_code: &c.country_code,
            country_code_ident: mod_ident(&c.country_code),
        })
        .collect();
    country_codes_ident.sort_by(|a, b| a.country_code.cmp(b.country_code));
    context.insert("country_codes", &country_codes_ident);

    let rendered = tera
        .render("mod.rs.tera", &context)
        .context("Failed to render mod.rs template")?;
    fs::write(&output_path, rendered).context("Failed to write mod.rs file")?;
    rustfmt_file(&output_path).with_context(|| {
        format!(
            "Failed to format generated mod.rs file at {}",
            output_path.display()
        )
    })?;

    Ok(())
}

fn update_cargo_toml(root_dir: &Path, countries: &Vec<&CountryInfo>) -> Result<()> {
    let cargo_toml_path = root_dir.join("Cargo.toml");
    let mut cargo_toml_content =
        fs::read_to_string(&cargo_toml_path).context("Failed to read Cargo.toml")?;

    // Remove existing features section
    if let Some(start) = cargo_toml_content.find("[features]") {
        let end = cargo_toml_content[start..]
            .find("\n[")
            .map(|e| start + e)
            .unwrap_or(cargo_toml_content.len());
        cargo_toml_content.replace_range(start..end, "");
    }

    // Append new features section
    cargo_toml_content.push_str("[features]\n");
    cargo_toml_content.push_str("# This section is auto-generated. Do not edit manually.\n");
    cargo_toml_content.push_str("default = [\"all\"]\n");
    cargo_toml_content.push_str("all = [");
    cargo_toml_content.push_str(
        &countries
            .iter()
            .map(|c| format!("\"{}\"", c.country_code.to_lowercase()))
            .collect::<Vec<String>>()
            .join(", "),
    );
    cargo_toml_content.push_str("]\n");
    for country in countries {
        cargo_toml_content.push_str(&format!("{} = []\n", country.country_code.to_lowercase()));
    }

    fs::write(&cargo_toml_path, cargo_toml_content).context("Failed to write Cargo.toml")?;
    Ok(())
}

/// Module identifier, raw-prefixed when the code is a Rust keyword (`IN` -> `r#in`).
fn mod_ident(code: &str) -> String {
    if RUST_KEYWORDS.contains(&code.to_lowercase().as_str()) {
        format!("r#{code}")
    } else {
        code.to_string()
    }
}
