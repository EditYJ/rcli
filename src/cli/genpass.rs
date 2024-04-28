use super::CmdExecutor;
use crate::handle_gen_pass_command;
use anyhow::Ok;
use clap::Parser;
use zxcvbn::zxcvbn;

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

impl CmdExecutor for GenPassOption {
    async fn execute(&self) -> anyhow::Result<()> {
        let password = handle_gen_pass_command(
            self.length,
            self.no_uppercase,
            self.no_lowercase,
            self.no_number,
            self.no_symbol,
        )?;
        println!("{}", password); // 输出生成的密码
        let estimate = zxcvbn(&password, &[])?;
        eprintln!("生成密码强度: {}", estimate.score());
        Ok(())
    }
}
