use std::{fmt::Display, str::FromStr};

use clap::Parser;

use crate::{base64_decode, base64_encode};

use super::{verify_file, CmdExecutor};

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(about = "Base64 encode")]
    Encode(Base64EncodeOption),
    #[command(about = "Base64 decode")]
    Decode(Base64DecodeOption),
}

impl CmdExecutor for Base64SubCommand {
    async fn execute(&self) -> anyhow::Result<()> {
        match self {
            Base64SubCommand::Encode(option) => option.execute().await,
            Base64SubCommand::Decode(option) => option.execute().await,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl FromStr for Base64Format {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(format!("Invalid base64 format: {}", s)),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOption {
    // 定义了用于 base64 编码的配置参数
    // - 表示 输入文件，默认为标准输入(stdin)
    #[arg(long, value_parser=verify_file, default_value="-")]
    pub input: String,

    #[arg(long, value_parser=base64_format_parser, default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExecutor for Base64EncodeOption {
    async fn execute(&self) -> anyhow::Result<()> {
        let encode_str = base64_encode(&self.input, self.format)?;
        println!("{}", encode_str);
        Ok(())
    }
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOption {
    //  定义了用于 base64 解码的配置参数
    // - 表示 输入文件，默认为标准输入(stdin)
    #[arg(long, value_parser=verify_file, default_value="-")]
    pub input: String,

    #[arg(long,value_parser=base64_format_parser,  default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExecutor for Base64DecodeOption {
    async fn execute(&self) -> anyhow::Result<()> {
        let decode_res = base64_decode(&self.input, self.format)?;
        println!("{}", decode_res);
        Ok(())
    }
}

fn base64_format_parser(format_str: &str) -> Result<Base64Format, String> {
    format_str.parse()
}
