mod base64;
mod csv;
mod genpass;

use std::path::Path;

pub use base64::{Base64Format, Base64Subcommand, DecodeOpts, EncodeOpts};
pub use csv::{CsvOpts, OutputFormat};
pub use genpass::GenPassOpts;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "rcli", author, version, about, long_about = None)]
pub struct Opts {
    #[clap(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    Genpass(GenPassOpts),
    #[clap(subcommand, about = "Base64 encode or decode")]
    Base64(Base64Subcommand),
}

pub fn validate_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() || filename == "-" {
        // Ok(filename.to_string())
        Ok(filename.into())
    } else {
        Err("Input file does not exist")
    }
}
