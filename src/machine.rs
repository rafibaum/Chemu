use crate::display::Display;
use crate::instruction::Instruction;
use crate::keyboard::{Key, Keyboard};
use rand::prelude::ThreadRng;
use rand::Rng;

use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::Read;

const DIGITS: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

const PROGRAM_START: usize = 512;
const MEMORY_SIZE: usize = 4096;
const STACK_START: usize = DIGITS.len();
const ADDR_SIZE: usize = 2;
const OPCODE_SIZE: usize = 2;

pub struct Machine {
    registers: Vec<u8>,
    address_register: usize,
    program_counter: usize,
    stack_pointer: usize,
    delay_timer: u8,
    sound_timer: u8,
    memory: Vec<u8>,
    random: ThreadRng,
    display: Display,
    keyboard: Keyboard,
}

impl Machine {
    pub fn from_file(file: &mut File) -> Result<Machine, std::io::Error> {
        let sdl_context = sdl2::init().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let keyboard = Keyboard::new(event_pump);

        let mut memory = vec![0; MEMORY_SIZE];

        // Copy program data into memory
        if let Err(e) = file.read_exact(&mut memory[PROGRAM_START..]) {
            if e.kind() != std::io::ErrorKind::UnexpectedEof {
                return Err(e);
            }
        };

        // Copy digit layouts into memory
        memory[0..DIGITS.len()].copy_from_slice(&DIGITS);

        Ok(Machine {
            registers: vec![0; 16],
            address_register: 0,
            program_counter: PROGRAM_START,
            stack_pointer: STACK_START,
            delay_timer: 0,
            sound_timer: 0,
            memory,
            random: rand::thread_rng(),
            display: Display::new(sdl_context, 640, 320),
            keyboard,
        })
    }

    pub fn exec_next(&mut self) {
        let encoded = &self.memory[self.program_counter..self.program_counter + OPCODE_SIZE];
        let instr =
            crate::instruction::decode(u16::from_be_bytes(encoded.try_into().unwrap())).unwrap();
        self.exec_instr(instr);
    }

    fn exec_instr(&mut self, instr: Instruction) {
        // Instructions that don't alter control-flow go here
        match &instr {
            Instruction::LdImm { register, value } => self.registers[*register as usize] = *value,
            Instruction::AddImm { register, value } => {
                self.registers[*register as usize] =
                    self.registers[*register as usize].wrapping_add(*value)
            }
            Instruction::LdReg { dest, src } => {
                self.registers[*dest as usize] = self.registers[*src as usize]
            }
            Instruction::Or { dest, src } => {
                self.registers[*dest as usize] |= self.registers[*src as usize];
            }
            Instruction::And { dest, src } => {
                self.registers[*dest as usize] &= self.registers[*src as usize];
            }
            Instruction::Xor { dest, src } => {
                self.registers[*dest as usize] ^= self.registers[*src as usize];
            }
            Instruction::AddReg { dest, src } => {
                let (result, overflow) =
                    self.registers[*dest as usize].overflowing_add(self.registers[*src as usize]);
                self.registers[*dest as usize] = result;
                self.registers[Register::VF as usize] = if overflow { 1 } else { 0 };
            }
            Instruction::Sub { dest, src } => {
                let (result, overflow) =
                    self.registers[*dest as usize].overflowing_sub(self.registers[*src as usize]);
                self.registers[*dest as usize] = result;
                self.registers[Register::VF as usize] = if overflow { 1 } else { 0 };
            }
            Instruction::Shr { dest, src } => {
                let value = self.registers[*src as usize];
                let bit = value & 0x1;
                self.registers[*dest as usize] = value >> 1;
                self.registers[Register::VF as usize] = bit;
            }
            Instruction::SubNeg { dest, src } => {
                let (result, overflow) =
                    self.registers[*src as usize].overflowing_sub(self.registers[*dest as usize]);
                self.registers[*dest as usize] = result;
                self.registers[Register::VF as usize] = if overflow { 1 } else { 0 };
            }
            Instruction::Shl { dest, src } => {
                let value = self.registers[*src as usize];
                let bit = value & 0x80;
                self.registers[*dest as usize] = value << 1;
                self.registers[Register::VF as usize] = bit;
            }
            Instruction::LdAddr { addr } => {
                self.address_register = *addr as usize;
            }
            Instruction::Rnd { register, mask } => {
                let val: u8 = self.random.gen();
                self.registers[*register as usize] = val & *mask;
            }
            Instruction::ReadDelay { register } => {
                self.registers[*register as usize] = self.delay_timer;
            }
            Instruction::StrDelay { register } => {
                self.delay_timer = self.registers[*register as usize];
            }
            Instruction::StrSound { register } => {
                self.sound_timer = self.registers[*register as usize];
            }
            Instruction::AddAddr { register } => {
                self.address_register += self.registers[*register as usize] as usize;
            }
            Instruction::LdDigit { register } => {
                self.address_register = (self.registers[*register as usize] * 5) as usize;
            }
            Instruction::LdBcd { register } => {
                let value = self.registers[*register as usize];
                self.memory[self.address_register] = value / 100;
                self.memory[self.address_register + 1] = (value % 100) / 10;
                self.memory[self.address_register + 2] = value % 10;
            }
            Instruction::StrArray { end } => {
                for i in 0..*end as usize {
                    self.memory[self.address_register + i] = self.registers[i];
                }
            }
            Instruction::LdArray { end } => {
                for i in 0..*end as usize {
                    self.registers[i] = self.memory[self.address_register + i];
                }
            }
            Instruction::Clr => self.display.clear(),
            Instruction::Drw { x, y, length } => {
                self.display.draw(
                    self.registers[*x as usize] as usize,
                    self.registers[*y as usize] as usize,
                    &self.memory[self.address_register..self.address_register + *length as usize],
                );
            }
            Instruction::LdKey { register } => loop {
                let Key(key) = self.keyboard.next_key();
                self.registers[*register as usize] = key;
            },
            Instruction::Ret => {}
            Instruction::Jmp { addr: _ } => {}
            Instruction::Call { addr: _ } => {}
            Instruction::SeImm {
                register: _,
                value: _,
            } => {}
            Instruction::SneImm {
                register: _,
                value: _,
            } => {}
            Instruction::SeReg { reg1: _, reg2: _ } => {}
            Instruction::SneReg { reg1: _, reg2: _ } => {}
            Instruction::JmpOff { base_addr: _ } => {}
            Instruction::Skp { keycode: _ } => {}
            Instruction::SkpNeg { keycode: _ } => {}
            _ => unimplemented!(),
        }

        // Instructions that modify the program counter go here
        match instr {
            Instruction::Jmp { addr } => self.program_counter = addr as usize,
            Instruction::Call { addr } => {
                let ret_addr = (self.program_counter + OPCODE_SIZE) as u16;
                self.memory[self.stack_pointer..self.stack_pointer + ADDR_SIZE]
                    .copy_from_slice(&ret_addr.to_be_bytes());
                self.stack_pointer += ADDR_SIZE;
                self.program_counter = addr as usize;
            }
            Instruction::Ret => {
                self.stack_pointer -= ADDR_SIZE;
                self.program_counter = self.read_address(self.stack_pointer) as usize;
            }
            Instruction::SeImm { register, value } => {
                if self.registers[register as usize] == value {
                    self.program_counter += OPCODE_SIZE * 2
                } else {
                    self.program_counter += OPCODE_SIZE;
                }
            }
            Instruction::SneImm { register, value } => {
                if self.registers[register as usize] != value {
                    self.program_counter += OPCODE_SIZE * 2
                } else {
                    self.program_counter += OPCODE_SIZE;
                }
            }
            Instruction::SeReg { reg1, reg2 } => {
                if self.registers[reg1 as usize] == self.registers[reg2 as usize] {
                    self.program_counter += OPCODE_SIZE * 2
                } else {
                    self.program_counter += OPCODE_SIZE;
                }
            }
            Instruction::SneReg { reg1, reg2 } => {
                if self.registers[reg1 as usize] != self.registers[reg2 as usize] {
                    self.program_counter += OPCODE_SIZE * 2
                } else {
                    self.program_counter += OPCODE_SIZE;
                }
            }
            Instruction::JmpOff { base_addr } => {
                self.program_counter =
                    (base_addr + self.registers[Register::V0 as usize] as u16) as usize;
            }
            Instruction::Skp { keycode } => {
                if self
                    .keyboard
                    .is_pressed(Key(self.registers[keycode as usize]))
                {
                    self.program_counter += OPCODE_SIZE * 2;
                } else {
                    self.program_counter += OPCODE_SIZE;
                }
            }
            Instruction::SkpNeg { keycode } => {
                if !self
                    .keyboard
                    .is_pressed(Key(self.registers[keycode as usize]))
                {
                    self.program_counter += OPCODE_SIZE * 2;
                } else {
                    self.program_counter += OPCODE_SIZE;
                }
            }
            _ => self.program_counter += OPCODE_SIZE,
        }
    }

    fn read_address(&self, address: usize) -> u16 {
        u16::from_be_bytes(
            self.memory[address..address + ADDR_SIZE]
                .try_into()
                .unwrap(),
        )
    }

    pub fn decrement_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn process_key_events(&mut self) {
        self.keyboard.process_events();
    }

    pub fn update_display(&mut self) {
        self.display.update();
    }
}

/// Represents all the registers directly available to programs in the Chip-8 architecture. Each
/// stores a byte of information.
#[derive(Copy, Clone, Debug)]
pub enum Register {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    /// VF is frequently written to by instructions that set flags. Programs should not use this
    /// register to store important data.
    VF,
}

#[derive(Debug)]
pub struct RegisterParseError {
    value: u16,
}

impl Error for RegisterParseError {}

impl fmt::Display for RegisterParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "no register found for value: {:X}", self.value)
    }
}

impl TryFrom<u16> for Register {
    type Error = RegisterParseError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Register::V0),
            1 => Ok(Register::V1),
            2 => Ok(Register::V2),
            3 => Ok(Register::V3),
            4 => Ok(Register::V4),
            5 => Ok(Register::V5),
            6 => Ok(Register::V6),
            7 => Ok(Register::V7),
            8 => Ok(Register::V8),
            9 => Ok(Register::V9),
            10 => Ok(Register::VA),
            11 => Ok(Register::VB),
            12 => Ok(Register::VC),
            13 => Ok(Register::VD),
            14 => Ok(Register::VE),
            15 => Ok(Register::VF),
            _ => Err(RegisterParseError { value }),
        }
    }
}
