# Chemu, a Chip-8 emulator written in Rust
Lately I've become interested in emulators and to learn about how they're written. I'm taking a crack at the Chip-8 
architecture as described [here](http://mattmik.com/files/chip8/mastering/chip8.html). My approach to this will be to
emulate the state of the machine at runtime, decode the instructions on the fly, and change the machine state as 
required. I'll be using SDL to show the display output.