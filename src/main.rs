use crate::machine::Machine;
use std::fs::File;

mod display;
mod instruction;
mod machine;

fn main() {
    let mut args = std::env::args();
    args.next().unwrap(); // Skip first argument (executable name)

    let file_path = match args.next() {
        Some(path) => path,
        None => {
            eprintln!("No CHIP-8 program passed in");
            return;
        }
    };

    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Could not open file");
            eprintln!("Cause: {}", e);
            return;
        }
    };

    let _machine = match Machine::from_file(&mut file) {
        Ok(machine) => machine,
        Err(e) => {
            eprintln!("Couldn't read file");
            eprintln!("Cause: {}", e);
            return;
        }
    };
}
