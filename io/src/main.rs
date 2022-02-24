use std::env;
use std::fs;
use std::io::{Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() < 3 {
        eprintln!("usage: {} <from> <to>", program);
        return;
    }

    if let Err(err) = io_process(&args[1], &args[2]) {
        eprintln!("{}", err)
    }
}

fn io_process(input_fname: &str, output_fname: &str) -> Result<(), String> {
    let mut contents = Vec::new();

    let mut input_file = fs::File::open(input_fname)
        .map_err(|err| format!("error opening input {}: {}", input_fname, err))?;
    input_file
        .read_to_end(&mut contents)
        .map_err(|err| format!("read error: {}", err))?;

    let mut output_file = fs::File::create(output_fname)
        .map_err(|err| format!("error opening output {}: {}", output_fname, err))?;
    output_file
        .write_all(&contents)
        .map_err(|err| format!("write error: {}", err))?;

    match fs::remove_file(output_fname) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("error")),
    }
}
