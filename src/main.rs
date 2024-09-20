use anyhow::Context;
use chipate::{core::Program, Emu};
use clap::Parser;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .pretty()
        .with_level(true)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let args = Args::try_parse().context("failed to parse command line arguments")?;

    let program = Program::load_from_file(args.file)?;

    let emu = Emu::init_with_program(program);

    println!("{:?}", emu);

    Ok(())
}
