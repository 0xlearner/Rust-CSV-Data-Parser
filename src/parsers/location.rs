use crate::error::ParserError;
use crate::models::Location;

pub fn parse_location(location: &str) -> Result<Location, ParserError> {
    let location = location.trim();

    if location.is_empty() {
        return Err(ParserError::LocationParseError(
            "Empty location string".to_string(),
        ));
    }

    // Handle special case for "NO OTHER LINE NEEDED"
    if location.to_uppercase().contains("NO OTHER LINE NEEDED") {
        return Ok(Location {
            city: location
                .split(',')
                .next()
                .ok_or_else(|| {
                    ParserError::LocationParseError(
                        "Invalid 'NO OTHER LINE NEEDED' format".to_string(),
                    )
                })?
                .trim()
                .to_string(),
            state: "UNKNOWN".to_string(),
        });
    }

    // Handle U.K. variations
    if location.to_lowercase().contains("u.k") {
        return Ok(Location {
            city: location
                .split(',')
                .next()
                .ok_or_else(|| {
                    ParserError::LocationParseError("Invalid UK location format".to_string())
                })?
                .trim()
                .to_string(),
            state: "UK".to_string(),
        });
    }

    let parts: Vec<&str> = location.split(',').collect();

    match parts.len() {
        0 => Err(ParserError::LocationParseError(
            "Location string split resulted in no parts".to_string(),
        )),
        1 => {
            if parts[0].trim().is_empty() {
                Err(ParserError::LocationParseError(
                    "Empty city name".to_string(),
                ))
            } else {
                Ok(Location {
                    city: parts[0].trim().to_string(),
                    state: "UNKNOWN".to_string(),
                })
            }
        }
        2 => {
            let city = parts[0].trim();
            if city.is_empty() {
                return Err(ParserError::LocationParseError(
                    "Empty city name".to_string(),
                ));
            }

            let state = parts[1].trim();
            let state = if state.eq_ignore_ascii_case("other") {
                "UNKNOWN".to_string()
            } else {
                if state.is_empty() {
                    return Err(ParserError::LocationParseError(
                        "Empty state name".to_string(),
                    ));
                }
                state.to_string()
            };

            Ok(Location {
                city: city.to_string(),
                state,
            })
        }
        _ => {
            let city = format!(
                "{}, {}",
                parts
                    .get(0)
                    .ok_or_else(|| ParserError::LocationParseError(
                        "Missing city part".to_string()
                    ))?
                    .trim(),
                parts
                    .get(1)
                    .ok_or_else(|| ParserError::LocationParseError(
                        "Missing second city part".to_string()
                    ))?
                    .trim()
            );

            let state = parts
                .last()
                .ok_or_else(|| ParserError::LocationParseError("Missing state part".to_string()))?
                .trim();

            if city.is_empty() {
                return Err(ParserError::LocationParseError(
                    "Empty city name".to_string(),
                ));
            }

            let state = if state.eq_ignore_ascii_case("other") {
                "UNKNOWN".to_string()
            } else {
                if state.is_empty() {
                    return Err(ParserError::LocationParseError(
                        "Empty state name".to_string(),
                    ));
                }
                state.to_string()
            };

            Ok(Location { city, state })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_location() {
        let result = parse_location("Wichita Falls, TX").unwrap();
        assert_eq!(result.city, "Wichita Falls");
        assert_eq!(result.state, "TX");
    }

    #[test]
    fn test_location_with_other() {
        let result = parse_location("Macclesfield, Other").unwrap();
        assert_eq!(result.city, "Macclesfield");
        assert_eq!(result.state, "UNKNOWN");
    }

    #[test]
    fn test_location_with_multiple_commas() {
        let result = parse_location("Linsldae, Leighton Buzzard, LA").unwrap();
        assert_eq!(result.city, "Linsldae, Leighton Buzzard");
        assert_eq!(result.state, "LA");
    }

    #[test]
    fn test_location_with_uk() {
        let result = parse_location("London u.K, other").unwrap();
        assert_eq!(result.city, "London u.K");
        assert_eq!(result.state, "UK");
    }

    #[test]
    fn test_location_with_no_other_line() {
        let result = parse_location("House, NO OTHER LINE NEEDED").unwrap();
        assert_eq!(result.city, "House");
        assert_eq!(result.state, "UNKNOWN");
    }

    #[test]
    fn test_empty_location() {
        assert!(matches!(
            parse_location(""),
            Err(ParserError::LocationParseError(_))
        ));
    }

    #[test]
    fn test_location_with_empty_city() {
        assert!(matches!(
            parse_location(", TX"),
            Err(ParserError::LocationParseError(_))
        ));
    }

    #[test]
    fn test_location_with_empty_state() {
        assert!(matches!(
            parse_location("Dallas, "),
            Err(ParserError::LocationParseError(_))
        ));
    }

    #[test]
    fn test_location_with_only_city() {
        let result = parse_location("Portland").unwrap();
        assert_eq!(result.city, "Portland");
        assert_eq!(result.state, "UNKNOWN");
    }

    #[test]
    fn test_location_with_spaces() {
        let result = parse_location("  Portland  ").unwrap();
        assert_eq!(result.city, "Portland");
        assert_eq!(result.state, "UNKNOWN");
    }
}
