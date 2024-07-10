use std::{env, process};
use std::fs::File;
use std::io::{Read, stdout, Write};
use std::path::Path;
use std::io;
use crate::{scanner, token};
use crate::ast::Expr;
use crate::token::{LiteralValue, TokenType};
use crate::parser::Parser;
use crate::printer::AstPrinter;

pub struct Interpreter {
    had_error:bool,
}

impl Interpreter{

    pub fn new()->Interpreter{
        Interpreter{
            had_error:false
        }
    }
    pub fn run(&mut self,  program: String){
        let mut lexer = scanner::Lexer::new(program);
        lexer.scan_tokens();
        
        let mut parser = Parser::new(Vec::from(lexer.tokens));
        let mut expr = match parser.parse(){
            Some(expr)=>expr,
            None => {self.had_error=true; Expr::new_literal(LiteralValue::None)} 
        };
        
        let mut printer = AstPrinter::new();
        let tree = printer.print(&mut expr);
        
        println!("{}",tree)
    }

    pub fn run_file(&mut self, program_path: &str){
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
        if self.had_error {process::exit(65)}
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
    
    pub fn error_token(token:token::Token, message:&str) {
        if token.token_type==TokenType::Eof {
            Interpreter::report(token.line, "at end", message)
        } else {
            Interpreter::report(token.line, &format!(" at '{}'", token.lexeme), message)
        }
    }

    fn report(line: i32, report_where: &str, message: &str){
        println!("[ {0}] Error {1}: {2}", line, report_where, message)
    }
}
