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
            eprintln!("No CHIP-8 program passed in");
            return;
        }
    };

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Could not open file");
            eprintln!("Cause: {}", e);
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
                eprintln!("An IO error has occurred");
                eprintln!("Cause: {}", e);
                return;
            }
        }

        let instruction = instruction::decode(u16::from_be_bytes(buffer));
        match instruction {
            Ok(decoded) => output.push(decoded),
            Err(e) => {
                eprintln!("Instruction decoding error at: 0x{:X}", bytes_read);
                eprintln!("Cause: {}", e);
                return;
            }
        }

        bytes_read += 2;
    }

    for (i, instruction) in output.iter().enumerate() {
        println!("{}: {:?}", 512 + i * 2, instruction);
    }
}
