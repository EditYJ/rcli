use anyhow::Result;
use rand::prelude::SliceRandom;

const UPPER_CASE: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER_CASE: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"23456789";
const SYMBOL: &[u8] = b"!@#$%&*=";

// 生成密码
pub fn handle_gen_pass_command(
    length: u8,
    no_uppercase: bool,
    no_lowercase: bool,
    no_number: bool,
    no_symbol: bool,
) -> Result<String> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if !no_uppercase {
        chars.extend_from_slice(UPPER_CASE);
        password.push(*UPPER_CASE.choose(&mut rng).expect("uppercase is empty"))
    }
    if !no_lowercase {
        chars.extend_from_slice(LOWER_CASE);
        password.push(*LOWER_CASE.choose(&mut rng).expect("lowercase is empty"))
    }
    if !no_number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("number is empty"))
    }
    if !no_symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("symbol is empty"))
    }

    for _ in 0..(length - password.len() as u8) {
        let word = chars.choose(&mut rng).expect("chars is empty");
        password.push(*word);
    }

    password.shuffle(&mut rng);
    let password = String::from_utf8(password)?;

    Ok(password)
}
