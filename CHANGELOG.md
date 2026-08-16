# Changelog

All notable changes to this project will be documented in this file.
## [0.1.0] - 2026-08-16

### 🚀 Features

- *(calendar)* Initial support of calendar logic
- *(builder)* Load available countries list
- *(builder)* Validate before caching so a truncated response never poisons the cache
- *(builder)* Ability to force refetching data even if cache exists
- *(builder)* Weekend days loader for each country
- *(builder)* Generate country holidays data
- *(builder)* Generate country enum module
- *(builder)* Weekdata builder
- *(builder)* Generate mod.rs with all generated countries
- *(builder)* Updater for Cargo.toml features section
- *(data)* Update data with holidays data for all available countries
- *(calendar)* Business days between function
- *(calendar)* Extended business days calculations
- *(builder)* Added possibility to change start year

### 🐛 Bug Fixes

- *(builder)* Filter holidays to only global & public
- *(builder)* Mod generator does not escaped country codes, that collide with rust keywords
- *(builder)* Sort country codes in generated mods alphabetically

### ⚙️ Miscellaneous Tasks

- *(tests)* Skip tests for excluded countries
- *(tests)* Better test coverage, depending of features configuration
- Added LICENSE file
- Updated README
- Fixed formatting
- Fixed typos in tests
- Added github actions for CI checks
- More tests
- Updated GitHub checkout action to v5
- Add git-cliff config
- Correct crate keywords
- Rename crate
- Fixed formatting
