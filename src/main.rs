use std::fs::File;
use std::io::{BufReader, Read};

mod instruction;
mod machine;

fn main() {
    let mut args = std::env::args();
    args.next().unwrap(); // Skip first argument (executable name)

    let file_path = match args.next() {
        Some(path) => path,
        None => {
            println!("No CHIP-8 program passed in");
            return;
        }
    };

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            println!("Could not open file");
            println!("Cause: {}", e);
            return;
        }
    };

    let mut reader = BufReader::new(file);

    let mut buffer = [0; 2];
    let mut bytes_read = 0;
    let mut output = Vec::new();

    'reading: loop {
        let result = reader.read_exact(&mut buffer);
        if let Err(e) = result {
            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                break 'reading;
            } else {
                println!("An IO error has occurred");
                println!("Cause: {}", e);
                return;
            }
        }

        let instruction = instruction::decode(u16::from_be_bytes(buffer));
        match instruction {
            Ok(decoded) => output.push(decoded),
            Err(e) => {
                println!("Instruction decoding error at: 0x{:X}", bytes_read);
                println!("Cause: {}", e);
                return;
            }
        }

        bytes_read += 2;
    }

    for instruction in output {
        println!("{:?}", instruction);
    }
}
