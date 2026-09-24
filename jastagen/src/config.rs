//! Holds the entrant details that the summary sheet asks for but the log cannot
//! supply.

use serde::Deserialize;

/// The whole settings file.
#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    /// Name of the contest as it appears on both sheets.
    #[serde(default = "default_contest_name")]
    pub contest_name: String,

    pub entry: Entry,
    pub operator: Operator,
}

/// The entry declarations the rules require on the summary sheet.
#[derive(Debug, Clone, Deserialize)]
pub struct Entry {
    pub callsign: String,

    /// `J` for a station in Japan, `S` for one outside it.
    pub category: Category,

    /// Whether this is the entrant's first contest.
    #[serde(default)]
    pub first_entry: bool,

    pub gender: Gender,

    /// Preferred T-shirt size, used only if the activity award is drawn.
    #[serde(default)]
    pub tshirt_size: TshirtSize,
}

/// Personal details reproduced verbatim on the summary sheet.
#[derive(Debug, Clone, Deserialize)]
pub struct Operator {
    pub name: String,

    /// Postal code, written after the 〒 mark.
    #[serde(default)]
    pub zip: String,

    #[serde(default)]
    pub address: String,

    #[serde(default)]
    pub email: String,

    /// Operator licence class, such as `第三級アマチュア無線技士`.
    #[serde(default)]
    pub license_class: String,

    /// Transmitter power, such as `25W`.
    #[serde(default)]
    pub power: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Category {
    /// Domestic entry.
    #[serde(rename = "J", alias = "j")]
    Domestic,

    /// Overseas entry, which a Japanese operator working from abroad also takes.
    #[serde(rename = "S", alias = "s")]
    Overseas,
}

impl Category {
    /// Returns the single letter the summary sheet carries.
    pub fn as_str(self) -> &'static str {
        match self {
            Category::Domestic => "J",
            Category::Overseas => "S",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Gender {
    #[serde(rename = "OM", alias = "om")]
    Om,

    #[serde(rename = "YL", alias = "yl", alias = "XYL", alias = "xyl")]
    Yl,
}

impl Gender {
    /// Returns the wording the summary sheet carries.
    pub fn as_str(self) -> &'static str {
        match self {
            Gender::Om => "OM",
            Gender::Yl => "YL/XYL",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserialize)]
pub enum TshirtSize {
    S,
    M,
    #[default]
    L,
    LL,
}

impl TshirtSize {
    /// Returns the size as written on the summary sheet.
    pub fn as_str(self) -> &'static str {
        match self {
            TshirtSize::S => "S",
            TshirtSize::M => "M",
            TshirtSize::L => "L",
            TshirtSize::LL => "LL",
        }
    }
}

fn default_contest_name() -> String {
    "JASTA SSTV activity contest".to_string()
}
