use anyhow::Result;
use csv::Reader;
use serde_json::Value;
use std::fs::write;

use crate::CsvOption;

pub fn handle_csv_command(csv_option: CsvOption) -> Result<()> {
    let mut input_file = Reader::from_path(csv_option.input)?;
    let mut result = Vec::with_capacity(128);
    let headers = input_file.headers()?.clone();
    for row in input_file.records() {
        let row_data = row?;
        let json_value = headers.iter().zip(row_data.iter()).collect::<Value>();
        result.push(json_value)
    }
    let json = serde_json::to_string_pretty(&result)?;
    write(csv_option.output, json)?;
    Ok(())
}
