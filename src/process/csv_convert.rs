use anyhow::Result;
use csv::Reader;
use serde_json::Value;
use std::fs::write;

use crate::OutputFormat;

// 处理 csv 文件
pub fn handle_csv_command(
    input_path: &str,
    output_path: String,
    format: OutputFormat,
) -> Result<()> {
    let mut input_file = Reader::from_path(input_path)?;
    let mut result = Vec::with_capacity(128);
    let headers = input_file.headers()?.clone();
    let mut toml_result = std::collections::HashMap::new();

    for row in input_file.records() {
        let row_data = row?;
        let json_value = headers.iter().zip(row_data.iter()).collect::<Value>();
        result.push(json_value)
    }

    toml_result.insert("root", &result);

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&result)?,
        OutputFormat::Yaml => serde_yaml::to_string(&result)?,
        OutputFormat::Toml => toml::ser::to_string_pretty(&toml_result)?,
    };
    write(output_path, content)?;
    Ok(())
}
