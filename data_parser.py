from sb_data_cleaner import DataParser # type: ignore


def main():
    # Create parser instance
    parser = DataParser()

    # Parse CSV file
    parser.parse_csv("reviews_data.csv")

    # Sort by date
    parser.sort_by_date()

    # Get records as list of dictionaries
    records = parser.get_records()

    # Process records in Python
    for record in records:
        print(f"Review for {record['name']} in {record['city']}, {record['state']}")
        print(f"Date: {record['date']}")
        print(f"Rating: {record['rating']}")
        print("---")

    # Save processed data
    parser.save_to_csv("cleaned_reviews.csv")


if __name__ == "__main__":
    main()
