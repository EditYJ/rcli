use super::{verify_dir, CmdExecutor};
use crate::process_http_serve;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum HttpSubCommand {
    #[command(about = "http file server")]
    Serve(HttpServeOption),
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
