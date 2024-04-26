mod base64;
mod csv;
mod genpass;
mod text;

use std::path::{Path, PathBuf};

use self::{csv::CsvOption, genpass::GenPassOption};
use clap::Parser;

pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::OutputFormat,
    text::{TextSignFormat, TextSubCommand},
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
    #[command(subcommand)]
    Text(TextSubCommand),
}

pub fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("指定文件不存存在！")
    }
}

pub fn verify_dir(filename: &str) -> Result<PathBuf, &'static str> {
    let path = Path::new(filename);
    if path.exists() && path.is_dir() {
        Ok(filename.into())
    } else {
        Err("指定目录不存在！")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input_file_parser() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("指定文件不存存在！"));
        assert_eq!(verify_file("not-exit"), Err("指定文件不存存在！"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
    }
}
