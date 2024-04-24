mod cli;
mod process;

pub use cli::{Base64Format, Base64SubCommand, Cli, OutputFormat, SubCommand};
pub use process::{base64_decode, base64_encode, handle_csv_command, handle_gen_pass_command};
