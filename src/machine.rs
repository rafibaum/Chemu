/// Represents all the registers directly available to programs in the Chip-8 architecture. Each
/// stores a byte of information.
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