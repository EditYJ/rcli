use std::fs;

use anyhow::Result;
use clap::Parser;
use rcli::{
    base64_decode, base64_encode, generate_sign_key, handle_csv_command, handle_gen_pass_command,
    process_http_serve, sign_text, verify_text, Base64SubCommand, Cli, SubCommand, TextSignFormat,
    TextSubCommand,
};
use zxcvbn::zxcvbn;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
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
        SubCommand::GenPass(gen_pass_option) => {
            let password = handle_gen_pass_command(
                gen_pass_option.length,
                gen_pass_option.no_uppercase,
                gen_pass_option.no_lowercase,
                gen_pass_option.no_number,
                gen_pass_option.no_symbol,
            )?;
            println!("{}", password); // 输出生成的密码
            let estimate = zxcvbn(&password, &[])?;
            eprintln!("生成密码强度: {}", estimate.score());
        }
        SubCommand::Base64(sub) => match sub {
            Base64SubCommand::Encode(encode_option) => {
                let encode_str = base64_encode(&encode_option.input, encode_option.format)?;
                println!("{}", encode_str);
            }
            Base64SubCommand::Decode(decode_option) => {
                let decode_res = base64_decode(&decode_option.input, decode_option.format)?;
                println!("{}", decode_res);
            }
        },
        SubCommand::Text(sub) => match sub {
            TextSubCommand::Sign(opts) => {
                let signed = sign_text(&opts.input, &opts.key, opts.format)?;
                println!("{}", signed);
            }
            TextSubCommand::Verify(opts) => {
                let verified = verify_text(&opts.input, &opts.key, &opts.sig, opts.format)?;
                println!("验证情况: {}", verified);
            }
            TextSubCommand::Generate(opts) => {
                let key = generate_sign_key(opts.format)?;
                match opts.format {
                    TextSignFormat::Ed25519 => {
                        let path_name = opts.output;
                        fs::write(path_name.join("ed25519.sk"), &key[0])?;
                        fs::write(path_name.join("ed25519.pk"), &key[1])?;
                    }
                    TextSignFormat::Blake3 => {
                        let path_name = opts.output.join("blake3.txt");
                        fs::write(path_name, &key[0])?;
                    }
                }
            }
        },
        SubCommand::Http(sub) => match sub {
            rcli::HttpSubCommand::Serve(opts) => {
                process_http_serve(opts.dir, opts.port).await?;
            }
        },
    }

    Ok(())
}
