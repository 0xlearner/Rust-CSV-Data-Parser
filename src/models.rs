use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct InputRecord {
    pub name: String,
    pub location: String,
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Rating")]
    pub rating: String,
    #[serde(rename = "Review")]
    pub review: String,
    #[serde(rename = "Image_Links")]
    pub image_links: String,
}

#[derive(Debug, Serialize)]
pub struct OutputRecord {
    pub name: String,
    pub city: String,
    pub state: String,
    #[serde(serialize_with = "serialize_date")]
    pub date: NaiveDate,
    pub rating: f32,
    pub review: String,
    pub image_links: String,
}

fn serialize_date<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&date.format("%Y-%m-%d").to_string())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub city: String,
    pub state: String,
}
