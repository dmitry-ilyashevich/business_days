use std::fs;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use chrono::Datelike;
use clap::Parser;
use serde::Deserialize;

// V4 have not holidays names in local languages, so we use V3 for fetching
// holidays and V4 for fetching countries list
const API_V3_URL: &str = "https://nagerholidays.com/api/v3";
const API_V4_URL: &str = "https://nagerholidays.com/api/v4";

const MAX_RETRIES: u32 = 5;

const START_YEAR: u32 = 2000;
const FUTURE_YEARS: u32 = 5;

const REQUEST_DELAY: Duration = Duration::from_millis(150); // milliseconds

#[derive(Deserialize, Clone)]
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
}

fn main() -> Result<()> {
    let args = Args::parse();

    let builder_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cache = builder_dir.join("cache");

    let current_year = chrono::Utc::now().year() as u32;
    let end_year = args.end_year.unwrap_or(current_year + FUTURE_YEARS);

    let countries = load_countries(&cache)?;

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

    // Fetch countries info
    for country in &countries {
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
            if path.exists() {
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

    Ok(())
}

fn load_countries(cache: &Path) -> Result<Vec<CountryInfo>> {
    let path = cache.join("countries.json");
    let raw_body = if path.exists() {
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
