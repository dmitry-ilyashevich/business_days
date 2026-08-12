// This file is generated automatically. Do not edit manually.

use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Country {
    /// Andorra
    AD,

    /// Antigua and Barbuda
    AG,

    /// Anguilla
    AI,

    /// Albania
    AL,

    /// Armenia
    AM,

    /// Angola
    AO,

    /// Argentina
    AR,

    /// Austria
    AT,

    /// Australia
    AU,

    /// Aruba
    AW,

    /// Åland Islands
    AX,

    /// Bosnia and Herzegovina
    BA,

    /// Barbados
    BB,

    /// Bangladesh
    BD,

    /// Belgium
    BE,

    /// Burkina Faso
    BF,

    /// Bulgaria
    BG,

    /// Bahrain
    BH,

    /// Burundi
    BI,

    /// Benin
    BJ,

    /// Saint Barthélemy
    BL,

    /// Bermuda
    BM,

    /// Bolivia
    BO,

    /// Caribbean Netherlands
    BQ,

    /// Brazil
    BR,

    /// Bahamas
    BS,

    /// Botswana
    BW,

    /// Belarus
    BY,

    /// Belize
    BZ,

    /// Canada
    CA,

    /// Cocos (Keeling) Islands
    CC,

    /// DR Congo
    CD,

    /// Central African Republic
    CF,

    /// Congo
    CG,

    /// Switzerland
    CH,

    /// Ivory Coast
    CI,

    /// Cook Islands
    CK,

    /// Chile
    CL,

    /// Cameroon
    CM,

    /// China
    CN,

    /// Colombia
    CO,

    /// Costa Rica
    CR,

    /// Cuba
    CU,

    /// Cape Verde
    CV,

    /// Curaçao
    CW,

    /// Christmas Island
    CX,

    /// Cyprus
    CY,

    /// Czechia
    CZ,

    /// Germany
    DE,

    /// Djibouti
    DJ,

    /// Denmark
    DK,

    /// Dominica
    DM,

    /// Dominican Republic
    DO,

    /// Algeria
    DZ,

    /// Ecuador
    EC,

    /// Estonia
    EE,

    /// Egypt
    EG,

    /// Eritrea
    ER,

    /// Spain
    ES,

    /// Ethiopia
    ET,

    /// Finland
    FI,

    /// Falkland Islands
    FK,

    /// Micronesia
    FM,

    /// Faroe Islands
    FO,

    /// France
    FR,

    /// Gabon
    GA,

    /// United Kingdom
    GB,

    /// Grenada
    GD,

    /// Georgia
    GE,

    /// French Guiana
    GF,

    /// Guernsey
    GG,

    /// Ghana
    GH,

    /// Gibraltar
    GI,

    /// Greenland
    GL,

    /// Gambia
    GM,

    /// Guinea
    GN,

    /// Guadeloupe
    GP,

    /// Equatorial Guinea
    GQ,

    /// Greece
    GR,

    /// Guatemala
    GT,

    /// Guinea-Bissau
    GW,

    /// Guyana
    GY,

    /// Hong Kong
    HK,

    /// Honduras
    HN,

    /// Croatia
    HR,

    /// Haiti
    HT,

    /// Hungary
    HU,

    /// Indonesia
    ID,

    /// Ireland
    IE,

    /// Isle of Man
    IM,

    /// Iraq
    IQ,

    /// Iceland
    IS,

    /// Italy
    IT,

    /// Jersey
    JE,

    /// Jamaica
    JM,

    /// Japan
    JP,

    /// Kenya
    KE,

    /// Cambodia
    KH,

    /// Kiribati
    KI,

    /// Comoros
    KM,

    /// Saint Kitts and Nevis
    KN,

    /// South Korea
    KR,

    /// Cayman Islands
    KY,

    /// Kazakhstan
    KZ,

    /// Saint Lucia
    LC,

    /// Liechtenstein
    LI,

    /// Liberia
    LR,

    /// Lesotho
    LS,

    /// Lithuania
    LT,

    /// Luxembourg
    LU,

    /// Latvia
    LV,

    /// Libya
    LY,

    /// Morocco
    MA,

    /// Monaco
    MC,

    /// Moldova
    MD,

    /// Montenegro
    ME,

    /// Saint Martin
    MF,

    /// Madagascar
    MG,

    /// Marshall Islands
    MH,

    /// North Macedonia
    MK,

    /// Mali
    ML,

    /// Mongolia
    MN,

    /// Northern Mariana Islands
    MP,

    /// Martinique
    MQ,

    /// Mauritania
    MR,

    /// Montserrat
    MS,

    /// Malta
    MT,

    /// Malawi
    MW,

    /// Mexico
    MX,

    /// Mozambique
    MZ,

    /// Namibia
    NA,

    /// New Caledonia
    NC,

    /// Niger
    NE,

    /// Norfolk Island
    NF,

    /// Nigeria
    NG,

    /// Nicaragua
    NI,

    /// Netherlands
    NL,

    /// Norway
    NO,

    /// Nauru
    NR,

    /// Niue
    NU,

    /// New Zealand
    NZ,

    /// Panama
    PA,

    /// Peru
    PE,

    /// French Polynesia
    PF,

    /// Papua New Guinea
    PG,

    /// Philippines
    PH,

    /// Poland
    PL,

    /// Saint Pierre and Miquelon
    PM,

    /// Pitcairn Islands
    PN,

    /// Puerto Rico
    PR,

    /// Portugal
    PT,

    /// Palau
    PW,

    /// Paraguay
    PY,

    /// Romania
    RO,

    /// Serbia
    RS,

    /// Russia
    RU,

    /// Rwanda
    RW,

    /// Solomon Islands
    SB,

    /// Seychelles
    SC,

    /// Sudan
    SD,

    /// Sweden
    SE,

    /// Singapore
    SG,

    /// Saint Helena, Ascension and Tristan da Cunha
    SH,

    /// Slovenia
    SI,

    /// Svalbard and Jan Mayen
    SJ,

    /// Slovakia
    SK,

    /// Sierra Leone
    SL,

    /// San Marino
    SM,

    /// Senegal
    SN,

    /// Somalia
    SO,

    /// Suriname
    SR,

    /// South Sudan
    SS,

    /// São Tomé and Príncipe
    ST,

    /// El Salvador
    SV,

    /// Sint Maarten
    SX,

    /// Syria
    SY,

    /// Eswatini
    SZ,

    /// Turks and Caicos Islands
    TC,

    /// Chad
    TD,

    /// Togo
    TG,

    /// Tokelau
    TK,

    /// Tunisia
    TN,

    /// Tonga
    TO,

    /// Türkiye
    TR,

    /// Trinidad and Tobago
    TT,

    /// Tuvalu
    TV,

    /// Tanzania
    TZ,

    /// Ukraine
    UA,

    /// Uganda
    UG,

    /// United States
    US,

    /// Uruguay
    UY,

    /// Vatican City
    VA,

    /// Saint Vincent and the Grenadines
    VC,

    /// Venezuela
    VE,

    /// British Virgin Islands
    VG,

    /// United States Virgin Islands
    VI,

    /// Vietnam
    VN,

    /// Vanuatu
    VU,

    /// Wallis and Futuna
    WF,

    /// Samoa
    WS,

    /// Yemen
    YE,

    /// South Africa
    ZA,

    /// Zambia
    ZM,

    /// Zimbabwe
    ZW,
}

impl Country {
    pub const ALL: [Country; 204] = [
        Country::AD,
        Country::AG,
        Country::AI,
        Country::AL,
        Country::AM,
        Country::AO,
        Country::AR,
        Country::AT,
        Country::AU,
        Country::AW,
        Country::AX,
        Country::BA,
        Country::BB,
        Country::BD,
        Country::BE,
        Country::BF,
        Country::BG,
        Country::BH,
        Country::BI,
        Country::BJ,
        Country::BL,
        Country::BM,
        Country::BO,
        Country::BQ,
        Country::BR,
        Country::BS,
        Country::BW,
        Country::BY,
        Country::BZ,
        Country::CA,
        Country::CC,
        Country::CD,
        Country::CF,
        Country::CG,
        Country::CH,
        Country::CI,
        Country::CK,
        Country::CL,
        Country::CM,
        Country::CN,
        Country::CO,
        Country::CR,
        Country::CU,
        Country::CV,
        Country::CW,
        Country::CX,
        Country::CY,
        Country::CZ,
        Country::DE,
        Country::DJ,
        Country::DK,
        Country::DM,
        Country::DO,
        Country::DZ,
        Country::EC,
        Country::EE,
        Country::EG,
        Country::ER,
        Country::ES,
        Country::ET,
        Country::FI,
        Country::FK,
        Country::FM,
        Country::FO,
        Country::FR,
        Country::GA,
        Country::GB,
        Country::GD,
        Country::GE,
        Country::GF,
        Country::GG,
        Country::GH,
        Country::GI,
        Country::GL,
        Country::GM,
        Country::GN,
        Country::GP,
        Country::GQ,
        Country::GR,
        Country::GT,
        Country::GW,
        Country::GY,
        Country::HK,
        Country::HN,
        Country::HR,
        Country::HT,
        Country::HU,
        Country::ID,
        Country::IE,
        Country::IM,
        Country::IQ,
        Country::IS,
        Country::IT,
        Country::JE,
        Country::JM,
        Country::JP,
        Country::KE,
        Country::KH,
        Country::KI,
        Country::KM,
        Country::KN,
        Country::KR,
        Country::KY,
        Country::KZ,
        Country::LC,
        Country::LI,
        Country::LR,
        Country::LS,
        Country::LT,
        Country::LU,
        Country::LV,
        Country::LY,
        Country::MA,
        Country::MC,
        Country::MD,
        Country::ME,
        Country::MF,
        Country::MG,
        Country::MH,
        Country::MK,
        Country::ML,
        Country::MN,
        Country::MP,
        Country::MQ,
        Country::MR,
        Country::MS,
        Country::MT,
        Country::MW,
        Country::MX,
        Country::MZ,
        Country::NA,
        Country::NC,
        Country::NE,
        Country::NF,
        Country::NG,
        Country::NI,
        Country::NL,
        Country::NO,
        Country::NR,
        Country::NU,
        Country::NZ,
        Country::PA,
        Country::PE,
        Country::PF,
        Country::PG,
        Country::PH,
        Country::PL,
        Country::PM,
        Country::PN,
        Country::PR,
        Country::PT,
        Country::PW,
        Country::PY,
        Country::RO,
        Country::RS,
        Country::RU,
        Country::RW,
        Country::SB,
        Country::SC,
        Country::SD,
        Country::SE,
        Country::SG,
        Country::SH,
        Country::SI,
        Country::SJ,
        Country::SK,
        Country::SL,
        Country::SM,
        Country::SN,
        Country::SO,
        Country::SR,
        Country::SS,
        Country::ST,
        Country::SV,
        Country::SX,
        Country::SY,
        Country::SZ,
        Country::TC,
        Country::TD,
        Country::TG,
        Country::TK,
        Country::TN,
        Country::TO,
        Country::TR,
        Country::TT,
        Country::TV,
        Country::TZ,
        Country::UA,
        Country::UG,
        Country::US,
        Country::UY,
        Country::VA,
        Country::VC,
        Country::VE,
        Country::VG,
        Country::VI,
        Country::VN,
        Country::VU,
        Country::WF,
        Country::WS,
        Country::YE,
        Country::ZA,
        Country::ZM,
        Country::ZW,
    ];

    pub fn code(&self) -> &'static str {
        match self {
            Country::AD => "AD",
            Country::AG => "AG",
            Country::AI => "AI",
            Country::AL => "AL",
            Country::AM => "AM",
            Country::AO => "AO",
            Country::AR => "AR",
            Country::AT => "AT",
            Country::AU => "AU",
            Country::AW => "AW",
            Country::AX => "AX",
            Country::BA => "BA",
            Country::BB => "BB",
            Country::BD => "BD",
            Country::BE => "BE",
            Country::BF => "BF",
            Country::BG => "BG",
            Country::BH => "BH",
            Country::BI => "BI",
            Country::BJ => "BJ",
            Country::BL => "BL",
            Country::BM => "BM",
            Country::BO => "BO",
            Country::BQ => "BQ",
            Country::BR => "BR",
            Country::BS => "BS",
            Country::BW => "BW",
            Country::BY => "BY",
            Country::BZ => "BZ",
            Country::CA => "CA",
            Country::CC => "CC",
            Country::CD => "CD",
            Country::CF => "CF",
            Country::CG => "CG",
            Country::CH => "CH",
            Country::CI => "CI",
            Country::CK => "CK",
            Country::CL => "CL",
            Country::CM => "CM",
            Country::CN => "CN",
            Country::CO => "CO",
            Country::CR => "CR",
            Country::CU => "CU",
            Country::CV => "CV",
            Country::CW => "CW",
            Country::CX => "CX",
            Country::CY => "CY",
            Country::CZ => "CZ",
            Country::DE => "DE",
            Country::DJ => "DJ",
            Country::DK => "DK",
            Country::DM => "DM",
            Country::DO => "DO",
            Country::DZ => "DZ",
            Country::EC => "EC",
            Country::EE => "EE",
            Country::EG => "EG",
            Country::ER => "ER",
            Country::ES => "ES",
            Country::ET => "ET",
            Country::FI => "FI",
            Country::FK => "FK",
            Country::FM => "FM",
            Country::FO => "FO",
            Country::FR => "FR",
            Country::GA => "GA",
            Country::GB => "GB",
            Country::GD => "GD",
            Country::GE => "GE",
            Country::GF => "GF",
            Country::GG => "GG",
            Country::GH => "GH",
            Country::GI => "GI",
            Country::GL => "GL",
            Country::GM => "GM",
            Country::GN => "GN",
            Country::GP => "GP",
            Country::GQ => "GQ",
            Country::GR => "GR",
            Country::GT => "GT",
            Country::GW => "GW",
            Country::GY => "GY",
            Country::HK => "HK",
            Country::HN => "HN",
            Country::HR => "HR",
            Country::HT => "HT",
            Country::HU => "HU",
            Country::ID => "ID",
            Country::IE => "IE",
            Country::IM => "IM",
            Country::IQ => "IQ",
            Country::IS => "IS",
            Country::IT => "IT",
            Country::JE => "JE",
            Country::JM => "JM",
            Country::JP => "JP",
            Country::KE => "KE",
            Country::KH => "KH",
            Country::KI => "KI",
            Country::KM => "KM",
            Country::KN => "KN",
            Country::KR => "KR",
            Country::KY => "KY",
            Country::KZ => "KZ",
            Country::LC => "LC",
            Country::LI => "LI",
            Country::LR => "LR",
            Country::LS => "LS",
            Country::LT => "LT",
            Country::LU => "LU",
            Country::LV => "LV",
            Country::LY => "LY",
            Country::MA => "MA",
            Country::MC => "MC",
            Country::MD => "MD",
            Country::ME => "ME",
            Country::MF => "MF",
            Country::MG => "MG",
            Country::MH => "MH",
            Country::MK => "MK",
            Country::ML => "ML",
            Country::MN => "MN",
            Country::MP => "MP",
            Country::MQ => "MQ",
            Country::MR => "MR",
            Country::MS => "MS",
            Country::MT => "MT",
            Country::MW => "MW",
            Country::MX => "MX",
            Country::MZ => "MZ",
            Country::NA => "NA",
            Country::NC => "NC",
            Country::NE => "NE",
            Country::NF => "NF",
            Country::NG => "NG",
            Country::NI => "NI",
            Country::NL => "NL",
            Country::NO => "NO",
            Country::NR => "NR",
            Country::NU => "NU",
            Country::NZ => "NZ",
            Country::PA => "PA",
            Country::PE => "PE",
            Country::PF => "PF",
            Country::PG => "PG",
            Country::PH => "PH",
            Country::PL => "PL",
            Country::PM => "PM",
            Country::PN => "PN",
            Country::PR => "PR",
            Country::PT => "PT",
            Country::PW => "PW",
            Country::PY => "PY",
            Country::RO => "RO",
            Country::RS => "RS",
            Country::RU => "RU",
            Country::RW => "RW",
            Country::SB => "SB",
            Country::SC => "SC",
            Country::SD => "SD",
            Country::SE => "SE",
            Country::SG => "SG",
            Country::SH => "SH",
            Country::SI => "SI",
            Country::SJ => "SJ",
            Country::SK => "SK",
            Country::SL => "SL",
            Country::SM => "SM",
            Country::SN => "SN",
            Country::SO => "SO",
            Country::SR => "SR",
            Country::SS => "SS",
            Country::ST => "ST",
            Country::SV => "SV",
            Country::SX => "SX",
            Country::SY => "SY",
            Country::SZ => "SZ",
            Country::TC => "TC",
            Country::TD => "TD",
            Country::TG => "TG",
            Country::TK => "TK",
            Country::TN => "TN",
            Country::TO => "TO",
            Country::TR => "TR",
            Country::TT => "TT",
            Country::TV => "TV",
            Country::TZ => "TZ",
            Country::UA => "UA",
            Country::UG => "UG",
            Country::US => "US",
            Country::UY => "UY",
            Country::VA => "VA",
            Country::VC => "VC",
            Country::VE => "VE",
            Country::VG => "VG",
            Country::VI => "VI",
            Country::VN => "VN",
            Country::VU => "VU",
            Country::WF => "WF",
            Country::WS => "WS",
            Country::YE => "YE",
            Country::ZA => "ZA",
            Country::ZM => "ZM",
            Country::ZW => "ZW",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Country::AD => "Andorra",
            Country::AG => "Antigua and Barbuda",
            Country::AI => "Anguilla",
            Country::AL => "Albania",
            Country::AM => "Armenia",
            Country::AO => "Angola",
            Country::AR => "Argentina",
            Country::AT => "Austria",
            Country::AU => "Australia",
            Country::AW => "Aruba",
            Country::AX => "Åland Islands",
            Country::BA => "Bosnia and Herzegovina",
            Country::BB => "Barbados",
            Country::BD => "Bangladesh",
            Country::BE => "Belgium",
            Country::BF => "Burkina Faso",
            Country::BG => "Bulgaria",
            Country::BH => "Bahrain",
            Country::BI => "Burundi",
            Country::BJ => "Benin",
            Country::BL => "Saint Barthélemy",
            Country::BM => "Bermuda",
            Country::BO => "Bolivia",
            Country::BQ => "Caribbean Netherlands",
            Country::BR => "Brazil",
            Country::BS => "Bahamas",
            Country::BW => "Botswana",
            Country::BY => "Belarus",
            Country::BZ => "Belize",
            Country::CA => "Canada",
            Country::CC => "Cocos (Keeling) Islands",
            Country::CD => "DR Congo",
            Country::CF => "Central African Republic",
            Country::CG => "Congo",
            Country::CH => "Switzerland",
            Country::CI => "Ivory Coast",
            Country::CK => "Cook Islands",
            Country::CL => "Chile",
            Country::CM => "Cameroon",
            Country::CN => "China",
            Country::CO => "Colombia",
            Country::CR => "Costa Rica",
            Country::CU => "Cuba",
            Country::CV => "Cape Verde",
            Country::CW => "Curaçao",
            Country::CX => "Christmas Island",
            Country::CY => "Cyprus",
            Country::CZ => "Czechia",
            Country::DE => "Germany",
            Country::DJ => "Djibouti",
            Country::DK => "Denmark",
            Country::DM => "Dominica",
            Country::DO => "Dominican Republic",
            Country::DZ => "Algeria",
            Country::EC => "Ecuador",
            Country::EE => "Estonia",
            Country::EG => "Egypt",
            Country::ER => "Eritrea",
            Country::ES => "Spain",
            Country::ET => "Ethiopia",
            Country::FI => "Finland",
            Country::FK => "Falkland Islands",
            Country::FM => "Micronesia",
            Country::FO => "Faroe Islands",
            Country::FR => "France",
            Country::GA => "Gabon",
            Country::GB => "United Kingdom",
            Country::GD => "Grenada",
            Country::GE => "Georgia",
            Country::GF => "French Guiana",
            Country::GG => "Guernsey",
            Country::GH => "Ghana",
            Country::GI => "Gibraltar",
            Country::GL => "Greenland",
            Country::GM => "Gambia",
            Country::GN => "Guinea",
            Country::GP => "Guadeloupe",
            Country::GQ => "Equatorial Guinea",
            Country::GR => "Greece",
            Country::GT => "Guatemala",
            Country::GW => "Guinea-Bissau",
            Country::GY => "Guyana",
            Country::HK => "Hong Kong",
            Country::HN => "Honduras",
            Country::HR => "Croatia",
            Country::HT => "Haiti",
            Country::HU => "Hungary",
            Country::ID => "Indonesia",
            Country::IE => "Ireland",
            Country::IM => "Isle of Man",
            Country::IQ => "Iraq",
            Country::IS => "Iceland",
            Country::IT => "Italy",
            Country::JE => "Jersey",
            Country::JM => "Jamaica",
            Country::JP => "Japan",
            Country::KE => "Kenya",
            Country::KH => "Cambodia",
            Country::KI => "Kiribati",
            Country::KM => "Comoros",
            Country::KN => "Saint Kitts and Nevis",
            Country::KR => "South Korea",
            Country::KY => "Cayman Islands",
            Country::KZ => "Kazakhstan",
            Country::LC => "Saint Lucia",
            Country::LI => "Liechtenstein",
            Country::LR => "Liberia",
            Country::LS => "Lesotho",
            Country::LT => "Lithuania",
            Country::LU => "Luxembourg",
            Country::LV => "Latvia",
            Country::LY => "Libya",
            Country::MA => "Morocco",
            Country::MC => "Monaco",
            Country::MD => "Moldova",
            Country::ME => "Montenegro",
            Country::MF => "Saint Martin",
            Country::MG => "Madagascar",
            Country::MH => "Marshall Islands",
            Country::MK => "North Macedonia",
            Country::ML => "Mali",
            Country::MN => "Mongolia",
            Country::MP => "Northern Mariana Islands",
            Country::MQ => "Martinique",
            Country::MR => "Mauritania",
            Country::MS => "Montserrat",
            Country::MT => "Malta",
            Country::MW => "Malawi",
            Country::MX => "Mexico",
            Country::MZ => "Mozambique",
            Country::NA => "Namibia",
            Country::NC => "New Caledonia",
            Country::NE => "Niger",
            Country::NF => "Norfolk Island",
            Country::NG => "Nigeria",
            Country::NI => "Nicaragua",
            Country::NL => "Netherlands",
            Country::NO => "Norway",
            Country::NR => "Nauru",
            Country::NU => "Niue",
            Country::NZ => "New Zealand",
            Country::PA => "Panama",
            Country::PE => "Peru",
            Country::PF => "French Polynesia",
            Country::PG => "Papua New Guinea",
            Country::PH => "Philippines",
            Country::PL => "Poland",
            Country::PM => "Saint Pierre and Miquelon",
            Country::PN => "Pitcairn Islands",
            Country::PR => "Puerto Rico",
            Country::PT => "Portugal",
            Country::PW => "Palau",
            Country::PY => "Paraguay",
            Country::RO => "Romania",
            Country::RS => "Serbia",
            Country::RU => "Russia",
            Country::RW => "Rwanda",
            Country::SB => "Solomon Islands",
            Country::SC => "Seychelles",
            Country::SD => "Sudan",
            Country::SE => "Sweden",
            Country::SG => "Singapore",
            Country::SH => "Saint Helena, Ascension and Tristan da Cunha",
            Country::SI => "Slovenia",
            Country::SJ => "Svalbard and Jan Mayen",
            Country::SK => "Slovakia",
            Country::SL => "Sierra Leone",
            Country::SM => "San Marino",
            Country::SN => "Senegal",
            Country::SO => "Somalia",
            Country::SR => "Suriname",
            Country::SS => "South Sudan",
            Country::ST => "São Tomé and Príncipe",
            Country::SV => "El Salvador",
            Country::SX => "Sint Maarten",
            Country::SY => "Syria",
            Country::SZ => "Eswatini",
            Country::TC => "Turks and Caicos Islands",
            Country::TD => "Chad",
            Country::TG => "Togo",
            Country::TK => "Tokelau",
            Country::TN => "Tunisia",
            Country::TO => "Tonga",
            Country::TR => "Türkiye",
            Country::TT => "Trinidad and Tobago",
            Country::TV => "Tuvalu",
            Country::TZ => "Tanzania",
            Country::UA => "Ukraine",
            Country::UG => "Uganda",
            Country::US => "United States",
            Country::UY => "Uruguay",
            Country::VA => "Vatican City",
            Country::VC => "Saint Vincent and the Grenadines",
            Country::VE => "Venezuela",
            Country::VG => "British Virgin Islands",
            Country::VI => "United States Virgin Islands",
            Country::VN => "Vietnam",
            Country::VU => "Vanuatu",
            Country::WF => "Wallis and Futuna",
            Country::WS => "Samoa",
            Country::YE => "Yemen",
            Country::ZA => "South Africa",
            Country::ZM => "Zambia",
            Country::ZW => "Zimbabwe",
        }
    }

    pub fn weekend(&self) -> &'static [chrono::Weekday] {
        use chrono::Weekday::*;

        #[allow(clippy::match_single_binding)]
        match self {
            Country::SD => &[Fri, Sat],
            Country::DZ => &[Fri, Sat],
            Country::EG => &[Fri, Sat],
            Country::SY => &[Fri, Sat],
            Country::YE => &[Fri, Sat],
            Country::BH => &[Fri, Sat],
            Country::IQ => &[Fri, Sat],
            Country::LY => &[Fri, Sat],
            Country::UG => &[Sun],

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
            "AD" => Ok(Country::AD),
            "AG" => Ok(Country::AG),
            "AI" => Ok(Country::AI),
            "AL" => Ok(Country::AL),
            "AM" => Ok(Country::AM),
            "AO" => Ok(Country::AO),
            "AR" => Ok(Country::AR),
            "AT" => Ok(Country::AT),
            "AU" => Ok(Country::AU),
            "AW" => Ok(Country::AW),
            "AX" => Ok(Country::AX),
            "BA" => Ok(Country::BA),
            "BB" => Ok(Country::BB),
            "BD" => Ok(Country::BD),
            "BE" => Ok(Country::BE),
            "BF" => Ok(Country::BF),
            "BG" => Ok(Country::BG),
            "BH" => Ok(Country::BH),
            "BI" => Ok(Country::BI),
            "BJ" => Ok(Country::BJ),
            "BL" => Ok(Country::BL),
            "BM" => Ok(Country::BM),
            "BO" => Ok(Country::BO),
            "BQ" => Ok(Country::BQ),
            "BR" => Ok(Country::BR),
            "BS" => Ok(Country::BS),
            "BW" => Ok(Country::BW),
            "BY" => Ok(Country::BY),
            "BZ" => Ok(Country::BZ),
            "CA" => Ok(Country::CA),
            "CC" => Ok(Country::CC),
            "CD" => Ok(Country::CD),
            "CF" => Ok(Country::CF),
            "CG" => Ok(Country::CG),
            "CH" => Ok(Country::CH),
            "CI" => Ok(Country::CI),
            "CK" => Ok(Country::CK),
            "CL" => Ok(Country::CL),
            "CM" => Ok(Country::CM),
            "CN" => Ok(Country::CN),
            "CO" => Ok(Country::CO),
            "CR" => Ok(Country::CR),
            "CU" => Ok(Country::CU),
            "CV" => Ok(Country::CV),
            "CW" => Ok(Country::CW),
            "CX" => Ok(Country::CX),
            "CY" => Ok(Country::CY),
            "CZ" => Ok(Country::CZ),
            "DE" => Ok(Country::DE),
            "DJ" => Ok(Country::DJ),
            "DK" => Ok(Country::DK),
            "DM" => Ok(Country::DM),
            "DO" => Ok(Country::DO),
            "DZ" => Ok(Country::DZ),
            "EC" => Ok(Country::EC),
            "EE" => Ok(Country::EE),
            "EG" => Ok(Country::EG),
            "ER" => Ok(Country::ER),
            "ES" => Ok(Country::ES),
            "ET" => Ok(Country::ET),
            "FI" => Ok(Country::FI),
            "FK" => Ok(Country::FK),
            "FM" => Ok(Country::FM),
            "FO" => Ok(Country::FO),
            "FR" => Ok(Country::FR),
            "GA" => Ok(Country::GA),
            "GB" => Ok(Country::GB),
            "GD" => Ok(Country::GD),
            "GE" => Ok(Country::GE),
            "GF" => Ok(Country::GF),
            "GG" => Ok(Country::GG),
            "GH" => Ok(Country::GH),
            "GI" => Ok(Country::GI),
            "GL" => Ok(Country::GL),
            "GM" => Ok(Country::GM),
            "GN" => Ok(Country::GN),
            "GP" => Ok(Country::GP),
            "GQ" => Ok(Country::GQ),
            "GR" => Ok(Country::GR),
            "GT" => Ok(Country::GT),
            "GW" => Ok(Country::GW),
            "GY" => Ok(Country::GY),
            "HK" => Ok(Country::HK),
            "HN" => Ok(Country::HN),
            "HR" => Ok(Country::HR),
            "HT" => Ok(Country::HT),
            "HU" => Ok(Country::HU),
            "ID" => Ok(Country::ID),
            "IE" => Ok(Country::IE),
            "IM" => Ok(Country::IM),
            "IQ" => Ok(Country::IQ),
            "IS" => Ok(Country::IS),
            "IT" => Ok(Country::IT),
            "JE" => Ok(Country::JE),
            "JM" => Ok(Country::JM),
            "JP" => Ok(Country::JP),
            "KE" => Ok(Country::KE),
            "KH" => Ok(Country::KH),
            "KI" => Ok(Country::KI),
            "KM" => Ok(Country::KM),
            "KN" => Ok(Country::KN),
            "KR" => Ok(Country::KR),
            "KY" => Ok(Country::KY),
            "KZ" => Ok(Country::KZ),
            "LC" => Ok(Country::LC),
            "LI" => Ok(Country::LI),
            "LR" => Ok(Country::LR),
            "LS" => Ok(Country::LS),
            "LT" => Ok(Country::LT),
            "LU" => Ok(Country::LU),
            "LV" => Ok(Country::LV),
            "LY" => Ok(Country::LY),
            "MA" => Ok(Country::MA),
            "MC" => Ok(Country::MC),
            "MD" => Ok(Country::MD),
            "ME" => Ok(Country::ME),
            "MF" => Ok(Country::MF),
            "MG" => Ok(Country::MG),
            "MH" => Ok(Country::MH),
            "MK" => Ok(Country::MK),
            "ML" => Ok(Country::ML),
            "MN" => Ok(Country::MN),
            "MP" => Ok(Country::MP),
            "MQ" => Ok(Country::MQ),
            "MR" => Ok(Country::MR),
            "MS" => Ok(Country::MS),
            "MT" => Ok(Country::MT),
            "MW" => Ok(Country::MW),
            "MX" => Ok(Country::MX),
            "MZ" => Ok(Country::MZ),
            "NA" => Ok(Country::NA),
            "NC" => Ok(Country::NC),
            "NE" => Ok(Country::NE),
            "NF" => Ok(Country::NF),
            "NG" => Ok(Country::NG),
            "NI" => Ok(Country::NI),
            "NL" => Ok(Country::NL),
            "NO" => Ok(Country::NO),
            "NR" => Ok(Country::NR),
            "NU" => Ok(Country::NU),
            "NZ" => Ok(Country::NZ),
            "PA" => Ok(Country::PA),
            "PE" => Ok(Country::PE),
            "PF" => Ok(Country::PF),
            "PG" => Ok(Country::PG),
            "PH" => Ok(Country::PH),
            "PL" => Ok(Country::PL),
            "PM" => Ok(Country::PM),
            "PN" => Ok(Country::PN),
            "PR" => Ok(Country::PR),
            "PT" => Ok(Country::PT),
            "PW" => Ok(Country::PW),
            "PY" => Ok(Country::PY),
            "RO" => Ok(Country::RO),
            "RS" => Ok(Country::RS),
            "RU" => Ok(Country::RU),
            "RW" => Ok(Country::RW),
            "SB" => Ok(Country::SB),
            "SC" => Ok(Country::SC),
            "SD" => Ok(Country::SD),
            "SE" => Ok(Country::SE),
            "SG" => Ok(Country::SG),
            "SH" => Ok(Country::SH),
            "SI" => Ok(Country::SI),
            "SJ" => Ok(Country::SJ),
            "SK" => Ok(Country::SK),
            "SL" => Ok(Country::SL),
            "SM" => Ok(Country::SM),
            "SN" => Ok(Country::SN),
            "SO" => Ok(Country::SO),
            "SR" => Ok(Country::SR),
            "SS" => Ok(Country::SS),
            "ST" => Ok(Country::ST),
            "SV" => Ok(Country::SV),
            "SX" => Ok(Country::SX),
            "SY" => Ok(Country::SY),
            "SZ" => Ok(Country::SZ),
            "TC" => Ok(Country::TC),
            "TD" => Ok(Country::TD),
            "TG" => Ok(Country::TG),
            "TK" => Ok(Country::TK),
            "TN" => Ok(Country::TN),
            "TO" => Ok(Country::TO),
            "TR" => Ok(Country::TR),
            "TT" => Ok(Country::TT),
            "TV" => Ok(Country::TV),
            "TZ" => Ok(Country::TZ),
            "UA" => Ok(Country::UA),
            "UG" => Ok(Country::UG),
            "US" => Ok(Country::US),
            "UY" => Ok(Country::UY),
            "VA" => Ok(Country::VA),
            "VC" => Ok(Country::VC),
            "VE" => Ok(Country::VE),
            "VG" => Ok(Country::VG),
            "VI" => Ok(Country::VI),
            "VN" => Ok(Country::VN),
            "VU" => Ok(Country::VU),
            "WF" => Ok(Country::WF),
            "WS" => Ok(Country::WS),
            "YE" => Ok(Country::YE),
            "ZA" => Ok(Country::ZA),
            "ZM" => Ok(Country::ZM),
            "ZW" => Ok(Country::ZW),

            other => Err(anyhow::anyhow!("Unknown country code: {other}")),
        }
    }
}
