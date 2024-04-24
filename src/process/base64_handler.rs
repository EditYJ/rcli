use std::{
    fs::File,
    io::{stdin, Read},
};

use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine as _,
};

use crate::Base64Format;

fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

pub fn base64_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encode_str = match format {
        Base64Format::Standard => STANDARD.encode(buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buf),
    };

    println!("{}", encode_str);

    Ok(())
}

pub fn base64_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader: Box<dyn Read> = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let decode_str = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };

    let decode_res = String::from_utf8(decode_str)?;

    println!("{}", decode_res);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_base64_encode() {
        let input = "Cargo.toml";
        assert!(base64_encode(input, Base64Format::UrlSafe).is_ok());
        assert!(base64_encode(input, Base64Format::Standard).is_ok());
    }

    #[test]
    fn test_base64_decode() {
        let urlsafe_input = "fixtures/b64.txt";
        let standard_input = "fixtures/b64_standard.txt";
        assert!(base64_decode(urlsafe_input, Base64Format::UrlSafe).is_ok());
        assert!(base64_decode(standard_input, Base64Format::Standard).is_ok());
    }
}
