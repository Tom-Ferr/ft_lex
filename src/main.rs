mod lex_spec_parser;

use std::env;
use std::io::{self, Read};
use std::fs::File;
use std::process;

use lex_spec_parser::{LexSpecParser, Section};

fn read_stdin() -> io::Result<String>{
    let mut buffer = String::new();
    let mut line = String::new();
    let stdin = io::stdin();
    let mut parser: LexSpecParser = LexSpecParser::new("<stdin>".to_string());
    while stdin.read_line(&mut line)? > 0{
        buffer.push_str(&line);
        parser.read(&line);
        if line.contains('\n') == false
            {break;}
        line.clear();
    }
    if parser.get_section() == Section::Definitions {
        parser.print_error("premature EOF");
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "",
        ))
    }
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
    if args.is_empty(){
        let buffer = read_stdin();
        match buffer{
            Ok(v) => println!("<stdin>:\n{}", v),
            Err(e) => {if e.kind() != io::ErrorKind::Other {eprintln!("ft_lex: {}", e)}; process::exit(1)}
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
