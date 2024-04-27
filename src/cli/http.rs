use super::verify_dir;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
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
