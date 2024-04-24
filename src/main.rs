use anyhow::Result;
use clap::Parser;
use rcli::{
    base64_decode, base64_encode, handle_csv_command, handle_gen_pass_command, Base64SubCommand,
    Cli, SubCommand,
};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        SubCommand::Csv(csv_option) => {
            let output = if let Some(output) = csv_option.output {
                output.clone()
            } else {
                format!("output.{}", csv_option.format)
            };
            handle_csv_command(&csv_option.input, output, csv_option.format)?;
        }
        SubCommand::GenPass(gen_pass_option) => handle_gen_pass_command(
            gen_pass_option.length,
            gen_pass_option.no_uppercase,
            gen_pass_option.no_lowercase,
            gen_pass_option.no_number,
            gen_pass_option.no_symbol,
        )?,
        SubCommand::Base64(sub) => match sub {
            Base64SubCommand::Encode(encode_option) => {
                base64_encode(&encode_option.input, encode_option.format)?;
            }
            Base64SubCommand::Decode(decode_option) => {
                base64_decode(&decode_option.input, decode_option.format)?;
            }
        },
    }

    Ok(())
}
