use anyhow::Context;
use chipate::{core::Program, Emu};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::try_parse().context("parse command line arguments")?;

    let program = Program::load_from_file(args.file)?;

    let emu = Emu::init_with_program(program);

    println!("{:?}", emu);

    Ok(())
}
