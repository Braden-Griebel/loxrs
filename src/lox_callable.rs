use std::cell::{Ref, RefCell};
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;
use crate::ast::Stmt;
use crate::interpreter::{Interpreter, InterpreterError};
use crate::token::{FunctionEnum, LiteralValue, NativeFunctions, UserDefinedFunction};
use std::time::SystemTime;
use crate::environment::Environment;

pub trait LoxCallable: Debug + PartialEq {
    fn call(&self, interpreter: &mut Interpreter,
            arguments: Vec<LiteralValue>) -> Result<LiteralValue, InterpreterError>;

    fn arity(&self) -> Result<u8, InterpreterError>;
}

impl LoxCallable for NativeFunctions {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<LiteralValue>) -> Result<LiteralValue, InterpreterError> {
        match self {
            NativeFunctions::Clock => {
                let now = SystemTime::now();
                let now_in_ms = now.duration_since(SystemTime::UNIX_EPOCH).expect("Error Getting Time").as_millis();
                Ok(LiteralValue::NumValue(now_in_ms as f64))
            }
        }
    }

    fn arity(&self) -> Result<u8, InterpreterError> {
        match self {
            NativeFunctions::Clock => {Ok(0u8)}
        }
    }
}

impl LoxCallable for UserDefinedFunction {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<LiteralValue>) -> Result<LiteralValue, InterpreterError> {
        let (mut name, mut params, mut body) = match self.declaration.borrow().deref() {
            Stmt::Function {name,  params, body } => {(name.clone(), 
                params.clone(), body.clone())}
            _ => {return Err(InterpreterError{msg: "Invalid function declaration".to_string(),
                returning: false,
                value: None})}
        };

        let call_env = Rc::new(RefCell::new(Environment::new_local(self.closure.clone())));
        
        for i in 0..params.len() {
            call_env.clone().borrow_mut().define(params[i].lexeme.clone(), arguments[i].clone());
        }
        
        match interpreter.execute_block(&mut body, call_env.clone()){
            Ok(val) => {Ok(val)}
            Err(err) => {
                if err.returning {
                    match err.value {
                        None => {Ok(LiteralValue::None)}
                        Some(val) => {Ok(val)}
                    }
                } else {
                    Ok(LiteralValue::None)
                }
            }
        }        
    }

    fn arity(&self) -> Result<u8, InterpreterError> {
        match self.declaration.borrow().deref() {
            Stmt::Function { ref params, .. } => {
                Ok(params.len() as u8)
            }
            _ => {
                Err(InterpreterError {
                    msg: "Invalid Function Definition".to_string(), 
                    returning: false, 
                    value: None,
                })
            }
        }
    }
}


impl LoxCallable for FunctionEnum {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<LiteralValue>) -> Result<LiteralValue, InterpreterError> {
        match self {
            FunctionEnum::Native(fun) => {
                fun.call(interpreter, arguments)
            }
            FunctionEnum::User(fun) => {
                fun.call(interpreter, arguments)
            }
        }
    }

    fn arity(&self) -> Result<u8, InterpreterError> {
        match self {
            FunctionEnum::Native(fun) => {
                fun.arity()
            }
            FunctionEnum::User(fun) => {
                fun.arity()
            }
        }
    }
}