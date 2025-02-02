use crate::error::ParserError;

pub fn parse_rating(rating: &str) -> Result<f32, ParserError> {
    match rating.trim() {
        "N/A" => Ok(0.0),
        rating_str => rating_str
            .parse::<f32>()
            .map_err(|e| ParserError::RatingParseError(e.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_integer_rating() {
        let result = parse_rating("5").unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_valid_float_rating() {
        let result = parse_rating("4.5").unwrap();
        assert_eq!(result, 4.5);
    }

    #[test]
    fn test_na_rating() {
        let result = parse_rating("N/A").unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_na_rating_with_spaces() {
        let result = parse_rating(" N/A ").unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_zero_rating() {
        let result = parse_rating("0").unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_invalid_rating() {
        assert!(parse_rating("invalid").is_err());
        assert!(parse_rating("").is_err());
        assert!(parse_rating("5.5.5").is_err());
        assert!(parse_rating("five").is_err());
    }
}
