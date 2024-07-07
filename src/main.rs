use std::{env, process};
use std::fs::File;
use std::io::{Read, stdout, Write};
use std::path::Path;
use std::io;
use crate::interpreter::Interpreter;

mod scanner;
mod interpreter;
mod ast;

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

struct Node {
    data:i32,
    left: NodeType,
    right: NodeType
}

enum NodeType{
    Leaf,
    Interior(Box<Node>),
    None,
}

impl Node{
    pub fn new_zero_child(data:i32)->Node{
        Node{
            data,
            left: NodeType::None,
            right: NodeType::None,
        }
    }

    pub fn new_one_child(data:i32, child: Node)->Node{
        Node{
            data,
            left:NodeType::Interior(Box::new(child)),
            right:NodeType::None
        }
    }

    pub fn new_two_child(data:i32, left: Node, right: Node)-> Node{
        Node{
            data,
            left:NodeType::Interior(Box::new(left)),
            right:NodeType::Interior(Box::new(right))
        }
    }
}
