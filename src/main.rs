use std::{env, process};
use std::fs::File;
use std::io::{Read, stdout, Write};
use std::path::Path;
use std::io;

mod scanner;

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() > 2{
        println!("Usage: loxrs [script]");
        process::exit(0);
    } else if args.len()==2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run(program: String){
    for char in program.chars(){
        print!("{}",char);
        stdout().flush().unwrap();
    }
}

fn run_file(program_path: &str){
    let file_path = Path::new(program_path);

    // open the file
    let mut file = match File::open(&file_path){
        Err(why)=> panic!("Couldn't read {}: {}", program_path, why),
        Ok(file)=> file,
    };

    // Read the file contents into string
    let mut contents = String::new();
    match file.read_to_string(&mut contents){
        Err(why)=> panic!("Couldn't read {}:{}", program_path, why),
        Ok(_) => {}
    };

    run(contents);
}

fn run_prompt(){
    loop{
        print!(">");
        stdout().flush().unwrap();

        // Read user input
        let mut input = String::new();
        let read_result = io::stdin().read_line(&mut input);

        // Check for EOF
        match read_result {
            Err(_)=>println!("Couldn't readline"),
            Ok(0)=>break,
            Ok(_)=>{}
        };

        run(input);
    }
}