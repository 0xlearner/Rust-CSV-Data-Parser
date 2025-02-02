use crate::error::ParserError;
use chrono::NaiveDate;

pub fn parse_date(date_str: &str) -> Result<NaiveDate, ParserError> {
    let date_str = date_str.replace("Reviewed ", "");

    let date_str = date_str
        .replace("Sept.", "September")
        .replace("Oct.", "October")
        .replace("Nov.", "November")
        .replace("Dec.", "December")
        .replace("Jan.", "January")
        .replace("Feb.", "February")
        .replace("Mar.", "March")
        .replace("Apr.", "April")
        .replace("Aug.", "August");

    NaiveDate::parse_from_str(&date_str, "%B %d, %Y")
        .map_err(|e| ParserError::DateParseError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_full_date_format() {
        let result = parse_date("Reviewed September 13, 2023").unwrap();
        assert_eq!(result, NaiveDate::from_ymd_opt(2023, 9, 13).unwrap());
    }

    #[test]
    fn test_abbreviated_month() {
        let result = parse_date("Reviewed Sept. 13, 2023").unwrap();
        assert_eq!(result, NaiveDate::from_ymd_opt(2023, 9, 13).unwrap());
    }

    #[test]
    fn test_different_months() {
        let test_cases = vec![
            (
                "Reviewed Jan. 1, 2023",
                NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            ),
            (
                "Reviewed Feb. 15, 2023",
                NaiveDate::from_ymd_opt(2023, 2, 15).unwrap(),
            ),
            (
                "Reviewed Mar. 30, 2023",
                NaiveDate::from_ymd_opt(2023, 3, 30).unwrap(),
            ),
            (
                "Reviewed Apr. 10, 2023",
                NaiveDate::from_ymd_opt(2023, 4, 10).unwrap(),
            ),
            (
                "Reviewed Aug. 20, 2023",
                NaiveDate::from_ymd_opt(2023, 8, 20).unwrap(),
            ),
        ];

        for (input, expected) in test_cases {
            let result = parse_date(input).unwrap();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_invalid_date() {
        assert!(parse_date("Invalid date").is_err());
        assert!(parse_date("Reviewed Invalid 13, 2023").is_err());
        assert!(parse_date("Reviewed Sept. 32, 2023").is_err());
    }
}
