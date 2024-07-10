use std::{env, process};
use std::io::{Read, Write};

use crate::lox::Lox;

mod scanner;
mod lox;
mod ast;
mod printer;
mod token;
mod parser;
mod interpreter;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut interpreter = Lox::new();
    if args.len() > 2{
        println!("Usage: loxrs [script]");
        process::exit(0);
    } else if args.len()==2 {
        interpreter.run_file(&args[1]);
    } else {
        interpreter.run_prompt();
    }
}

