use anyhow::Context;
use chipate::{core::{gfx::Font, Program}, Emu};
use clap::Parser;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(short, long)]
    rom: String,
    #[arg(short, long, default_value_t = 700)]
    instructions_per_second: u16,

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

    let args = Args::parse();

    let program = Program::from_file(args.rom).context("failed to load program rom file")?;
    let font = Font::default();

    Emu::new(args.instructions_per_second, program, font).run()
}
