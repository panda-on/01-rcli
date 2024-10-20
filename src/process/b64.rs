use crate::cli::Base64Format;
use std::{fs::File, io::Read};

use anyhow::Result;
use base64::{
    engine::general_purpose::STANDARD, engine::general_purpose::URL_SAFE_NO_PAD, Engine as _,
};

pub fn base64_encode(input: &str, format: Base64Format) -> Result<()> {
    // let mut read: Box<dyn Read> = if input == "-" {
    //     Box::new(std::io::stdin())
    // } else {
    //     Box::new(File::open(input)?)
    // };

    // let mut buf = Vec::new();
    // read.read_to_end(&mut buf)?;
    // let encoded = match format {
    //     Base64Format::Standard => STANDARD.encode(buf),
    //     Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buf),
    // };

    // println!("{}", encoded);

    // Ok(())

    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(&buf),
    };
    println!("{}", encoded);
    Ok(())
}

pub fn base64_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut read: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };

    let mut buf = Vec::new();
    read.read_to_end(&mut buf)?;

    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };
    println!("{}", String::from_utf8(decoded)?);

    Ok(())
}

fn get_reader(input: &str) -> Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(input)?)
    };
    Ok(reader)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(base64_encode(input, format).is_ok());
    }

    #[test]
    fn test_base64_decode() {
        let input = "fixtures/b64.txt";
        let format = Base64Format::Standard;
        assert!(base64_decode(input, format).is_ok());
    }
}
