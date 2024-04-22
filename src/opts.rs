use clap::{command, Parser};
use std::path::Path;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    #[command(name = "csv", about = "转换Csv文件")]
    Csv(CsvOption),
}

#[derive(Debug, Parser)]
pub struct CsvOption {
    #[arg(short, long, value_parser=csv_input_opt_parser)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn csv_input_opt_parser(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("制定文件不存存在！")
    }
}
