use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs::write;

use crate::CsvOption;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn handle_csv_command(csv_option: CsvOption) -> Result<()> {
    let mut input_file = Reader::from_path(csv_option.input)?;
    let mut res = Vec::with_capacity(128);
    for row in input_file.deserialize() {
        let result: Player = row?;
        res.push(result)
    }
    let json = serde_json::to_string_pretty(&res)?;
    write(csv_option.output, json)?;
    Ok(())
}
