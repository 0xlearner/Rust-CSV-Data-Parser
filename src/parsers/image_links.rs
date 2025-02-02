use crate::error::ParserError;

pub fn parse_image_links(links: &str) -> Result<String, ParserError> {
    let json_str = links
        .replace("['", "[\"")
        .replace("']", "\"]")
        .replace("', '", "\", \"");

    serde_json::from_str::<Vec<String>>(&json_str)
        .map_err(|e| ParserError::ImageLinksParseError(e.to_string()))?;

    Ok(json_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_image_link() {
        let input = "['https://example.com/image.jpg']";
        let result = parse_image_links(input).unwrap();
        assert_eq!(result, "[\"https://example.com/image.jpg\"]");

        // Verify it's valid JSON
        let parsed: Vec<String> = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0], "https://example.com/image.jpg");
    }

    #[test]
    fn test_multiple_image_links() {
        let input = "['https://example.com/1.jpg', 'https://example.com/2.jpg']";
        let result = parse_image_links(input).unwrap();

        // Verify it's valid JSON
        let parsed: Vec<String> = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0], "https://example.com/1.jpg");
        assert_eq!(parsed[1], "https://example.com/2.jpg");
    }

    #[test]
    fn test_no_images() {
        let input = "['No Images']";
        let result = parse_image_links(input).unwrap();

        // Verify it's valid JSON
        let parsed: Vec<String> = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0], "No Images");
    }

    #[test]
    fn test_invalid_format() {
        assert!(parse_image_links("[invalid format]").is_err());
        assert!(parse_image_links("not a list").is_err());
        assert!(parse_image_links("").is_err());
    }
}
