use pyo3::prelude::*;
use pyo3::types::PyDict;
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

#[pyclass]
struct DataParser {
    records: Vec<OutputRecord>,
}

#[pymethods]
impl DataParser {
    #[new]
    fn new() -> Self {
        DataParser {
            records: Vec::new(),
        }
    }

    fn parse_csv(&mut self, filepath: &str) -> PyResult<()> {
        let file = File::open(filepath).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to open file: {}", e))
        })?;

        let mut rdr = csv::Reader::from_reader(file);
        let input_records: Vec<InputRecord> = rdr
            .deserialize()
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Failed to parse CSV: {}",
                    e
                ))
            })?;

        self.records = input_records
            .par_iter()
            .filter_map(|record| match process_record(record) {
                Ok(record) => Some(record),
                Err(e) => {
                    eprintln!("Error processing record: {}", e);
                    None
                }
            })
            .collect();

        Ok(())
    }

    fn sort_by_date(&mut self) -> PyResult<()> {
        self.records.sort_by_key(|record| record.date);
        Ok(())
    }

    fn get_records(&self) -> PyResult<Vec<PyObject>> {
        Python::with_gil(|py| {
            self.records
                .iter()
                .map(|record| {
                    let dict = PyDict::new(py);
                    dict.set_item("name", record.name.clone())?;
                    dict.set_item("city", record.city.clone())?;
                    dict.set_item("state", record.state.clone())?;
                    dict.set_item("date", record.date.format("%Y-%m-%d").to_string())?;
                    dict.set_item("rating", record.rating)?;
                    dict.set_item("review", record.review.clone())?;
                    dict.set_item("image_links", record.image_links.clone())?;
                    Ok(dict.to_object(py))
                })
                .collect()
        })
    }

    fn save_to_csv(&self, filepath: &str) -> PyResult<()> {
        let output_file = File::create(filepath).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to create file: {}", e))
        })?;

        let mut writer = csv::Writer::from_writer(output_file);

        for record in &self.records {
            writer.serialize(record).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Failed to write record: {}",
                    e
                ))
            })?;
        }

        Ok(())
    }
}

#[pymodule]
fn sb_data_cleaner(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<DataParser>()?;
    Ok(())
}
