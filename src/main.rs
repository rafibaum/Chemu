use crate::machine::Machine;
use std::fs::File;
use std::time::{Duration, Instant};

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

    let mut machine = match Machine::from_file(&mut file) {
        Ok(machine) => machine,
        Err(e) => {
            eprintln!("Couldn't read file");
            eprintln!("Cause: {}", e);
            return;
        }
    };


    let timer_delta = Duration::from_secs_f64(1.0 / 60.0);
    let mut tick_deadline = Instant::now();
    loop {
        machine.exec_next();
        while tick_deadline.elapsed() >= timer_delta {
            machine.decrement_timers();
            tick_deadline += timer_delta;
        }
    }
}
