use std::path::PathBuf;

use clap::Parser;

use crate::{process_http_serve, CmdExecutor};

use super::verify_path;

// rcli http serve -d /path/to/dir -p 8080

#[derive(Debug, Parser)]
pub enum HttpCommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(HttpOpts),
}

#[derive(Debug, Parser)]
pub struct HttpOpts {
    #[arg(short, long, default_value = ".", value_parser = verify_path)]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl CmdExecutor for HttpCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            Self::Serve(opts) => {
                process_http_serve(opts.dir, opts.port).await?;
                Ok(())
            }
        }
    }
}
