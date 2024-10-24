use clap::Parser;

use crate::CmdExecutor;
use zxcvbn::zxcvbn;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub upper: bool,

    #[arg(long, default_value_t = true)]
    pub lower: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

impl CmdExecutor for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let ret = crate::generate_password(
            self.length,
            self.upper,
            self.lower,
            self.number,
            self.symbol,
        )?;
        println!("{}", ret);
        let estimate = zxcvbn(&ret, &[]);
        eprintln!("Estimated password strength: {:?}", estimate.score());
        Ok(())
    }
}
