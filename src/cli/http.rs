use crate::process_http_serve;

use super::{verify_dir, CmdExecutor};
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "http file server")]
    Serve(HttpServeOption),
}

impl CmdExecutor for HttpSubCommand {
    async fn execute(&self) -> anyhow::Result<()> {
        match self {
            Self::Serve(option) => option.execute().await,
        }
    }
}

#[derive(Debug, Parser)]
pub struct HttpServeOption {
    #[arg(short, long, value_parser=verify_dir, default_value=".")]
    pub dir: PathBuf,

    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExecutor for HttpServeOption {
    async fn execute(&self) -> anyhow::Result<()> {
        process_http_serve(self.dir.clone(), self.port).await
    }
}
