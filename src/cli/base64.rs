use core::fmt;
use std::str::FromStr;

use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::CmdExecutor;

use super::verify_file;

// base64 encode --input xxx --output xxx --format
// base64 decode --input xxx --output xxx --format

#[derive(Parser, Debug)]
#[enum_dispatch(CmdExecutor)]
pub enum Base64Subcommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(EncodeOpts),
    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(DecodeOpts),
}

#[derive(Parser, Debug)]
pub struct EncodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = parse_b64_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Parser, Debug)]
pub struct DecodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = parse_b64_format, default_value = "standard")]
    pub format: Base64Format,
}

impl CmdExecutor for EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = crate::base64_encode(&self.input, self.format)?;
        println!("{}", ret);
        Ok(())
    }
}

impl CmdExecutor for DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = crate::base64_decode(&self.input, self.format)?;
        println!("{}", ret);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_b64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("Invalid format!")),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(format: Base64Format) -> Self {
        match format {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
