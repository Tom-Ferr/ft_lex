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
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1{
        let buffer = read_stdin();
        match buffer{
            Ok(v) => println!("<stdin>:\n{}", v),
            Err(e) => {eprintln!("ft_lex: {}", e); process::exit(1)}
        }
    }
    else{
        let mut result = String::new();
        for file_name in args{
            let buffer;
            match file_name.as_str(){
                "-" => buffer = read_stdin(),
                _ => buffer = read_file(&file_name)
            }
            match buffer{
                Ok(v) => result.push_str(&v),
                Err(e) => {eprintln!("ft_lex: {}", e); process::exit(1)}
            }
        }
        println!("{}", result);
    }
}
