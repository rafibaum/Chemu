use crate::machine::Register;

/// Represents all the possible instructions that can be encoded in the Chip-8 architecture.
enum Instruction {
    /// Jump to a machine code routine at the specified address. This instruction was only
    /// implemented on the original Chip-8 interpreter and is ignored in modern interpreters.
    Sys { addr: u16 },
    /// Clear the display
    Clr,
    /// Return from a subroutine. Sets the program counter to the address at the top of the stack,
    /// then subtracts 1 from the stack pointer.
    Ret,
    /// Jump to the specified address.
    Jmp { addr: u16 },
    /// Call subroutine at the specified address.
    Call { addr: u16 },
    /// Skips the next instruction if the register value equals the specified value.
    SeImm { register: Register, value: u8 },
    /// Skips the next instruction if the register value is not equal to the specified value.
    SneImm { register: Register, value: u8 },
    /// Skips the next instruction if the two register values are equal to each other.
    SeReg { reg1: Register, reg2: Register },
    /// Load value into register.
    LdImm { register: Register, value: u8 },
    /// Increments the register by the value.
    AddImm { register: Register, value: u8 },
    /// Sets destination register equal to source register.
    LdReg { dest: Register, src: Register },
    /// Performs bitwise OR on both registers and stores in destination register.
    Or { dest: Register, src: Register },
    /// Performs bitwise AND on both registers and stores in destination register.
    And { dest: Register, src: Register },
    /// Performs bitwise XOR on both registers and stores in destination register.
    Xor { dest: Register, src: Register },
    /// Adds two register values and stores result in destination register. If the result is larger
    /// than what can be stored in 8 bits (255), VF is set to 1 and the result is wrapped. Otherwise
    /// VF is set to 0.
    AddReg { dest: Register, src: Register },
    /// Subtracts the source value from the destination, and stores the result in the destination
    /// register. If the destination value is larger than the source, VF is set to 1. Otherwise, VF
    /// is set to 0.
    Sub { dest: Register, src: Register },
    /// Performs a right shift on the source and places the result into the destination. VF is set
    /// to the value of the bit that was shifted.
    Shr { dest: Register, src: Register },
    /// Subtracts the destination value from the source and stores the result in the destination. If
    /// the source is larger than the destination, then VF is set to 1. Otherwise its set to 0.
    SubNeg { dest: Register, src: Register },
    /// Performs a left shift on the source and places the result into the destination. VF is set
    /// to the value of the bit that was shifted.
    Shl { dest: Register, src: Register },
    /// Skips the next instruction if the two registers are not equal.
    SneReg { reg1: Register, reg2: Register },
    /// Set the value of the address register to the specified address.
    LdAddr { addr: u16 },
    /// Jump to the specified location added to the value specified in V0.
    JpOff { base_addr: u16 },
    /// Fetches a random number, performs a bitwise AND with the mask, and stores the result in the
    /// register.
    Rnd { register: Register, mask: u8 },
    /// Draws the sprite stored at the location in the address register of the specified length to
    /// the location specified by the two register values.
    Drw { x: Register, y: Register, length: u8 },
    /// Skip the next instruction if the key with the value in the register is pressed.
    Skp { keycode: Register },
    /// Skip the next instruction if the key with the value in the register is not pressed.
    SkpNeg { keycode: Register },
    /// Read the value of the delay timer and store it in the register.
    ReadDelay { register: Register },
    /// Wait for a key press and then store the value of the key in the register.
    LdKey { register: Register },
    /// Set the delay timer equal to the value in the register.
    StrDelay { register: Register },
    /// Set the sound timer equal to the value in the register.
    StrSound { register: Register },
    /// Increment the address register by the value in the specified register.
    AddAddr { register: Register },
    /// Set the address register to the location in memory of the sprite representing the
    /// hexadecimal digit stored in the specified register.
    LdDigit { register: Register },
    /// Stores the binary coded decimal representation of the number in the specified register at
    /// the location specified by the address register. The first byte stores the hundreds digit,
    /// the next the tens digit, and then the ones digit.
    LdBcd { register: Register },
    /// Stores the value of registers V0 through the specified register at the location specified
    /// by the address register.
    StrArray { end: Register },
    /// Loads the value of registers V0 through the specified register from the location specified
    /// by the address register.
    LdArray { end: Register },
}