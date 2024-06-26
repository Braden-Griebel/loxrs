use std::{env, process};
use std::fs::File;
use std::io::{Read, stdout, Write};
use std::path::Path;
use std::io;
use crate::interpreter::Interpreter;

mod scanner;
mod interpreter;

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
