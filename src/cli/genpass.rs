use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPassOption {
    // 定义了一组用于密码生成的配置参数
    #[arg(long, default_value_t = 16)]
    pub length: u8, // 密码长度，默认值为16

    #[arg(long, default_value_t = false)]
    pub no_uppercase: bool,

    #[arg(long, default_value_t = false)]
    pub no_lowercase: bool,

    #[arg(long, default_value_t = false)]
    pub no_number: bool,

    #[arg(long, default_value_t = false)]
    pub no_symbol: bool,
}
