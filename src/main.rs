// rcli csv -i input.csv -o output.json --header -d ','
use anyhow::Result;
use clap::Parser;
use rcli::{handle_csv_command, Cli, Command};

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
    }

    Ok(())
}
