use std::env;
use std::io::{self, Read};
use std::fs::File;
use std::process;

fn read_stdin() -> io::Result<String>{
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn read_file(target: &String) -> io::Result<String> {
    let mut f = File::open(target)?;
    let mut data = String::new();
    f.read_to_string(&mut data)?;
    Ok(data)
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        let buffer = read_stdin();
        match buffer{
            Ok(v) => println!("stdin:\n{}", v),
            Err(e) => {eprintln!("Error: {}", e); process::exit(1)}
        }
    }
    else{
        let buffer = read_file(&args[1]);
        match buffer{
            Ok(v) => println!("file:\n{}", v),
            Err(e) => {eprintln!("Error: {}", e); process::exit(1)}
        }
    }
}
