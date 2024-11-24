use serde::Deserialize;
use serde::de::Deserializer;
use chrono::NaiveDate;

#[derive(Debug, Deserialize)]
pub struct MarketData {
    #[serde(rename = "Date", with = "date_format")]
    pub date: NaiveDate,

    #[serde(rename = "Price", deserialize_with = "parse_optional_number")]
    pub price: Option<f64>,

    #[serde(rename = "Open", deserialize_with = "parse_optional_number")]
    pub open: Option<f64>,

    #[serde(rename = "High", deserialize_with = "parse_optional_number")]
    pub high: Option<f64>,

    #[serde(rename = "Low", deserialize_with = "parse_optional_number")]
    pub low: Option<f64>,

    #[serde(rename = "Vol.", deserialize_with = "parse_optional_volume")]
    pub volume: Option<f64>,

    #[serde(rename = "Change %", deserialize_with = "parse_optional_change_percent")]
    pub change_percent: Option<f64>,
}

mod date_format {
    use chrono::NaiveDate;
    use serde::Deserialize;
    use serde::de::{self, Deserializer};

    const FORMAT: &str = "%m/%d/%Y"; // Formato de data: MM/DD/YYYY

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(de::Error::custom)
    }
}

fn parse_optional_number<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    if let Some(s) = s {
        let s_clean = s.replace(",", ""); // Remove as vírgulas
        if s_clean.trim().is_empty() {
            Ok(None)
        } else {
            s_clean.parse::<f64>().map(Some).map_err(serde::de::Error::custom)
        }
    } else {
        Ok(None)
    }
}

fn parse_optional_volume<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    if let Some(s) = s {
        let s = s.replace(",", ""); // Remove as vírgulas

        let multiplier = if s.ends_with('K') {
            1_000.0
        } else if s.ends_with('M') {
            1_000_000.0
        } else if s.ends_with('B') {
            1_000_000_000.0
        } else {
            1.0
        };

        let s_clean = s.trim_end_matches(|c| c == 'K' || c == 'M' || c == 'B');
        if s_clean.trim().is_empty() {
            Ok(None)
        } else {
            let value = s_clean.parse::<f64>().map_err(serde::de::Error::custom)?;
            Ok(Some(value * multiplier))
        }
    } else {
        Ok(None)
    }
}

fn parse_optional_change_percent<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    if let Some(s) = s {
        let s_clean = s.trim_end_matches('%');
        if s_clean.trim().is_empty() {
            Ok(None)
        } else {
            s_clean.parse::<f64>().map(Some).map_err(serde::de::Error::custom)
        }
    } else {
        Ok(None)
    }
}
