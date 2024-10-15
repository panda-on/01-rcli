use clap::Parser;
use rcli::{generate_password, process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output, opts.format)?,
        SubCommand::Genpass(opts) => generate_password(
            opts.length,
            opts.upper,
            opts.lower,
            opts.number,
            opts.symbol,
        )?,
    }

    Ok(())
}
