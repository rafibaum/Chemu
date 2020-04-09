use std::fs::File;
use std::io::{BufReader, Read};

mod instruction;
mod machine;

fn main() {
    let mut args = std::env::args();
    args.next().unwrap();
    let file_path = args.next().unwrap();
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);

    let mut buffer = [0; 2];
    let mut output = Vec::new();

    'reading: loop {
        let result = reader.read_exact(&mut buffer);
        match result {
            Ok(()) => (),
            Err(e) => {
                match e.kind() {
                    std::io::ErrorKind::UnexpectedEof => break 'reading,
                    _ => panic!(e)
                }
            }
        }

        let instruction = instruction::decode(u16::from_be_bytes(buffer));
        output.push(instruction);
    }

    for instruction in output {
        println!("{:?}", instruction);
    }
}
