use anyhow::Context;
use chipate::{
    core::{cpu::Mode, gfx::Font, Program},
    Config, Emu,
};
use clap::Parser;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[arg(short, long)]
    mode: Option<Mode>,
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

    let config = Config {
        mode: args.mode.unwrap_or_default(),
        instructions_per_sec: args.instructions_per_second,
        font: Font::default(),
    };

    let program = Program::from_file(args.rom).context("load program rom file")?;

    let mut emu = Emu::new(config);
    emu.load_program(program);
    emu.run()
}
