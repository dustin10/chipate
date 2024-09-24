pub mod core;

use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

use crate::core::{
    cpu::{Mode, CPU},
    memory::RAM,
    Font, Program,
};

use std::time::Instant;

pub const PROGRAM_START_ADDR: u16 = 0x200;

pub const DISPLAY_PIXELS_WIDTH: u8 = 64;

pub const DISPLAY_PIXELS_HEIGHT: u8 = 32;

const NUM_PIXELS: usize = 64 * 32;

#[derive(Clone, Debug)]
pub struct Config {
    pub mode: Mode,
    pub instructions_per_sec: u16,
    pub font: Font,
}

#[derive(Clone, Debug)]
pub struct DisplayState {
    pixels: [bool; NUM_PIXELS],
}

impl DisplayState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn clear(&mut self) {
        self.pixels.fill(false);
    }
    pub fn read_pixel(&self, idx: u16) -> bool {
        self.pixels[idx as usize]
    }
    pub fn write_pixel(&mut self, idx: u16, value: bool) {
        self.pixels[idx as usize] = value;
    }
}

impl Default for DisplayState {
    fn default() -> Self {
        Self {
            pixels: [false; NUM_PIXELS],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Key {
    Num1,
    Num2,
    Num3,
    Num4,
    Q,
    W,
    E,
    R,
    A,
    S,
    D,
    F,
    Z,
    X,
    C,
    V,
}

impl Key {
    fn get_state_idx(&self) -> usize {
        match self {
            Key::Num1 => 0x1,
            Key::Num2 => 0x2,
            Key::Num3 => 0x3,
            Key::Num4 => 0xC,
            Key::Q => 0x4,
            Key::W => 0x5,
            Key::E => 0x6,
            Key::R => 0xD,
            Key::A => 0x7,
            Key::S => 0x8,
            Key::D => 0x9,
            Key::F => 0xE,
            Key::Z => 0xA,
            Key::X => 0x0,
            Key::C => 0xB,
            Key::V => 0xF,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct KeyState {
    keys: [bool; 16],
}

impl KeyState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn reset(&mut self) {
        self.keys.fill(false);
    }
    pub fn key_pressed(&mut self, key: Key) {
        let idx = key.get_state_idx();

        self.keys[idx] = true;
    }
    pub fn get_pressed_key(&self) -> Option<u8> {
        self.keys
            .iter()
            .enumerate()
            .find_map(|(idx, v)| if *v { Some(idx as u8) } else { None })
    }
}

#[derive(Clone, Debug)]
pub struct Emu {
    config: Config,
    cpu: CPU,
    memory: RAM,
    display: DisplayState,
    keyboard: KeyState,
}

impl Emu {
    pub fn new(config: Config) -> Self {
        let mut memory = RAM::new();

        config.font.load(&mut memory);
        tracing::debug!("loaded {} font into memory", config.font.name);

        Self {
            config,
            cpu: CPU::default(),
            memory,
            display: DisplayState::default(),
            keyboard: KeyState::default(),
        }
    }
    pub fn load_program(&mut self, program: Program) {
        program.load(&mut self.memory);
        tracing::debug!("loaded {} program into memory", program.name);
    }
    pub fn run(&mut self) -> anyhow::Result<()> {
        let min_ms_per_tick = 1000_u128 / self.config.instructions_per_sec as u128;
        let mut last_tick = Instant::now();

        let min_ms_per_timer_dec = 1000_u128 / 60_u128;
        let mut last_timer = Instant::now();

        let sdl_context = match sdl2::init() {
            Err(msg) => anyhow::bail!(msg),
            Ok(ctx) => ctx,
        };

        let video_subsystem = match sdl_context.video() {
            Err(msg) => anyhow::bail!(msg),
            Ok(video_subsystem) => video_subsystem,
        };

        let window = match video_subsystem
            .window("chipate", 640, 320)
            .position_centered()
            .build()
        {
            Err(msg) => anyhow::bail!(msg),
            Ok(window) => window,
        };

        let mut canvas = match window.into_canvas().build() {
            Err(msg) => anyhow::bail!(msg),
            Ok(canvas) => canvas,
        };

        canvas.set_draw_color(Color::BLACK);

        let mut event_pump = match sdl_context.event_pump() {
            Err(msg) => anyhow::bail!(msg),
            Ok(event_pump) => event_pump,
        };

        'main: loop {
            canvas.clear();

            for event in event_pump.poll_iter() {
                match event {
                    Event::KeyUp {
                        keycode: Some(keycode),
                        ..
                    } => match keycode {
                        Keycode::Escape => break 'main,
                        Keycode::Num1 => self.keyboard.key_pressed(Key::Num1),
                        Keycode::Num2 => self.keyboard.key_pressed(Key::Num2),
                        Keycode::Num3 => self.keyboard.key_pressed(Key::Num3),
                        Keycode::Num4 => self.keyboard.key_pressed(Key::Num4),
                        Keycode::Q => self.keyboard.key_pressed(Key::Q),
                        Keycode::W => self.keyboard.key_pressed(Key::W),
                        Keycode::E => self.keyboard.key_pressed(Key::E),
                        Keycode::R => self.keyboard.key_pressed(Key::R),
                        Keycode::A => self.keyboard.key_pressed(Key::A),
                        Keycode::S => self.keyboard.key_pressed(Key::S),
                        Keycode::D => self.keyboard.key_pressed(Key::D),
                        Keycode::F => self.keyboard.key_pressed(Key::F),
                        Keycode::Z => self.keyboard.key_pressed(Key::Z),
                        Keycode::X => self.keyboard.key_pressed(Key::X),
                        Keycode::C => self.keyboard.key_pressed(Key::C),
                        Keycode::V => self.keyboard.key_pressed(Key::V),
                        _ => {}
                    },
                    _ => {}
                }
            }

            let tick_elapsed = last_tick.elapsed();
            if tick_elapsed.as_millis() >= min_ms_per_tick {
                self.cpu.tick(
                    &mut self.memory,
                    &mut self.display,
                    &self.config.font,
                    &self.keyboard,
                );

                last_tick = Instant::now();
                self.keyboard.reset();
            }

            let timer_elapsed = last_timer.elapsed();
            if timer_elapsed.as_millis() >= min_ms_per_timer_dec {
                self.cpu.dec_timers();
                last_timer = Instant::now();
            }

            // TODO: draw based on display state

            canvas.present();
        }

        tracing::debug!("exited main loop");

        Ok(())
    }
}

fn _print_display_state(display: &DisplayState) {
    let mut grid = String::new();
    for r in 0..32 {
        grid.push('\n');
        for c in 0..64 {
            let idx = r as u16 * 64 + c as u16;

            let white = display.read_pixel(idx);
            if white {
                grid.push('\u{2588}');
            } else {
                grid.push(' ');
            }
        }
    }
    println!("{}", grid);
}
