use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Failed to parse date: {0}")]
    DateParseError(String),

    #[error("Failed to parse location: {0}")]
    LocationParseError(String),

    #[error("Failed to parse rating: {0}")]
    RatingParseError(String),

    #[error("Failed to parse image links: {0}")]
    ImageLinksParseError(String),

    #[error("CSV error: {0}")]
    CsvError(#[from] csv::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}
