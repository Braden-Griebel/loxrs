use std::cell::RefCell;
use std::collections::HashMap;
use crate::token::{LiteralValue, Token};
use std::rc::Rc;

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, LiteralValue>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new_global() -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_local(enclosing: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }
    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<LiteralValue, EnvironmentError> {
        match self.values.get(&name.lexeme) {
            None => {
                match &self.enclosing {
                    None => { Err(EnvironmentError { msg: format!("Couldn't Find Variable: {}", &name.lexeme) }) }
                    Some(env) => {
                        env.borrow().get(&name)
                    }
                }
            }
            Some(val) => { Ok(val.clone()) }
        }
    }

    pub fn assign(&mut self, name: &Token, value: LiteralValue) -> Option<EnvironmentError> {
        match self.values.get_mut(&name.lexeme) {
            None => {
                match &mut self.enclosing {
                    None => {
                        Some(EnvironmentError {
                            msg: format!("Variable {} not yet declared", name.lexeme)
                        })
                    }
                    Some(env) => {
                        env.borrow_mut().assign(name, value)
                    }
                }
            }
            Some(val) => {
                *val = value;
                None
            }
        }
    }
}

pub struct EnvironmentError {
    pub msg: String,
}