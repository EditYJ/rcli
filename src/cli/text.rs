use std::{fmt::Display, path::PathBuf, str::FromStr};

use super::{verify_dir, verify_file};
use clap::Parser;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign text with a private/shared key")]
    Sign(SignTextOption),
    #[command(about = "Verify a sign message")]
    Verify(VerifyTextOption),
    #[command(about = "Generate a new key")]
    Generate(GenerateTextOption),
}

#[derive(Debug, Parser)]
pub struct GenerateTextOption {
    #[arg(long, value_parser=sign_format_parser, default_value = "blake3")]
    pub format: TextSignFormat,

    #[arg(short, long, value_parser=verify_dir)]
    pub output: PathBuf,
}

#[derive(Debug, Parser)]
pub struct SignTextOption {
    #[arg(short, long, value_parser=verify_file, default_value="-")]
    pub input: String,

    #[arg(short, long, value_parser=verify_file)]
    pub key: String,

    #[arg(long, value_parser=sign_format_parser, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct VerifyTextOption {
    #[arg(short, long, value_parser=verify_file, default_value="-")]
    pub input: String,

    #[arg(short, long, value_parser=verify_file)]
    pub key: String,

    #[arg(short, long)]
    pub sig: String,

    #[arg(long, value_parser=sign_format_parser, default_value = "blake3")]
    pub format: TextSignFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

impl FromStr for TextSignFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(format!("Invalid sign format: {}", s)),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}

fn sign_format_parser(s: &str) -> Result<TextSignFormat, String> {
    s.parse()
}
