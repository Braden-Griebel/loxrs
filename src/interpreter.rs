use std::{env, process};
use std::fs::File;
use std::io::{Read, stdout, Write};
use std::path::Path;
use std::io;
use crate::{scanner};
pub struct Interpreter {
    had_error:bool,
}

impl Interpreter{

    pub fn new()->Interpreter{
        Interpreter{
            had_error:false
        }
    }
    pub fn run(&self, program: String){
        let mut lexer = scanner::Lexer::new(program);
        lexer.scan_tokens();
        for token in lexer.tokens.iter(){
            println!("{}", token)
        }
    }

    pub fn run_file(&self, program_path: &str){
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

        self.run(contents);

        // Indicate an error in th exit code.
        if (self.had_error) {process::exit(65)}
    }

    pub fn run_prompt(&mut self){
        loop{
            print!(">");
            stdout().flush().unwrap();

            // Read user input
            let mut input = String::new();
            let read_result = io::stdin().read_line(&mut input);
            //input += &String::from('\n');

            // Check for EOF
            match read_result {
                Err(_)=>println!("Couldn't readline"),
                Ok(0)=>break,
                Ok(_)=>{}
            };

            self.run(input);
            self.had_error = false;
        }
    }

    pub fn error(line:i32, message:&str){
        Interpreter::report(line, "", message)
    }

    fn report(line: i32, report_where: &str, message: &str){
        println!("[ {0}] Error {1}: {2}", line, report_where, message)
    }
}
