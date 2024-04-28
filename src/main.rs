use anyhow::Result;
use clap::Parser;
use rcli::{Cli, CmdExecutor};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    cli.execute().await?;
    Ok(())
}
