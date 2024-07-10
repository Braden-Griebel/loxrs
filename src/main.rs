use std::{env, process};
use std::io::{Read, Write};

use crate::interpreter::Interpreter;

mod scanner;
mod interpreter;
mod ast;
mod printer;
mod token;
mod parser;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut interpreter = Interpreter::new();
    if args.len() > 2{
        println!("Usage: loxrs [script]");
        process::exit(0);
    } else if args.len()==2 {
        interpreter.run_file(&args[1]);
    } else {
        interpreter.run_prompt();
    }
}

