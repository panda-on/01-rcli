// 1. 给文本签名；2. 验证文本签名；3. 生成密钥；

use crate::{get_content, get_reader, process_text_generate_key, process_text_verify, CmdExecutor};

use super::{verify_file, verify_path};
use clap::Parser;
use enum_dispatch::enum_dispatch;
use std::{fmt, fs, path::PathBuf, str::FromStr};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum TextSubcommand {
    #[command(about = "Sign a text file with a private/session key and return a signature")]
    Sign(TextSignOpts),
    #[command(about = "Verify a signature with a private/session key")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a random blake3 or ed25519 key")]
    Keygen(GenerateKeyOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, default_value = "-", value_parser = verify_file)]
    pub input: String,
    #[arg(short,long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, default_value = "-", value_parser = verify_file)]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long, value_parser = verify_file)]
    pub signature: String,
    #[arg(long, default_value = "blake3", value_parser = parse_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct GenerateKeyOpts {
    #[arg(long, default_value = "blake3", value_parser = parse_sign_format)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output_path: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_sign_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}

impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}

impl CmdExecutor for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = crate::get_reader(&self.input)?;
        let key = crate::get_content(&self.key)?;
        let ret = crate::process_text_sign(&mut reader, self.format, &key)?;
        println!("{:?}", ret);
        Ok(())
    }
}

impl CmdExecutor for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let mut reader = get_reader(&self.input)?;
        let key = get_content(&self.input)?;
        let signature = get_content(&self.signature)?;
        let ret = process_text_verify(&mut reader, &key, &signature, self.format)?;
        println!("{:?}", ret);
        Ok(())
    }
}

impl CmdExecutor for GenerateKeyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let map = process_text_generate_key(self.format)?;
        for (k, v) in map {
            fs::write(self.output_path.join(k), v)?;
        }
        Ok(())
    }
}
