//use std::env;
//use std::fs;
use std::io::{self, BufRead, Write};

use sputter::{eval, parse};

fn main() -> io::Result<()> {
    //let args: Vec<String> = env::args().collect();
    //let default_fp = String::from(r"D:\Rust\sputter\sputters\cool.sputter");
    //let fp = args.get(1).unwrap_or(&default_fp);
    //let code_line = fs::read_to_string(fp)?;
    let stdin = io::stdin();
    loop {
        let mut code_line = String::new();
        print!("> ");
        io::stdout().flush()?;
        stdin.lock().read_line(&mut code_line)?;
        if code_line.trim() == "quit" {
            break;
        }
        let program = match parse(&code_line) {
            Ok(x) => x,
            Err(e) => {
                println!("Could not parse: {:?}", e);
                continue;
            }
        };
        println!("Parsed: {:?}", program);
        let result = eval(program);
        println!("Evaluated Result: {:?}", result);
    }
    Ok(())
}
