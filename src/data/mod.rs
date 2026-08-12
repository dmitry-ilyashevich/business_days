// This file is generated automatically. Do not edit manually.

#![allow(unused_imports)]

use std::collections::HashMap;
use std::ops::Range;

use anyhow::Result;
use chrono::NaiveDate;

use crate::country::Country;
use crate::holiday::{from_ymd_res, Holiday, HolidayPerYearMap, Year};

#[cfg(feature = "bi")]
pub mod bi;

#[cfg(feature = "mp")]
pub mod mp;

#[cfg(feature = "zw")]
pub mod zw;

#[cfg(feature = "et")]
pub mod et;

#[cfg(feature = "id")]
pub mod id;

#[cfg(feature = "pm")]
pub mod pm;

#[cfg(feature = "cu")]
pub mod cu;

#[cfg(feature = "aw")]
pub mod aw;

#[cfg(feature = "td")]
pub mod td;

#[cfg(feature = "rw")]
pub mod rw;

#[cfg(feature = "lu")]
pub mod lu;

#[cfg(feature = "cv")]
pub mod cv;

#[cfg(feature = "ie")]
pub mod ie;

#[cfg(feature = "jm")]
pub mod jm;

#[cfg(feature = "jp")]
pub mod jp;

#[cfg(feature = "gn")]
pub mod gn;

#[cfg(feature = "mq")]
pub mod mq;

#[cfg(feature = "ke")]
pub mod ke;

#[cfg(feature = "mt")]
pub mod mt;

#[cfg(feature = "gb")]
pub mod gb;

#[cfg(feature = "gg")]
pub mod gg;

#[cfg(feature = "na")]
pub mod na;

#[cfg(feature = "mr")]
pub mod mr;

#[cfg(feature = "py")]
pub mod py;

#[cfg(feature = "al")]
pub mod al;

#[cfg(feature = "sg")]
pub mod sg;

#[cfg(feature = "pn")]
pub mod pn;

#[cfg(feature = "ai")]
pub mod ai;

#[cfg(feature = "bs")]
pub mod bs;

#[cfg(feature = "gp")]
pub mod gp;

#[cfg(feature = "bl")]
pub mod bl;

#[cfg(feature = "gi")]
pub mod gi;

#[cfg(feature = "sd")]
pub mod sd;

#[cfg(feature = "be")]
pub mod be;

#[cfg(feature = "ci")]
pub mod ci;

#[cfg(feature = "ga")]
pub mod ga;

#[cfg(feature = "gw")]
pub mod gw;

#[cfg(feature = "ls")]
pub mod ls;

#[cfg(feature = "si")]
pub mod si;

#[cfg(feature = "no")]
pub mod no;

#[cfg(feature = "cf")]
pub mod cf;

#[cfg(feature = "ms")]
pub mod ms;

#[cfg(feature = "sx")]
pub mod sx;

#[cfg(feature = "bf")]
pub mod bf;

#[cfg(feature = "ua")]
pub mod ua;

#[cfg(feature = "pl")]
pub mod pl;

#[cfg(feature = "vi")]
pub mod vi;

#[cfg(feature = "zm")]
pub mod zm;

#[cfg(feature = "sm")]
pub mod sm;

#[cfg(feature = "so")]
pub mod so;

#[cfg(feature = "md")]
pub mod md;

#[cfg(feature = "gh")]
pub mod gh;

#[cfg(feature = "lv")]
pub mod lv;

#[cfg(feature = "bd")]
pub mod bd;

#[cfg(feature = "ht")]
pub mod ht;

#[cfg(feature = "mk")]
pub mod mk;

#[cfg(feature = "dz")]
pub mod dz;

#[cfg(feature = "sy")]
pub mod sy;

#[cfg(feature = "gy")]
pub mod gy;

#[cfg(feature = "ch")]
pub mod ch;

#[cfg(feature = "je")]
pub mod je;

#[cfg(feature = "to")]
pub mod to;

#[cfg(feature = "us")]
pub mod us;

#[cfg(feature = "bq")]
pub mod bq;

#[cfg(feature = "va")]
pub mod va;

#[cfg(feature = "bj")]
pub mod bj;

#[cfg(feature = "mn")]
pub mod mn;

#[cfg(feature = "nf")]
pub mod nf;

#[cfg(feature = "nu")]
pub mod nu;

#[cfg(feature = "nz")]
pub mod nz;

#[cfg(feature = "cd")]
pub mod cd;

#[cfg(feature = "pa")]
pub mod pa;

#[cfg(feature = "ve")]
pub mod ve;

#[cfg(feature = "ca")]
pub mod ca;

#[cfg(feature = "za")]
pub mod za;

#[cfg(feature = "ng")]
pub mod ng;

#[cfg(feature = "sb")]
pub mod sb;

#[cfg(feature = "vu")]
pub mod vu;

#[cfg(feature = "tt")]
pub mod tt;

#[cfg(feature = "cn")]
pub mod cn;

#[cfg(feature = "ru")]
pub mod ru;

#[cfg(feature = "ax")]
pub mod ax;

#[cfg(feature = "ck")]
pub mod ck;

#[cfg(feature = "rs")]
pub mod rs;

#[cfg(feature = "bb")]
pub mod bb;

#[cfg(feature = "ma")]
pub mod ma;

#[cfg(feature = "co")]
pub mod co;

#[cfg(feature = "gl")]
pub mod gl;

#[cfg(feature = "fm")]
pub mod fm;

#[cfg(feature = "ar")]
pub mod ar;

#[cfg(feature = "lc")]
pub mod lc;

#[cfg(feature = "mh")]
pub mod mh;

#[cfg(feature = "gd")]
pub mod gd;

#[cfg(feature = "ec")]
pub mod ec;

#[cfg(feature = "de")]
pub mod de;

#[cfg(feature = "hu")]
pub mod hu;

#[cfg(feature = "cr")]
pub mod cr;

#[cfg(feature = "pg")]
pub mod pg;

#[cfg(feature = "sc")]
pub mod sc;

#[cfg(feature = "sn")]
pub mod sn;

#[cfg(feature = "ky")]
pub mod ky;

#[cfg(feature = "is")]
pub mod is;

#[cfg(feature = "sv")]
pub mod sv;

#[cfg(feature = "sz")]
pub mod sz;

#[cfg(feature = "dk")]
pub mod dk;

#[cfg(feature = "sr")]
pub mod sr;

#[cfg(feature = "tk")]
pub mod tk;

#[cfg(feature = "uy")]
pub mod uy;

#[cfg(feature = "lr")]
pub mod lr;

#[cfg(feature = "it")]
pub mod it;

#[cfg(feature = "pe")]
pub mod pe;

#[cfg(feature = "gq")]
pub mod gq;

#[cfg(feature = "im")]
pub mod im;

#[cfg(feature = "mc")]
pub mod mc;

#[cfg(feature = "ki")]
pub mod ki;

#[cfg(feature = "pw")]
pub mod pw;

#[cfg(feature = "hr")]
pub mod hr;

#[cfg(feature = "tc")]
pub mod tc;

#[cfg(feature = "vc")]
pub mod vc;

#[cfg(feature = "hk")]
pub mod hk;

#[cfg(feature = "bh")]
pub mod bh;

#[cfg(feature = "mg")]
pub mod mg;

#[cfg(feature = "hn")]
pub mod hn;

#[cfg(feature = "iq")]
pub mod iq;

#[cfg(feature = "mw")]
pub mod mw;

#[cfg(feature = "tz")]
pub mod tz;

#[cfg(feature = "cl")]
pub mod cl;

#[cfg(feature = "sk")]
pub mod sk;

#[cfg(feature = "ne")]
pub mod ne;

#[cfg(feature = "nl")]
pub mod nl;

#[cfg(feature = "ro")]
pub mod ro;

#[cfg(feature = "nr")]
pub mod nr;

#[cfg(feature = "sh")]
pub mod sh;

#[cfg(feature = "br")]
pub mod br;

#[cfg(feature = "eg")]
pub mod eg;

#[cfg(feature = "me")]
pub mod me;

#[cfg(feature = "au")]
pub mod au;

#[cfg(feature = "nc")]
pub mod nc;

#[cfg(feature = "cc")]
pub mod cc;

#[cfg(feature = "bz")]
pub mod bz;

#[cfg(feature = "ge")]
pub mod ge;

#[cfg(feature = "kh")]
pub mod kh;

#[cfg(feature = "dm")]
pub mod dm;

#[cfg(feature = "kz")]
pub mod kz;

#[cfg(feature = "ph")]
pub mod ph;

#[cfg(feature = "do")]
pub mod r#do;

#[cfg(feature = "ao")]
pub mod ao;

#[cfg(feature = "pr")]
pub mod pr;

#[cfg(feature = "ba")]
pub mod ba;

#[cfg(feature = "sj")]
pub mod sj;

#[cfg(feature = "lt")]
pub mod lt;

#[cfg(feature = "bw")]
pub mod bw;

#[cfg(feature = "ss")]
pub mod ss;

#[cfg(feature = "cw")]
pub mod cw;

#[cfg(feature = "cz")]
pub mod cz;

#[cfg(feature = "at")]
pub mod at;

#[cfg(feature = "cx")]
pub mod cx;

#[cfg(feature = "fr")]
pub mod fr;

#[cfg(feature = "km")]
pub mod km;

#[cfg(feature = "gr")]
pub mod gr;

#[cfg(feature = "li")]
pub mod li;

#[cfg(feature = "bm")]
pub mod bm;

#[cfg(feature = "er")]
pub mod er;

#[cfg(feature = "fk")]
pub mod fk;

#[cfg(feature = "ly")]
pub mod ly;

#[cfg(feature = "bo")]
pub mod bo;

#[cfg(feature = "mf")]
pub mod mf;

#[cfg(feature = "dj")]
pub mod dj;

#[cfg(feature = "mx")]
pub mod mx;

#[cfg(feature = "pt")]
pub mod pt;

#[cfg(feature = "st")]
pub mod st;

#[cfg(feature = "ad")]
pub mod ad;

#[cfg(feature = "mz")]
pub mod mz;

#[cfg(feature = "gf")]
pub mod gf;

#[cfg(feature = "cy")]
pub mod cy;

#[cfg(feature = "gt")]
pub mod gt;

#[cfg(feature = "kr")]
pub mod kr;

#[cfg(feature = "tn")]
pub mod tn;

#[cfg(feature = "tr")]
pub mod tr;

#[cfg(feature = "wf")]
pub mod wf;

#[cfg(feature = "cg")]
pub mod cg;

#[cfg(feature = "bg")]
pub mod bg;

#[cfg(feature = "cm")]
pub mod cm;

#[cfg(feature = "ag")]
pub mod ag;

#[cfg(feature = "am")]
pub mod am;

#[cfg(feature = "fi")]
pub mod fi;

#[cfg(feature = "sl")]
pub mod sl;

#[cfg(feature = "gm")]
pub mod gm;

#[cfg(feature = "tg")]
pub mod tg;

#[cfg(feature = "by")]
pub mod by;

#[cfg(feature = "ee")]
pub mod ee;

#[cfg(feature = "tv")]
pub mod tv;

#[cfg(feature = "vn")]
pub mod vn;

#[cfg(feature = "kn")]
pub mod kn;

#[cfg(feature = "pf")]
pub mod pf;

#[cfg(feature = "ug")]
pub mod ug;

#[cfg(feature = "ml")]
pub mod ml;

#[cfg(feature = "ye")]
pub mod ye;

#[cfg(feature = "se")]
pub mod se;

#[cfg(feature = "vg")]
pub mod vg;

#[cfg(feature = "ws")]
pub mod ws;

#[cfg(feature = "ni")]
pub mod ni;

#[cfg(feature = "fo")]
pub mod fo;

#[cfg(feature = "es")]
pub mod es;

pub fn build(country: Country, years: Option<&Range<Year>>) -> Result<HolidayPerYearMap> {
    match country {
        #[cfg(feature = "bi")]
        Country::BI => bi::build(years),

        #[cfg(feature = "mp")]
        Country::MP => mp::build(years),

        #[cfg(feature = "zw")]
        Country::ZW => zw::build(years),

        #[cfg(feature = "et")]
        Country::ET => et::build(years),

        #[cfg(feature = "id")]
        Country::ID => id::build(years),

        #[cfg(feature = "pm")]
        Country::PM => pm::build(years),

        #[cfg(feature = "cu")]
        Country::CU => cu::build(years),

        #[cfg(feature = "aw")]
        Country::AW => aw::build(years),

        #[cfg(feature = "td")]
        Country::TD => td::build(years),

        #[cfg(feature = "rw")]
        Country::RW => rw::build(years),

        #[cfg(feature = "lu")]
        Country::LU => lu::build(years),

        #[cfg(feature = "cv")]
        Country::CV => cv::build(years),

        #[cfg(feature = "ie")]
        Country::IE => ie::build(years),

        #[cfg(feature = "jm")]
        Country::JM => jm::build(years),

        #[cfg(feature = "jp")]
        Country::JP => jp::build(years),

        #[cfg(feature = "gn")]
        Country::GN => gn::build(years),

        #[cfg(feature = "mq")]
        Country::MQ => mq::build(years),

        #[cfg(feature = "ke")]
        Country::KE => ke::build(years),

        #[cfg(feature = "mt")]
        Country::MT => mt::build(years),

        #[cfg(feature = "gb")]
        Country::GB => gb::build(years),

        #[cfg(feature = "gg")]
        Country::GG => gg::build(years),

        #[cfg(feature = "na")]
        Country::NA => na::build(years),

        #[cfg(feature = "mr")]
        Country::MR => mr::build(years),

        #[cfg(feature = "py")]
        Country::PY => py::build(years),

        #[cfg(feature = "al")]
        Country::AL => al::build(years),

        #[cfg(feature = "sg")]
        Country::SG => sg::build(years),

        #[cfg(feature = "pn")]
        Country::PN => pn::build(years),

        #[cfg(feature = "ai")]
        Country::AI => ai::build(years),

        #[cfg(feature = "bs")]
        Country::BS => bs::build(years),

        #[cfg(feature = "gp")]
        Country::GP => gp::build(years),

        #[cfg(feature = "bl")]
        Country::BL => bl::build(years),

        #[cfg(feature = "gi")]
        Country::GI => gi::build(years),

        #[cfg(feature = "sd")]
        Country::SD => sd::build(years),

        #[cfg(feature = "be")]
        Country::BE => be::build(years),

        #[cfg(feature = "ci")]
        Country::CI => ci::build(years),

        #[cfg(feature = "ga")]
        Country::GA => ga::build(years),

        #[cfg(feature = "gw")]
        Country::GW => gw::build(years),

        #[cfg(feature = "ls")]
        Country::LS => ls::build(years),

        #[cfg(feature = "si")]
        Country::SI => si::build(years),

        #[cfg(feature = "no")]
        Country::NO => no::build(years),

        #[cfg(feature = "cf")]
        Country::CF => cf::build(years),

        #[cfg(feature = "ms")]
        Country::MS => ms::build(years),

        #[cfg(feature = "sx")]
        Country::SX => sx::build(years),

        #[cfg(feature = "bf")]
        Country::BF => bf::build(years),

        #[cfg(feature = "ua")]
        Country::UA => ua::build(years),

        #[cfg(feature = "pl")]
        Country::PL => pl::build(years),

        #[cfg(feature = "vi")]
        Country::VI => vi::build(years),

        #[cfg(feature = "zm")]
        Country::ZM => zm::build(years),

        #[cfg(feature = "sm")]
        Country::SM => sm::build(years),

        #[cfg(feature = "so")]
        Country::SO => so::build(years),

        #[cfg(feature = "md")]
        Country::MD => md::build(years),

        #[cfg(feature = "gh")]
        Country::GH => gh::build(years),

        #[cfg(feature = "lv")]
        Country::LV => lv::build(years),

        #[cfg(feature = "bd")]
        Country::BD => bd::build(years),

        #[cfg(feature = "ht")]
        Country::HT => ht::build(years),

        #[cfg(feature = "mk")]
        Country::MK => mk::build(years),

        #[cfg(feature = "dz")]
        Country::DZ => dz::build(years),

        #[cfg(feature = "sy")]
        Country::SY => sy::build(years),

        #[cfg(feature = "gy")]
        Country::GY => gy::build(years),

        #[cfg(feature = "ch")]
        Country::CH => ch::build(years),

        #[cfg(feature = "je")]
        Country::JE => je::build(years),

        #[cfg(feature = "to")]
        Country::TO => to::build(years),

        #[cfg(feature = "us")]
        Country::US => us::build(years),

        #[cfg(feature = "bq")]
        Country::BQ => bq::build(years),

        #[cfg(feature = "va")]
        Country::VA => va::build(years),

        #[cfg(feature = "bj")]
        Country::BJ => bj::build(years),

        #[cfg(feature = "mn")]
        Country::MN => mn::build(years),

        #[cfg(feature = "nf")]
        Country::NF => nf::build(years),

        #[cfg(feature = "nu")]
        Country::NU => nu::build(years),

        #[cfg(feature = "nz")]
        Country::NZ => nz::build(years),

        #[cfg(feature = "cd")]
        Country::CD => cd::build(years),

        #[cfg(feature = "pa")]
        Country::PA => pa::build(years),

        #[cfg(feature = "ve")]
        Country::VE => ve::build(years),

        #[cfg(feature = "ca")]
        Country::CA => ca::build(years),

        #[cfg(feature = "za")]
        Country::ZA => za::build(years),

        #[cfg(feature = "ng")]
        Country::NG => ng::build(years),

        #[cfg(feature = "sb")]
        Country::SB => sb::build(years),

        #[cfg(feature = "vu")]
        Country::VU => vu::build(years),

        #[cfg(feature = "tt")]
        Country::TT => tt::build(years),

        #[cfg(feature = "cn")]
        Country::CN => cn::build(years),

        #[cfg(feature = "ru")]
        Country::RU => ru::build(years),

        #[cfg(feature = "ax")]
        Country::AX => ax::build(years),

        #[cfg(feature = "ck")]
        Country::CK => ck::build(years),

        #[cfg(feature = "rs")]
        Country::RS => rs::build(years),

        #[cfg(feature = "bb")]
        Country::BB => bb::build(years),

        #[cfg(feature = "ma")]
        Country::MA => ma::build(years),

        #[cfg(feature = "co")]
        Country::CO => co::build(years),

        #[cfg(feature = "gl")]
        Country::GL => gl::build(years),

        #[cfg(feature = "fm")]
        Country::FM => fm::build(years),

        #[cfg(feature = "ar")]
        Country::AR => ar::build(years),

        #[cfg(feature = "lc")]
        Country::LC => lc::build(years),

        #[cfg(feature = "mh")]
        Country::MH => mh::build(years),

        #[cfg(feature = "gd")]
        Country::GD => gd::build(years),

        #[cfg(feature = "ec")]
        Country::EC => ec::build(years),

        #[cfg(feature = "de")]
        Country::DE => de::build(years),

        #[cfg(feature = "hu")]
        Country::HU => hu::build(years),

        #[cfg(feature = "cr")]
        Country::CR => cr::build(years),

        #[cfg(feature = "pg")]
        Country::PG => pg::build(years),

        #[cfg(feature = "sc")]
        Country::SC => sc::build(years),

        #[cfg(feature = "sn")]
        Country::SN => sn::build(years),

        #[cfg(feature = "ky")]
        Country::KY => ky::build(years),

        #[cfg(feature = "is")]
        Country::IS => is::build(years),

        #[cfg(feature = "sv")]
        Country::SV => sv::build(years),

        #[cfg(feature = "sz")]
        Country::SZ => sz::build(years),

        #[cfg(feature = "dk")]
        Country::DK => dk::build(years),

        #[cfg(feature = "sr")]
        Country::SR => sr::build(years),

        #[cfg(feature = "tk")]
        Country::TK => tk::build(years),

        #[cfg(feature = "uy")]
        Country::UY => uy::build(years),

        #[cfg(feature = "lr")]
        Country::LR => lr::build(years),

        #[cfg(feature = "it")]
        Country::IT => it::build(years),

        #[cfg(feature = "pe")]
        Country::PE => pe::build(years),

        #[cfg(feature = "gq")]
        Country::GQ => gq::build(years),

        #[cfg(feature = "im")]
        Country::IM => im::build(years),

        #[cfg(feature = "mc")]
        Country::MC => mc::build(years),

        #[cfg(feature = "ki")]
        Country::KI => ki::build(years),

        #[cfg(feature = "pw")]
        Country::PW => pw::build(years),

        #[cfg(feature = "hr")]
        Country::HR => hr::build(years),

        #[cfg(feature = "tc")]
        Country::TC => tc::build(years),

        #[cfg(feature = "vc")]
        Country::VC => vc::build(years),

        #[cfg(feature = "hk")]
        Country::HK => hk::build(years),

        #[cfg(feature = "bh")]
        Country::BH => bh::build(years),

        #[cfg(feature = "mg")]
        Country::MG => mg::build(years),

        #[cfg(feature = "hn")]
        Country::HN => hn::build(years),

        #[cfg(feature = "iq")]
        Country::IQ => iq::build(years),

        #[cfg(feature = "mw")]
        Country::MW => mw::build(years),

        #[cfg(feature = "tz")]
        Country::TZ => tz::build(years),

        #[cfg(feature = "cl")]
        Country::CL => cl::build(years),

        #[cfg(feature = "sk")]
        Country::SK => sk::build(years),

        #[cfg(feature = "ne")]
        Country::NE => ne::build(years),

        #[cfg(feature = "nl")]
        Country::NL => nl::build(years),

        #[cfg(feature = "ro")]
        Country::RO => ro::build(years),

        #[cfg(feature = "nr")]
        Country::NR => nr::build(years),

        #[cfg(feature = "sh")]
        Country::SH => sh::build(years),

        #[cfg(feature = "br")]
        Country::BR => br::build(years),

        #[cfg(feature = "eg")]
        Country::EG => eg::build(years),

        #[cfg(feature = "me")]
        Country::ME => me::build(years),

        #[cfg(feature = "au")]
        Country::AU => au::build(years),

        #[cfg(feature = "nc")]
        Country::NC => nc::build(years),

        #[cfg(feature = "cc")]
        Country::CC => cc::build(years),

        #[cfg(feature = "bz")]
        Country::BZ => bz::build(years),

        #[cfg(feature = "ge")]
        Country::GE => ge::build(years),

        #[cfg(feature = "kh")]
        Country::KH => kh::build(years),

        #[cfg(feature = "dm")]
        Country::DM => dm::build(years),

        #[cfg(feature = "kz")]
        Country::KZ => kz::build(years),

        #[cfg(feature = "ph")]
        Country::PH => ph::build(years),

        #[cfg(feature = "do")]
        Country::DO => r#do::build(years),

        #[cfg(feature = "ao")]
        Country::AO => ao::build(years),

        #[cfg(feature = "pr")]
        Country::PR => pr::build(years),

        #[cfg(feature = "ba")]
        Country::BA => ba::build(years),

        #[cfg(feature = "sj")]
        Country::SJ => sj::build(years),

        #[cfg(feature = "lt")]
        Country::LT => lt::build(years),

        #[cfg(feature = "bw")]
        Country::BW => bw::build(years),

        #[cfg(feature = "ss")]
        Country::SS => ss::build(years),

        #[cfg(feature = "cw")]
        Country::CW => cw::build(years),

        #[cfg(feature = "cz")]
        Country::CZ => cz::build(years),

        #[cfg(feature = "at")]
        Country::AT => at::build(years),

        #[cfg(feature = "cx")]
        Country::CX => cx::build(years),

        #[cfg(feature = "fr")]
        Country::FR => fr::build(years),

        #[cfg(feature = "km")]
        Country::KM => km::build(years),

        #[cfg(feature = "gr")]
        Country::GR => gr::build(years),

        #[cfg(feature = "li")]
        Country::LI => li::build(years),

        #[cfg(feature = "bm")]
        Country::BM => bm::build(years),

        #[cfg(feature = "er")]
        Country::ER => er::build(years),

        #[cfg(feature = "fk")]
        Country::FK => fk::build(years),

        #[cfg(feature = "ly")]
        Country::LY => ly::build(years),

        #[cfg(feature = "bo")]
        Country::BO => bo::build(years),

        #[cfg(feature = "mf")]
        Country::MF => mf::build(years),

        #[cfg(feature = "dj")]
        Country::DJ => dj::build(years),

        #[cfg(feature = "mx")]
        Country::MX => mx::build(years),

        #[cfg(feature = "pt")]
        Country::PT => pt::build(years),

        #[cfg(feature = "st")]
        Country::ST => st::build(years),

        #[cfg(feature = "ad")]
        Country::AD => ad::build(years),

        #[cfg(feature = "mz")]
        Country::MZ => mz::build(years),

        #[cfg(feature = "gf")]
        Country::GF => gf::build(years),

        #[cfg(feature = "cy")]
        Country::CY => cy::build(years),

        #[cfg(feature = "gt")]
        Country::GT => gt::build(years),

        #[cfg(feature = "kr")]
        Country::KR => kr::build(years),

        #[cfg(feature = "tn")]
        Country::TN => tn::build(years),

        #[cfg(feature = "tr")]
        Country::TR => tr::build(years),

        #[cfg(feature = "wf")]
        Country::WF => wf::build(years),

        #[cfg(feature = "cg")]
        Country::CG => cg::build(years),

        #[cfg(feature = "bg")]
        Country::BG => bg::build(years),

        #[cfg(feature = "cm")]
        Country::CM => cm::build(years),

        #[cfg(feature = "ag")]
        Country::AG => ag::build(years),

        #[cfg(feature = "am")]
        Country::AM => am::build(years),

        #[cfg(feature = "fi")]
        Country::FI => fi::build(years),

        #[cfg(feature = "sl")]
        Country::SL => sl::build(years),

        #[cfg(feature = "gm")]
        Country::GM => gm::build(years),

        #[cfg(feature = "tg")]
        Country::TG => tg::build(years),

        #[cfg(feature = "by")]
        Country::BY => by::build(years),

        #[cfg(feature = "ee")]
        Country::EE => ee::build(years),

        #[cfg(feature = "tv")]
        Country::TV => tv::build(years),

        #[cfg(feature = "vn")]
        Country::VN => vn::build(years),

        #[cfg(feature = "kn")]
        Country::KN => kn::build(years),

        #[cfg(feature = "pf")]
        Country::PF => pf::build(years),

        #[cfg(feature = "ug")]
        Country::UG => ug::build(years),

        #[cfg(feature = "ml")]
        Country::ML => ml::build(years),

        #[cfg(feature = "ye")]
        Country::YE => ye::build(years),

        #[cfg(feature = "se")]
        Country::SE => se::build(years),

        #[cfg(feature = "vg")]
        Country::VG => vg::build(years),

        #[cfg(feature = "ws")]
        Country::WS => ws::build(years),

        #[cfg(feature = "ni")]
        Country::NI => ni::build(years),

        #[cfg(feature = "fo")]
        Country::FO => fo::build(years),

        #[cfg(feature = "es")]
        Country::ES => es::build(years),

        #[allow(unreachable_patterns)]
        other => anyhow::bail!(
            "Country {other:?} is not supported. Please enable the `{}` cargo feature.",
            other.code().to_ascii_uppercase()
        ),
    }
}

pub fn build_year<const N: usize>(
    years: Option<&Range<Year>>,
    year: Year,
    holidays: [(NaiveDate, &'static str, &'static str); N],
    map: &mut HolidayPerYearMap,
    country: Country,
    country_name: &'static str,
) {
    if let Some(range) = years {
        if !range.contains(&year) {
            return;
        }
    }

    map.insert(
        year,
        holidays
            .into_iter()
            .map(|(date, name, name_en)| {
                (
                    date,
                    Holiday {
                        date,
                        country,
                        country_name,
                        name,
                        name_en,
                    },
                )
            })
            .collect(),
    );
}
