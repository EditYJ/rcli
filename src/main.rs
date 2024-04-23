// rcli csv -i input.csv -o output.json --header -d ','
use anyhow::Result;
use clap::Parser;
use rcli::{handle_csv_command, handle_gen_pass_command, Cli, Command};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Command::Csv(csv_option) => {
            let output = if let Some(output) = csv_option.output {
                output.clone()
            } else {
                format!("output.{}", csv_option.format)
            };
            handle_csv_command(&csv_option.input, output, csv_option.format)?;
        }
        Command::GenPass(gen_pass_option) => handle_gen_pass_command(
            gen_pass_option.length,
            gen_pass_option.no_uppercase,
            gen_pass_option.no_lowercase,
            gen_pass_option.no_number,
            gen_pass_option.no_symbol,
        )?,
    }

    Ok(())
}
