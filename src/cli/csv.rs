use crate::validate_input_file;
use clap::Parser;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = validate_input_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short, long, default_value = ",")]
    pub delimiter: String,

    #[arg(short, long, value_parser = parse_format, default_value = "csv")]
    pub format: OutputFormat,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.to_lowercase().parse()
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}
