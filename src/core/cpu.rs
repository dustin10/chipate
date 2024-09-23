use crate::core::{
    gfx::{Display, WINDOW_PIXELS_HEIGHT, WINDOW_PIXELS_WIDTH},
    memory::{self, RAM},
};

const PROGRAM_COUNTER_START: u16 = 0x200;

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

#[derive(Clone, Debug)]
enum Instruction {
    AddRegister { v: usize, value: u8 },
    ClearScreen,
    Display { vx: usize, vy: usize, pixels: u8 },
    Jump { address: u16 },
    MachineLanguageRoutine { address: u16 },
    SetIndex { value: u16 },
    SetRegister { v: usize, value: u8 },
    SkipEqual { v: usize, value: u8 },
    SkipEqualReg { vx: usize, vy: usize },
    SkipNotEqual { v: usize, value: u8 },
    SkipNotEqualReg { vx: usize, vy: usize },
    SubroutineCall { address: u16 },
    SubroutineReturn,
}

impl Instruction {
    fn from_op_code(op_code: u16) -> Option<Instruction> {
        // precompute X, Y, N, NN and NNN nibbles
        let x = (op_code & 0x0F00) >> 8;
        let y = (op_code & 0x00F0) >> 4;
        let n = op_code & 0x000F;
        let nn = (op_code & 0x00FF) as u8;
        let nnn = (op_code & 0x0FFF);

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
            0x6000 => Some(Instruction::SetRegister {
                v: x as usize,
                value: nn,
            }),
            0x7000 => Some(Instruction::AddRegister {
                v: x as usize,
                value: nn,
            }),
            0x9000 => Some(Instruction::SkipNotEqualReg {
                vx: x as usize,
                vy: y as usize,
            }),
            0xA000 => Some(Instruction::SetIndex { value: nnn }),
            0xD000 => Some(Instruction::Display {
                vx: x as usize,
                vy: y as usize,
                pixels: n as u8,
            }),
            _ => None,
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::AddRegister { v, value } => {
                f.write_str(&format!("add v{} {:#04x}", v, value))
            }
            Instruction::ClearScreen => f.write_str("clear"),
            Instruction::Display { vx, vy, pixels } => {
                f.write_str(&format!("disp v{} v{} {:#04x}", vx, vy, pixels))
            }
            Instruction::Jump { address } => f.write_str(&format!("jump {:#04x}", address)),
            Instruction::MachineLanguageRoutine { address } => {
                f.write_str(&format!("mlr {:#04x}", address))
            }
            Instruction::SetIndex { value } => f.write_str(&format!("load l {:#04x}", value)),
            Instruction::SetRegister { v, value } => {
                f.write_str(&format!("load v{} {:#04x}", v, value))
            }
            Instruction::SkipEqual { v, value } => {
                f.write_str(&format!("skip_eq v{} {:#04x}", v, value))
            }
            Instruction::SkipEqualReg { vx, vy } => {
                f.write_str(&format!("skip_eq_reg v{} v{}", vx, vy))
            }
            Instruction::SkipNotEqual { v, value } => {
                f.write_str(&format!("skip_neq v{} {:#04x}", v, value))
            }

            Instruction::SkipNotEqualReg { vx, vy } => {
                f.write_str(&format!("skip_neq_reg v{} v{}", vx, vy))
            }
            Instruction::SubroutineCall { address } => {
                f.write_str(&format!("sub_call {:#04x}", address))
            }
            Instruction::SubroutineReturn => f.write_str("sub_ret"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CPU {
    registers: Registers,
    prog_counter: u16,
    stack: Stack,
    delay_timer: u8,
    sound_timer: u8,
    history: Vec<Instruction>,
}

impl CPU {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn tick(&mut self, memory: &mut RAM, display: &mut Display) {
        let op_code = self.fetch(memory);

        match Instruction::from_op_code(op_code) {
            None => tracing::warn!("unknown op code: {:#04x}", op_code),
            Some(instruction) => self.execute(instruction, display),
        }
    }
    fn fetch(&mut self, memory: &mut RAM) -> u16 {
        let high = memory.read(self.prog_counter) as u16;
        let low = memory.read(self.prog_counter + 1) as u16;

        self.prog_counter += 2;

        (high << 8) | low
    }
    fn execute(&mut self, instruction: Instruction, display: &mut Display) {
        tracing::debug!("executing instruction '{}'", instruction);

        match instruction {
            Instruction::AddRegister { v, value } => self.registers.vs[v] += value,
            Instruction::ClearScreen => display.clear(),
            Instruction::Display { vx, vy, pixels } => self.display(display, vx, vy, pixels),
            Instruction::Jump { address } => self.prog_counter = address,
            Instruction::MachineLanguageRoutine { .. } => {
                tracing::info!("machine routine instruction not supported")
            }
            Instruction::SetIndex { value } => self.registers.i = value,
            Instruction::SetRegister { v, value } => self.registers.vs[v] = value,
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
            Instruction::SubroutineCall { address } => {
                self.stack.push(self.prog_counter);
                self.prog_counter = address;
            }
            Instruction::SubroutineReturn => match self.stack.pop() {
                Some(address) => self.prog_counter = address,
                None => tracing::warn!("attempted to pop off of empty stack"),
            },
        }

        self.history.push(instruction);
    }
    fn display(&mut self, display: &mut Display, vx: usize, vy: usize, pixels: u8) {
        let x = self.registers.vs[vx] % WINDOW_PIXELS_HEIGHT;
        let y = self.registers.vs[vy] % WINDOW_PIXELS_WIDTH;

        self.registers.set_f(0);

        tracing::warn!("todo: finish display impl");
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            registers: Registers::default(),
            prog_counter: PROGRAM_COUNTER_START,
            stack: Stack::default(),
            delay_timer: 0,
            sound_timer: 0,
            history: Vec::new(),
        }
    }
}
