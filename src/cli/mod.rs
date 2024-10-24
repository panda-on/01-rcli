mod base64;
mod csv;
mod genpass;
mod http;
mod text;

use std::path::{Path, PathBuf};

pub use base64::{Base64Format, Base64Subcommand, DecodeOpts, EncodeOpts};
pub use csv::{CsvOpts, OutputFormat};
pub use genpass::GenPassOpts;
pub use http::HttpCommand;
pub use http::HttpOpts;
pub use text::*;

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
    #[clap(subcommand, about = "Sign a file")]
    Text(TextSubcommand),
    #[clap(subcommand, about = "Serve a directory as specified port")]
    Http(HttpCommand),
}

pub fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

pub fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist or is not a file")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_verify_path() {
        assert_eq!(
            verify_path("*"),
            Err("Path does not exist or is not a directory")
        );
    }

    #[test]
    fn test_verify_file() {
        assert_eq!(
            verify_file("*"),
            Err("File does not exist or is not a file")
        );
    }

    #[test]
    fn test_httpopts() {}
}
