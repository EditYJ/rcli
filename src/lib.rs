mod cli;
mod process;
mod utils;

pub use cli::{
    Base64Format, Base64SubCommand, Cli, OutputFormat, SubCommand, TextSignFormat, TextSubCommand,
};
pub use process::{
    base64_decode, base64_encode, generate_sign_key, handle_csv_command, handle_gen_pass_command,
    sign_text, verify_text,
};

pub use utils::get_reader;
