pub mod core;

use crate::core::{
    cpu::{Mode, CPU},
    memory::RAM,
    Font, Program,
};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};
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
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    A,
    B,
    C,
    D,
    E,
    F,
}

impl Key {
    fn idx(&self) -> usize {
        match self {
            Key::Num0 => 0x0,
            Key::Num1 => 0x1,
            Key::Num2 => 0x2,
            Key::Num3 => 0x3,
            Key::Num4 => 0x4,
            Key::Num5 => 0x5,
            Key::Num6 => 0x6,
            Key::Num7 => 0x7,
            Key::Num8 => 0x8,
            Key::Num9 => 0x9,
            Key::A => 0xA,
            Key::B => 0xB,
            Key::C => 0xC,
            Key::D => 0xD,
            Key::E => 0xE,
            Key::F => 0xF,
        }
    }
    fn from_idx(idx: usize) -> Self {
        match idx {
            0x0 => Key::Num0,
            0x1 => Key::Num1,
            0x2 => Key::Num2,
            0x3 => Key::Num3,
            0x4 => Key::Num4,
            0x5 => Key::Num5,
            0x6 => Key::Num6,
            0x7 => Key::Num7,
            0x8 => Key::Num8,
            0x9 => Key::Num9,
            0xA => Key::A,
            0xB => Key::B,
            0xC => Key::C,
            0xD => Key::D,
            0xE => Key::E,
            0xF => Key::F,
            _ => panic!("unknown Key index: {}", idx),
        }
    }
}

impl From<usize> for Key {
    fn from(value: usize) -> Self {
        Key::from_idx(value)
    }
}

impl From<Key> for usize {
    fn from(value: Key) -> Self {
        value.idx()
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
    pub fn mark_key_pressed(&mut self, key: Key) {
        tracing::debug!("{:?} key pressed", key);

        let idx = key.idx();

        self.keys[idx] = true;
    }
    pub fn is_key_pressed(&self, key: Key) -> bool {
        let idx: usize = key.into();

        self.keys[idx]
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

        let mut event_pump = match sdl_context.event_pump() {
            Err(msg) => anyhow::bail!(msg),
            Ok(event_pump) => event_pump,
        };

        'main: loop {
            canvas.set_draw_color(Color::BLACK);
            canvas.clear();

            for event in event_pump.poll_iter() {
                match event {
                    Event::KeyUp {
                        keycode: Some(keycode),
                        ..
                    } => match keycode {
                        Keycode::Escape => break 'main,
                        Keycode::Num1 => self.keyboard.mark_key_pressed(Key::Num1),
                        Keycode::Num2 => self.keyboard.mark_key_pressed(Key::Num2),
                        Keycode::Num3 => self.keyboard.mark_key_pressed(Key::Num3),
                        Keycode::Num4 => self.keyboard.mark_key_pressed(Key::C),
                        Keycode::Q => self.keyboard.mark_key_pressed(Key::Num4),
                        Keycode::W => self.keyboard.mark_key_pressed(Key::Num5),
                        Keycode::E => self.keyboard.mark_key_pressed(Key::Num6),
                        Keycode::R => self.keyboard.mark_key_pressed(Key::D),
                        Keycode::A => self.keyboard.mark_key_pressed(Key::Num7),
                        Keycode::S => self.keyboard.mark_key_pressed(Key::Num8),
                        Keycode::D => self.keyboard.mark_key_pressed(Key::Num9),
                        Keycode::F => self.keyboard.mark_key_pressed(Key::E),
                        Keycode::Z => self.keyboard.mark_key_pressed(Key::A),
                        Keycode::X => self.keyboard.mark_key_pressed(Key::Num0),
                        Keycode::C => self.keyboard.mark_key_pressed(Key::B),
                        Keycode::V => self.keyboard.mark_key_pressed(Key::F),
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
                if self.cpu.is_sound_playable() {
                    // TODO: sdl2 audio instead of bell char
                    print!("\u{7}");
                }

                last_timer = Instant::now();
            }

            canvas.set_draw_color(Color::WHITE);

            for c in 0..DISPLAY_PIXELS_WIDTH {
                for r in 0..DISPLAY_PIXELS_HEIGHT {
                    let idx = (r as i32 * DISPLAY_PIXELS_WIDTH as i32) + c as i32;

                    if self.display.read_pixel(idx as u16) {
                        // window is a factor of 10 larger than display state grid
                        let x = (c as i32 % DISPLAY_PIXELS_WIDTH as i32) * 10;
                        let y = (r as i32 % DISPLAY_PIXELS_HEIGHT as i32) * 10;

                        let rect = Rect::new(x, y, 10, 10);
                        if let Err(msg) = canvas.fill_rect(rect) {
                            tracing::error!("fill rect error: {}", msg);
                        }
                    }
                }
            }

            canvas.present();
        }

        tracing::debug!("exited main loop");

        Ok(())
    }
}
