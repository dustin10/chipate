use crate::{
    core::memory::RAM, DisplayState, Font, Key, KeyState, DISPLAY_PIXELS_HEIGHT,
    DISPLAY_PIXELS_WIDTH,
};

use rand::{rngs::ThreadRng, Rng};
use std::collections::VecDeque;

const PROGRAM_COUNTER_START: u16 = 0x200;

const MAX_HISTORY_SIZE: usize = 100;

#[derive(Clone, Debug, Default)]
struct Registers {
    vs: [u8; 16],
    i: u16,
}

impl Registers {
    fn set_f(&mut self, value: u8) {
        self.vs[15] = value;
    }
}

#[derive(Clone, Debug, Default)]
struct Stack {
    data: Vec<u16>,
}

impl Stack {
    fn push(&mut self, address: u16) {
        self.data.push(address);
    }
    fn pop(&mut self) -> Option<u16> {
        self.data.pop()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    Classic,
    Modern,
}

impl From<String> for Mode {
    fn from(value: String) -> Self {
        if value.as_str() == "classic" {
            Mode::Classic
        } else {
            Mode::Modern
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self::Modern
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    Add { vx: usize, vy: usize },
    AddIndex { v: usize },
    AddRegister { v: usize, value: u8 },
    And { vx: usize, vy: usize },
    BcdConversion { v: usize },
    ClearScreen,
    DelayTimerLoad { v: usize },
    DelayTimerSet { v: usize },
    Display { vx: usize, vy: usize, pixels: u8 },
    GetKey { v: usize },
    Jump { address: u16 },
    Load { n: usize },
    LoadFontChar { v: usize },
    MachineLanguageRoutine { address: u16 },
    Or { vx: usize, vy: usize },
    Random { v: usize, value: u8 },
    SetIndex { value: u16 },
    Set { v: usize, value: u8 },
    SetRegister { vx: usize, vy: usize },
    ShiftLeft { vx: usize, vy: usize },
    ShiftRight { vx: usize, vy: usize },
    SkipEqual { v: usize, value: u8 },
    SkipEqualReg { vx: usize, vy: usize },
    SkipIfKeyNotPressed { v: usize },
    SkipIfKeyPressed { v: usize },
    SkipNotEqual { v: usize, value: u8 },
    SkipNotEqualReg { vx: usize, vy: usize },
    SoundTimerSet { v: usize },
    Store { n: usize },
    Subtract { vx: usize, vy: usize },
    SubtractRev { vx: usize, vy: usize },
    SubroutineCall { address: u16 },
    SubroutineReturn,
    Xor { vx: usize, vy: usize },
}

impl Instruction {
    fn from_op_code(op_code: u16) -> Option<Instruction> {
        // precompute X, Y, N, NN and NNN nibbles
        let x = (op_code & 0x0F00) >> 8;
        let y = (op_code & 0x00F0) >> 4;
        let n = op_code & 0x000F;
        let nn = (op_code & 0x00FF) as u8;
        let nnn = op_code & 0x0FFF;

        // match on first nibble and proceed from there
        match op_code & 0xF000 {
            0x0000 => match nnn {
                0x0E0 => Some(Instruction::ClearScreen),
                0x0EE => Some(Instruction::SubroutineReturn),
                _ => Some(Instruction::MachineLanguageRoutine { address: nnn }),
            },
            0x1000 => Some(Instruction::Jump { address: nnn }),
            0x2000 => Some(Instruction::SubroutineCall { address: nnn }),
            0x3000 => Some(Instruction::SkipEqual {
                v: x as usize,
                value: nn,
            }),
            0x4000 => Some(Instruction::SkipNotEqual {
                v: x as usize,
                value: nn,
            }),
            0x5000 => Some(Instruction::SkipEqualReg {
                vx: x as usize,
                vy: y as usize,
            }),
            0x6000 => Some(Instruction::Set {
                v: x as usize,
                value: nn,
            }),
            0x7000 => Some(Instruction::AddRegister {
                v: x as usize,
                value: nn,
            }),
            0x8000 => match n {
                0x0 => Some(Instruction::SetRegister {
                    vx: x as usize,
                    vy: y as usize,
                }),
                0x1 => Some(Instruction::Or {
                    vx: x as usize,
                    vy: y as usize,
                }),
                0x2 => Some(Instruction::And {
                    vx: x as usize,
                    vy: y as usize,
                }),
                0x3 => Some(Instruction::Xor {
                    vx: x as usize,
                    vy: y as usize,
                }),
                0x4 => Some(Instruction::Add {
                    vx: x as usize,
                    vy: y as usize,
                }),
                0x5 => Some(Instruction::Subtract {
                    vx: x as usize,
                    vy: y as usize,
                }),
                0x6 => Some(Instruction::ShiftRight {
                    vx: x as usize,
                    vy: y as usize,
                }),
                0x7 => Some(Instruction::SubtractRev {
                    vx: x as usize,
                    vy: y as usize,
                }),
                0xE => Some(Instruction::ShiftLeft {
                    vx: x as usize,
                    vy: y as usize,
                }),
                _ => None,
            },
            0x9000 => Some(Instruction::SkipNotEqualReg {
                vx: x as usize,
                vy: y as usize,
            }),
            0xA000 => Some(Instruction::SetIndex { value: nnn }),
            0xC000 => Some(Instruction::Random {
                v: x as usize,
                value: nn,
            }),
            0xD000 => Some(Instruction::Display {
                vx: x as usize,
                vy: y as usize,
                pixels: n as u8,
            }),
            0xE000 => match nn {
                0x9E => Some(Instruction::SkipIfKeyPressed { v: x as usize }),
                0xA1 => Some(Instruction::SkipIfKeyNotPressed { v: x as usize }),
                _ => None,
            },
            0xF000 => match nn {
                0x07 => Some(Instruction::DelayTimerLoad { v: x as usize }),
                0x0A => Some(Instruction::GetKey { v: x as usize }),
                0x15 => Some(Instruction::DelayTimerSet { v: x as usize }),
                0x18 => Some(Instruction::SoundTimerSet { v: x as usize }),
                0x1E => Some(Instruction::AddIndex { v: x as usize }),
                0x29 => Some(Instruction::LoadFontChar { v: x as usize }),
                0x33 => Some(Instruction::BcdConversion { v: x as usize }),
                0x55 => Some(Instruction::Store { n: x as usize }),
                0x65 => Some(Instruction::Load { n: x as usize }),
                _ => None,
            },
            _ => None,
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Add { vx, vy } => f.write_str(&format!("add v{} v{}", vx, vy)),
            Instruction::AddIndex { v } => f.write_str(&format!("add_i v{}", v)),
            Instruction::AddRegister { v, value } => {
                f.write_str(&format!("add v{} {:#04x}", v, value))
            }
            Instruction::And { vx, vy } => f.write_str(&format!("and v{} v{}", vx, vy)),
            Instruction::BcdConversion { v } => f.write_str(&format!("bcd_cnv v{}", v)),
            Instruction::ClearScreen => f.write_str("clear"),
            Instruction::DelayTimerLoad { v } => f.write_str(&format!("delay_load v{}", v)),
            Instruction::DelayTimerSet { v } => f.write_str(&format!("delay_set v{}", v)),
            Instruction::Display { vx, vy, pixels } => {
                f.write_str(&format!("disp v{} v{} {:#04x}", vx, vy, pixels))
            }
            Instruction::GetKey { v } => f.write_str(&format!("get_key v{}", v)),
            Instruction::Jump { address } => f.write_str(&format!("jump {:#04x}", address)),
            Instruction::Load { n } => f.write_str(&format!("load {}", n)),
            Instruction::LoadFontChar { v } => f.write_str(&format!("load_font_ch v{}", v)),
            Instruction::MachineLanguageRoutine { address } => {
                f.write_str(&format!("mlr {:#04x}", address))
            }
            Instruction::Or { vx, vy } => f.write_str(&format!("or v{} v{}", vx, vy)),
            Instruction::Random { v, value } => f.write_str(&format!("rand v{} {:#04x}", v, value)),
            Instruction::SetIndex { value } => f.write_str(&format!("set i {:#04x}", value)),
            Instruction::Set { v, value } => f.write_str(&format!("set v{} {:#04x}", v, value)),
            Instruction::SetRegister { vx, vy } => f.write_str(&format!("set v{} v{}", vx, vy)),
            Instruction::ShiftLeft { vx, vy } => f.write_str(&format!("shift_l v{} v{}", vx, vy)),
            Instruction::ShiftRight { vx, vy } => f.write_str(&format!("shift_r v{} v{}", vx, vy)),
            Instruction::SkipEqual { v, value } => {
                f.write_str(&format!("skip_eq v{} {:#04x}", v, value))
            }
            Instruction::SkipEqualReg { vx, vy } => {
                f.write_str(&format!("skip_eq_reg v{} v{}", vx, vy))
            }
            Instruction::SkipIfKeyNotPressed { v } => f.write_str(&format!("skip_key v{}", v)),
            Instruction::SkipIfKeyPressed { v } => f.write_str(&format!("skip_not_key v{}", v)),
            Instruction::SkipNotEqual { v, value } => {
                f.write_str(&format!("skip_neq v{} {:#04x}", v, value))
            }
            Instruction::SkipNotEqualReg { vx, vy } => {
                f.write_str(&format!("skip_neq_reg v{} v{}", vx, vy))
            }
            Instruction::SoundTimerSet { v } => f.write_str(&format!("sound_set v{}", v)),
            Instruction::Store { n } => f.write_str(&format!("store {}", n)),
            Instruction::Subtract { vx, vy } => f.write_str(&format!("sub v{} v{}", vx, vy)),
            Instruction::SubtractRev { vx, vy } => f.write_str(&format!("sub_rev v{} v{}", vx, vy)),
            Instruction::SubroutineCall { address } => {
                f.write_str(&format!("sub_call {:#04x}", address))
            }
            Instruction::SubroutineReturn => f.write_str("sub_ret"),
            Instruction::Xor { vx, vy } => f.write_str(&format!("xor v{} v{}", vx, vy)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CPU {
    mode: Mode,
    registers: Registers,
    prog_counter: u16,
    stack: Stack,
    delay_timer: u8,
    sound_timer: u8,
    history: VecDeque<Instruction>,
    rand_gen: ThreadRng,
}

impl CPU {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn tick(
        &mut self,
        memory: &mut RAM,
        display: &mut DisplayState,
        font: &Font,
        keyboard: &KeyState,
    ) {
        let op_code = self.fetch(memory);

        match Instruction::from_op_code(op_code) {
            None => tracing::warn!("unknown op code: {:#04x}", op_code),
            Some(instruction) => self.execute(instruction, memory, display, font, keyboard),
        }
    }
    pub fn dec_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
    pub fn is_sound_playable(&self) -> bool {
        self.sound_timer > 0
    }
    fn fetch(&mut self, memory: &mut RAM) -> u16 {
        let high = memory.read(self.prog_counter) as u16;
        let low = memory.read(self.prog_counter + 1) as u16;

        self.prog_counter += 2;

        (high << 8) | low
    }
    fn execute(
        &mut self,
        instruction: Instruction,
        memory: &mut RAM,
        display: &mut DisplayState,
        font: &Font,
        keyboard: &KeyState,
    ) {
        tracing::debug!("executing instruction '{}'", instruction);

        match instruction {
            Instruction::Add { vx, vy } => {
                let (value, overflowed) =
                    self.registers.vs[vx].overflowing_add(self.registers.vs[vy]);

                self.registers.vs[vx] = value;

                if overflowed {
                    self.registers.set_f(1);
                } else {
                    self.registers.set_f(0);
                }
            }
            Instruction::AddIndex { v } => {
                self.registers.i += self.registers.vs[v] as u16;
                if self.registers.i >= 0x1000 {
                    self.registers.set_f(1);
                }
            }
            Instruction::AddRegister { v, value } => {
                let (result, _) = self.registers.vs[v].overflowing_add(value);
                self.registers.vs[v] = result;
            }
            Instruction::And { vx, vy } => self.registers.vs[vx] &= self.registers.vs[vy],
            Instruction::BcdConversion { v } => {
                let value = self.registers.vs[v];

                memory.write(self.registers.i, value / 100);
                memory.write(self.registers.i + 1, (value % 100) / 10);
                memory.write(self.registers.i + 2, value % 10);
            }
            Instruction::ClearScreen => display.clear(),
            Instruction::DelayTimerLoad { v } => self.delay_timer = self.registers.vs[v],
            Instruction::DelayTimerSet { v } => self.delay_timer = self.registers.vs[v],
            Instruction::Display { vx, vy, pixels } => {
                self.display(memory, display, vx, vy, pixels)
            }
            Instruction::GetKey { v } => {
                if let Some(key) = keyboard.get_pressed_key() {
                    self.registers.vs[v] = key;
                } else {
                    self.prog_counter -= 2;
                }
            }
            Instruction::Jump { address } => self.prog_counter = address,
            Instruction::Load { n } => match self.mode {
                Mode::Classic => {
                    for i in 0..=n {
                        self.registers.vs[i] = memory.read(self.registers.i);
                        self.registers.i += 1;
                    }
                }
                Mode::Modern => {
                    for i in 0..=n {
                        self.registers.vs[i] = memory.read(self.registers.i + i as u16);
                    }
                }
            },
            Instruction::LoadFontChar { v } => {
                let char = self.registers.vs[v];
                self.registers.i = font.char_addr(char);
            }
            Instruction::MachineLanguageRoutine { .. } => {
                tracing::info!("machine routine instruction not supported")
            }
            Instruction::Or { vx, vy } => self.registers.vs[vx] |= self.registers.vs[vy],
            Instruction::Random { v, value } => {
                self.registers.vs[v] = self.rand_gen.gen_range(0..value) & value
            }
            Instruction::SetIndex { value } => self.registers.i = value,
            Instruction::Set { v, value } => self.registers.vs[v] = value,
            Instruction::SetRegister { vx, vy } => self.registers.vs[vx] = self.registers.vs[vy],
            Instruction::ShiftLeft { vx, vy } => {
                if self.mode == Mode::Classic {
                    self.registers.vs[vx] = self.registers.vs[vy];
                }

                let overflow = (self.registers.vs[vx] & 0x80) != 0;
                let value = self.registers.vs[vx] << 1;

                self.registers.vs[vx] = value;

                if overflow {
                    self.registers.set_f(1)
                } else {
                    self.registers.set_f(0)
                };
            }
            Instruction::ShiftRight { vx, vy } => {
                if self.mode == Mode::Classic {
                    self.registers.vs[vx] = self.registers.vs[vy];
                }

                let underflow = (self.registers.vs[vx] & 0x1) != 0;
                let value = self.registers.vs[vx] >> 1;

                self.registers.vs[vx] = value;

                if underflow {
                    self.registers.set_f(1)
                } else {
                    self.registers.set_f(0)
                };
            }
            Instruction::SkipEqual { v, value } => {
                if self.registers.vs[v] == value {
                    self.prog_counter += 2;
                }
            }
            Instruction::SkipEqualReg { vx, vy } => {
                if self.registers.vs[vx] == self.registers.vs[vy] {
                    self.prog_counter += 2;
                }
            }
            Instruction::SkipIfKeyNotPressed { v } => {
                let key = Key::from(v);

                if !keyboard.is_key_pressed(key) {
                    self.prog_counter += 2;
                }
            }
            Instruction::SkipIfKeyPressed { v } => {
                let key = Key::from(v);

                if keyboard.is_key_pressed(key) {
                    self.prog_counter += 2;
                }
            }
            Instruction::SkipNotEqual { v, value } => {
                if self.registers.vs[v] != value {
                    self.prog_counter += 2;
                }
            }
            Instruction::SkipNotEqualReg { vx, vy } => {
                if self.registers.vs[vx] != self.registers.vs[vy] {
                    self.prog_counter += 2;
                }
            }
            Instruction::SoundTimerSet { v } => self.sound_timer = self.registers.vs[v],
            Instruction::Store { n } => match self.mode {
                Mode::Classic => {
                    for i in 0..=n {
                        memory.write(self.registers.i, self.registers.vs[i]);
                        self.registers.i += 1;
                    }
                }
                Mode::Modern => {
                    for i in 0..=n {
                        memory.write(self.registers.i + i as u16, self.registers.vs[i]);
                    }
                }
            },
            Instruction::Subtract { vx, vy } => {
                let minuend = self.registers.vs[vx];
                let subtrahend = self.registers.vs[vy];

                let (value, overflowed) = minuend.overflowing_sub(subtrahend);

                self.registers.vs[vx] = value;

                if !overflowed {
                    self.registers.set_f(1);
                } else {
                    self.registers.set_f(0);
                }
            }
            Instruction::SubtractRev { vx, vy } => {
                let minuend = self.registers.vs[vy];
                let subtrahend = self.registers.vs[vx];

                let (value, overflowed) = minuend.overflowing_sub(subtrahend);

                self.registers.vs[vx] = value;

                if !overflowed {
                    self.registers.set_f(1);
                } else {
                    self.registers.set_f(0);
                }
            }
            Instruction::SubroutineCall { address } => {
                self.stack.push(self.prog_counter);
                self.prog_counter = address;
            }
            Instruction::SubroutineReturn => match self.stack.pop() {
                Some(address) => self.prog_counter = address,
                None => tracing::warn!("attempted to pop off of empty stack"),
            },
            Instruction::Xor { vx, vy } => self.registers.vs[vx] ^= self.registers.vs[vy],
        }

        if self.history.len() == MAX_HISTORY_SIZE {
            self.history.pop_front();
        }

        self.history.push_back(instruction);
    }
    fn display(
        &mut self,
        memory: &mut RAM,
        display: &mut DisplayState,
        vx: usize,
        vy: usize,
        pixels: u8,
    ) {
        let mut x = self.registers.vs[vx] % DISPLAY_PIXELS_WIDTH;
        let mut y = self.registers.vs[vy] % DISPLAY_PIXELS_HEIGHT;

        self.registers.set_f(0);

        'rows: for i in 0..pixels {
            let b = memory.read(self.registers.i + i as u16);

            'cols: for j in 0..8 {
                let px = b & (0x1 << (7 - j));
                let idx = y as u16 * DISPLAY_PIXELS_WIDTH as u16 + x as u16;

                let px_current = display.read_pixel(idx);
                display.write_pixel(idx, px_current ^ (px != 0));
                if px_current && ((px != 0) ^ px_current) {
                    self.registers.set_f(1);
                }

                x += 1;
                if x >= DISPLAY_PIXELS_WIDTH {
                    break 'cols;
                }
            }

            y += 1;
            if y >= DISPLAY_PIXELS_HEIGHT {
                break 'rows;
            }

            x = self.registers.vs[vx] % DISPLAY_PIXELS_WIDTH;
        }
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            mode: Mode::default(),
            registers: Registers::default(),
            prog_counter: PROGRAM_COUNTER_START,
            stack: Stack::default(),
            delay_timer: 0,
            sound_timer: 0,
            history: VecDeque::with_capacity(MAX_HISTORY_SIZE),
            rand_gen: ThreadRng::default(),
        }
    }
}
