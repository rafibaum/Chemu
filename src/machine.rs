use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
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

pub struct Machine {
    registers: Vec<u8>,
    address_register: u16,
    program_counter: u16,
    stack_pointer: u16,
    delay_timer: u8,
    sound_timer: u8,
    memory: Vec<u8>,
}

impl Machine {
    pub fn from_file(file: &mut File) -> Result<Machine, std::io::Error> {
        let mut memory = Vec::with_capacity(MEMORY_SIZE);

        // Copy program data into memory
        if let Err(e) = file.read_exact(&mut memory[PROGRAM_START..]) {
            if e.kind() != std::io::ErrorKind::UnexpectedEof {
                return Err(e);
            }
        };

        // Copy digit layouts into memory
        for (i, byte) in DIGITS.iter().enumerate() {
            memory[i] = *byte;
        }

        Ok(Machine {
            registers: Vec::new(),
            address_register: 0,
            program_counter: PROGRAM_START as u16,
            stack_pointer: STACK_START as u16,
            delay_timer: 0,
            sound_timer: 0,
            memory,
        })
    }
}

/// Represents all the registers directly available to programs in the Chip-8 architecture. Each
/// stores a byte of information.
#[derive(Debug)]
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

impl Display for RegisterParseError {
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
