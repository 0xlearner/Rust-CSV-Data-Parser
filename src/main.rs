use rayon::prelude::*;
use std::fs::File;

mod error;
mod models;
mod parsers;

use crate::error::ParserError;
use crate::models::{InputRecord, OutputRecord};
use crate::parsers::{date, image_links, location, rating};

fn process_record(record: &InputRecord) -> Result<OutputRecord, ParserError> {
    let location = location::parse_location(&record.location)?;
    let date = date::parse_date(&record.date)?;
    let rating = rating::parse_rating(&record.rating)?;
    let image_links = image_links::parse_image_links(&record.image_links)?;

    Ok(OutputRecord {
        name: record.name.clone(),
        city: location.city,
        state: location.state,
        date,
        rating,
        review: record.review.clone(),
        image_links,
    })
}

fn main() -> Result<(), ParserError> {
    let file = File::open("reviews_data.csv")?;
    let mut rdr = csv::Reader::from_reader(file);

    let records: Vec<InputRecord> = rdr.deserialize().collect::<Result<Vec<_>, _>>()?;

    let mut processed_records: Vec<OutputRecord> = records
        .par_iter()
        .filter_map(|record| match process_record(record) {
            Ok(record) => Some(record),
            Err(e) => {
                eprintln!("Error processing record: {}", e);
                None
            }
        })
        .collect();

    processed_records.sort_by_key(|record| record.date);

    let output_file = File::create("cleaned_starbucks_reviews.csv")?;
    let mut writer = csv::Writer::from_writer(output_file);

    for record in processed_records {
        writer.serialize(record)?;
    }

    Ok(())
}
