use std::fs::{self};

use anyhow::Ok;
use clap::Parser;
use rcli::{
    base64_decode, base64_encode, generate_password, get_content, get_reader, process_csv,
    process_text_generate_key, process_text_sign, process_text_verify, Base64Subcommand, Opts,
    SubCommand, TextSubcommand,
};
use zxcvbn::zxcvbn;

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output, opts.format)?,
        SubCommand::Genpass(opts) => {
            let ret = generate_password(
                opts.length,
                opts.upper,
                opts.lower,
                opts.number,
                opts.symbol,
            )?;
            println!("{}", ret);
            let estimate = zxcvbn(&ret, &[]);
            eprintln!("Estimated password strength: {:?}", estimate.score());
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64Subcommand::Decode(opts) => base64_decode(&opts.input, opts.format)?,
            Base64Subcommand::Encode(opts) => base64_encode(&opts.input, opts.format)?,
        },
        SubCommand::Text(cmd) => match cmd {
            TextSubcommand::Sign(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let ret = process_text_sign(&mut reader, opts.format, &key)?;
                println!("{:?}", ret);
            }
            TextSubcommand::Verify(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.input)?;
                let signature = get_content(&opts.signature)?;
                let ret = process_text_verify(&mut reader, &key, &signature, opts.format)?;
                println!("{:?}", ret);
            }
            TextSubcommand::Keygen(opts) => {
                let map = process_text_generate_key(opts.format)?;
                for (k, v) in map {
                    fs::write(opts.output_path.join(k), v)?;
                }
            }
        },
    }

    Ok(())
}
