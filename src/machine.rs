use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;

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
pub struct NoRegisterError {
    value: u16
}

impl Error for NoRegisterError {}

impl Display for NoRegisterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "No register found for value: {}", self.value)
    }
}

impl TryFrom<u16> for Register {
    type Error = NoRegisterError;

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
            _ => Err(NoRegisterError { value })
        }
    }
}