use clap::Parser;
use rcli::{
    base64_decode, base64_encode, generate_password, process_csv, Base64Subcommand, Opts,
    SubCommand,
};

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
        SubCommand::Base64(subcmd) => match subcmd {
            Base64Subcommand::Decode(opts) => base64_decode(&opts.input, opts.format)?,
            Base64Subcommand::Encode(opts) => base64_encode(&opts.input, opts.format)?,
        },
    }

    Ok(())
}
