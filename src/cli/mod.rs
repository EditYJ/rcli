mod base64;
mod csv;
mod genpass;

use std::path::Path;

use self::{csv::CsvOption, genpass::GenPassOption};
use clap::Parser;

pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::OutputFormat,
};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "转换Csv文件")]
    Csv(CsvOption),
    #[command(name = "genpass", about = "生成随机密码")]
    GenPass(GenPassOption),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

pub fn input_file_parser(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("指定文件不存存在！")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input_file_parser() {
        assert_eq!(input_file_parser("-"), Ok("-".into()));
        assert_eq!(input_file_parser("*"), Err("指定文件不存存在！"));
        assert_eq!(input_file_parser("not-exit"), Err("指定文件不存存在！"));
        assert_eq!(input_file_parser("Cargo.toml"), Ok("Cargo.toml".into()));
    }
}
