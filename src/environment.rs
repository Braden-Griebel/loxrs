use std::cell::RefCell;
use std::collections::HashMap;
use crate::token::{LiteralValue, Token};
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub struct Environment {
    pub(crate) values: HashMap<String, LiteralValue>,
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
    
    pub fn full_clone(environment: &Rc<RefCell<Environment>>, 
                  cur_env_copy: Option<Environment>)->Environment {
        // Create a new environment if needed (for recursion)
        let mut new_env = match cur_env_copy {
            None => {Environment::new_global()}
            Some(env) => {env}
        };
        
        // Get the values from the passed in reference environment
        let environment_values = environment.borrow().values.clone();
        
        for (key, value) in &environment_values {
            new_env.define(key.clone(), value.clone())
        }
        
        
        // If there is an enclosing environment, copy it into this 
        // function as well
         match &environment.borrow().enclosing {
            None => {new_env}
            Some(env) => {
                Environment::full_clone(env, Some(new_env))
            }
        }
    }
}

pub struct EnvironmentError {
    pub msg: String,
}