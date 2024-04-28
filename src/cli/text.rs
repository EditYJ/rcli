use std::{fmt::Display, path::PathBuf, str::FromStr};

use crate::{generate_sign_key, sign_text, verify_text};

use super::{verify_dir, verify_file, CmdExecutor};
use clap::Parser;
use tokio::fs;

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign text with a private/shared key")]
    Sign(SignTextOption),
    #[command(about = "Verify a sign message")]
    Verify(VerifyTextOption),
    #[command(about = "Generate a new key")]
    Generate(GenerateTextOption),
}

impl CmdExecutor for TextSubCommand {
    async fn execute(&self) -> anyhow::Result<()> {
        match self {
            TextSubCommand::Sign(opts) => opts.execute().await,
            TextSubCommand::Verify(opts) => opts.execute().await,
            TextSubCommand::Generate(opts) => opts.execute().await,
        }
    }
}

#[derive(Debug, Parser)]
pub struct GenerateTextOption {
    #[arg(long, value_parser=sign_format_parser, default_value = "blake3")]
    pub format: TextSignFormat,

    #[arg(short, long, value_parser=verify_dir)]
    pub output: PathBuf,
}

impl CmdExecutor for GenerateTextOption {
    async fn execute(&self) -> anyhow::Result<()> {
        let key = generate_sign_key(self.format)?;
        match self.format {
            TextSignFormat::Ed25519 => {
                let path_name = &self.output;
                fs::write(path_name.join("ed25519.sk"), &key[0]).await?;
                fs::write(path_name.join("ed25519.pk"), &key[1]).await?;
            }
            TextSignFormat::Blake3 => {
                let path_name = self.output.join("blake3.txt");
                fs::write(path_name, &key[0]).await?;
            }
        }
        Ok(())
    }
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

impl CmdExecutor for SignTextOption {
    async fn execute(&self) -> anyhow::Result<()> {
        let signed = sign_text(&self.input, &self.key, self.format)?;
        println!("{}", signed);
        Ok(())
    }
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

impl CmdExecutor for VerifyTextOption {
    async fn execute(&self) -> anyhow::Result<()> {
        let verified = verify_text(&self.input, &self.key, &self.sig, self.format)?;
        println!("验证情况: {}", verified);
        Ok(())
    }
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
