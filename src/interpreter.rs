use std::cell::RefCell;
use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::rc::Rc;
use crate::ast::{Expr, Stmt, Visitor};
use crate::ast::Expr::Literal;
use crate::token::{LiteralValue, Token, TokenType};
use crate::environment::{Environment, EnvironmentError};
use crate::parser::ParseError;

pub struct Interpreter {
    pub environment: Rc<RefCell<Environment>>,
}

pub struct InterpreterError {
    pub(crate) msg: String,
}

impl Visitor<Result<LiteralValue, InterpreterError>> for Interpreter {
    fn visit_expr(&mut self, expr: &mut Expr) -> Result<LiteralValue, InterpreterError> {
        match expr {
            Expr::Assign { name, value } => {
                let value: LiteralValue = self.evaluate(value)?;
                match self.environment.borrow_mut().assign(name, value.clone()) {
                    None => { Ok(value) }
                    Some(err) => {
                        Err(InterpreterError {
                            msg: err.msg
                        })
                    }
                }
            }
            Expr::Binary { left, operator, right } => {
                let lhs = self.evaluate(left)?;
                let rhs = self.evaluate(right)?;
                match operator.token_type {
                    TokenType::Minus => {
                        match lhs - rhs {
                            Ok(literal_value) => {
                                Ok(literal_value)
                            }
                            Err(_) => {
                                Err(InterpreterError { msg: "Invalid Subtraction".to_string() })
                            }
                        }
                    }
                    TokenType::Plus => {
                        match lhs + rhs {
                            Ok(literal_value) => {
                                Ok(literal_value)
                            }
                            Err(_) => {
                                Err(InterpreterError { msg: "Invalid Addition".to_string() })
                            }
                        }
                    }
                    TokenType::Slash => {
                        match lhs / rhs {
                            Ok(literal_value) => {
                                Ok(literal_value)
                            }
                            Err(_) => {
                                Err(InterpreterError { msg: "Invalid Division".to_string() })
                            }
                        }
                    }
                    TokenType::Star => {
                        match lhs * rhs {
                            Ok(literal_value) => {
                                Ok(literal_value)
                            }
                            Err(_) => {
                                Err(InterpreterError { msg: "".to_string() })
                            }
                        }
                    }
                    TokenType::BangEqual => {
                        match LiteralValue::partial_cmp(&lhs, &rhs) {
                            None => {
                                Err(
                                    InterpreterError {
                                        msg: "Invalid Inequality Comparison".to_string()
                                    }
                                )
                            }
                            Some(cmp) => {
                                match cmp {
                                    Ordering::Less => {
                                        Ok(LiteralValue::True)
                                    }
                                    Ordering::Equal => {
                                        Ok(LiteralValue::False)
                                    }
                                    Ordering::Greater => {
                                        Ok(LiteralValue::True)
                                    }
                                }
                            }
                        }
                    }
                    TokenType::EqualEqual => {
                        match LiteralValue::partial_cmp(&lhs, &rhs) {
                            None => {
                                Err(
                                    InterpreterError {
                                        msg: "Invalid Equality Comparison".to_string()
                                    }
                                )
                            }
                            Some(cmp) => {
                                match cmp {
                                    Ordering::Less => {
                                        Ok(LiteralValue::False)
                                    }
                                    Ordering::Equal => {
                                        Ok(LiteralValue::True)
                                    }
                                    Ordering::Greater => {
                                        Ok(LiteralValue::False)
                                    }
                                }
                            }
                        }
                    }
                    TokenType::Greater => {
                        match LiteralValue::partial_cmp(&lhs, &rhs) {
                            None => {
                                Err(
                                    InterpreterError {
                                        msg: "Invalid Greater Comparison".to_string()
                                    }
                                )
                            }
                            Some(cmp) => {
                                match cmp {
                                    Ordering::Less => {
                                        Ok(LiteralValue::False)
                                    }
                                    Ordering::Equal => {
                                        Ok(LiteralValue::False)
                                    }
                                    Ordering::Greater => {
                                        Ok(LiteralValue::True)
                                    }
                                }
                            }
                        }
                    }
                    TokenType::GreaterEqual => {
                        match LiteralValue::partial_cmp(&lhs, &rhs) {
                            None => {
                                Err(
                                    InterpreterError {
                                        msg: "Invalid Greater/Equal Comparison".to_string()
                                    }
                                )
                            }
                            Some(cmp) => {
                                match cmp {
                                    Ordering::Less => {
                                        Ok(LiteralValue::False)
                                    }
                                    Ordering::Equal => {
                                        Ok(LiteralValue::True)
                                    }
                                    Ordering::Greater => {
                                        Ok(LiteralValue::True)
                                    }
                                }
                            }
                        }
                    }
                    TokenType::Less => {
                        match LiteralValue::partial_cmp(&lhs, &rhs) {
                            None => {
                                Err(
                                    InterpreterError {
                                        msg: "Invalid Less Comparison".to_string()
                                    }
                                )
                            }
                            Some(cmp) => {
                                match cmp {
                                    Ordering::Less => {
                                        Ok(LiteralValue::True)
                                    }
                                    Ordering::Equal => {
                                        Ok(LiteralValue::False)
                                    }
                                    Ordering::Greater => {
                                        Ok(LiteralValue::False)
                                    }
                                }
                            }
                        }
                    }
                    TokenType::LessEqual => {
                        match LiteralValue::partial_cmp(&lhs, &rhs) {
                            None => {
                                Err(
                                    InterpreterError {
                                        msg: "Invalid Less/Equal Comparison".to_string()
                                    }
                                )
                            }
                            Some(cmp) => {
                                match cmp {
                                    Ordering::Less => {
                                        Ok(LiteralValue::True)
                                    }
                                    Ordering::Equal => {
                                        Ok(LiteralValue::True)
                                    }
                                    Ordering::Greater => {
                                        Ok(LiteralValue::False)
                                    }
                                }
                            }
                        }
                    }
                    TokenType::And => {
                        match lhs & rhs {
                            Ok(value) => { Ok(value) }
                            Err(_) => {
                                Err(InterpreterError {
                                    msg: "Invalid AND Operation".to_string()
                                })
                            }
                        }
                    }
                    TokenType::Or => {
                        match lhs | rhs {
                            Ok(value) => { Ok(value) }
                            Err(_) => {
                                Err(InterpreterError {
                                    msg: "Invalid OR Operation".to_string()
                                })
                            }
                        }
                    }
                    _ => {
                        Err(InterpreterError { msg: "Invalid Binary Operator".to_string() })
                    }
                }
            }
            Expr::Call { .. } => { Err(InterpreterError { msg: "Not Implemented Yet".to_string() }) }
            Expr::Get { .. } => { Err(InterpreterError { msg: "Not Implemented Yet".to_string() }) }
            Expr::Grouping { expression } => {
                self.evaluate(expression)
            }
            Expr::Literal { value } => {
                Ok(value.clone())
            }
            Expr::Logical { left, operator, right } => {
                let left: LiteralValue = self.evaluate(left)?;

                if operator.token_type == TokenType::Or {
                    if Interpreter::is_truthy(&left)? {
                        return Ok(left);
                    }
                } else {
                    if !Interpreter::is_truthy(&left)? {
                        return Ok(left);
                    }
                }
                self.evaluate(right)
            }
            Expr::Set { .. } => { Err(InterpreterError { msg: "Not Implemented Yet".to_string() }) }
            Expr::Super { .. } => { Err(InterpreterError { msg: "Not Implemented Yet".to_string() }) }
            Expr::This { .. } => { Err(InterpreterError { msg: "Not Implemented Yet".to_string() }) }
            Expr::Unary { operator, right } => {
                let rhs = self.evaluate(right);
                match rhs {
                    Ok(literal_value) => {
                        match operator.token_type {
                            TokenType::Minus => {
                                match -literal_value {
                                    Ok(new_value) => {
                                        Ok(new_value)
                                    }
                                    Err(_) => {
                                        Err(InterpreterError {
                                            msg: "Invalid Negative Operation".to_string()
                                        })
                                    }
                                }
                            }
                            TokenType::Bang => {
                                match !literal_value {
                                    Ok(new_value) => {
                                        Ok(new_value)
                                    }
                                    Err(_) => {
                                        Err(InterpreterError {
                                            msg: "Invalid Not Operation".to_string()
                                        })
                                    }
                                }
                            }
                            _ => Err(InterpreterError {
                                msg: "Invalid Unary Operator".to_string()
                            })
                        }
                    }
                    Err(err) => {
                        Err(InterpreterError {
                            msg:
                            err.msg
                        })
                    }
                }
            }
            Expr::Variable { name } => {
                match self.environment.borrow().get(name) {
                    Ok(val) => { Ok(val) }
                    Err(_) => {
                        Err(InterpreterError {
                            msg: format!("{} not defined", name.lexeme)
                        })
                    }
                }
            }
        }
    }

    fn visit_stmt(&mut self, stmt: &mut Stmt) -> Result<LiteralValue, InterpreterError> {
        match stmt {
            Stmt::Block { statements } => {
                let _ =self.execute_block(statements, Environment::new_local(self.environment.clone()))?;
                return Ok(LiteralValue::None);
            }
            Stmt::Class { .. } => { Err(InterpreterError { msg: "Not Implemented Yet".to_string() }) }
            Stmt::Expression { expression } => {
                self.evaluate(expression)
            }
            Stmt::Function { .. } => { Err(InterpreterError { msg: "Not Implemented Yet".to_string() }) }
            Stmt::If { condition, then_branch, else_branch } => {
                if Interpreter::is_truthy(&self.evaluate(condition)?)? {
                    self.execute(then_branch)
                } else {
                    match else_branch {
                        None => { Ok(LiteralValue::None) }
                        Some(stmt) => { self.execute(stmt) }
                    }
                }
            }
            Stmt::Print { expression } => {
                let value: LiteralValue = self.evaluate(expression)?;
                println!("{}", value);
                match io::stdout().flush() {
                    Ok(_) => {}
                    Err(_) => { return Err(InterpreterError { msg: "Error Flushing StdOut in Print Statement".to_string() }) }
                };
                return Ok(LiteralValue::None);
            }
            Stmt::Return { .. } => { Err(InterpreterError { msg: "Not Implemented Yet".to_string() }) }
            Stmt::Variable { name, initializer } => {
                match initializer {
                    None => {
                        self.environment.borrow_mut().define(name.lexeme.clone(), LiteralValue::None);
                        Ok(LiteralValue::None)
                    }
                    Some(expr) => {
                        let value = self.evaluate(expr)?;
                        self.environment.borrow_mut().define(name.lexeme.clone(), value);
                        Ok(LiteralValue::None)
                    }
                }
            }
            Stmt::While { condition, body } => {
                while Interpreter::is_truthy(&self.evaluate(condition)?)?{
                    let _ = self.execute(body)?;
                }
                return Ok(LiteralValue::None)
            }
        }
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Rc::new(RefCell::new(Environment::new_global()))
        }
    }

    fn evaluate(&mut self, expr: &mut Box<Expr>) -> Result<LiteralValue, InterpreterError> {
        return self.visit_expr(expr);
    }

    pub fn interpret(&mut self, statements: &mut Vec<Stmt>) -> Result<LiteralValue, InterpreterError> {
        let mut last_val: LiteralValue = LiteralValue::None;
        for statement in statements.iter_mut() {
            last_val = match self.execute(statement) {
                Ok(value) => { value }
                Err(_) => { return Err(InterpreterError { msg: "Error Executing Statemment".to_string() }) }
            };
        }
        return Ok(last_val);
    }

    fn execute(&mut self, stmt: &mut Stmt) -> Result<LiteralValue, InterpreterError> {
        self.visit_stmt(stmt)
    }

    fn execute_block(&mut self, statements: &mut Vec<Box<Stmt>>, environment: Environment) -> Result<LiteralValue, InterpreterError> {
        let previous: Rc<RefCell<Environment>> = self.environment.clone();
        self.environment = Rc::new(RefCell::new(environment));
        for stmt in statements {
            match self.execute(stmt) {
                Ok(_) => {}
                Err(_) => {
                    self.environment = previous;
                    return Err(InterpreterError {
                        msg: "Error executing Block".to_string()
                    });
                }
            };
        }
        self.environment = previous;
        Ok(LiteralValue::None)
    }

    fn is_truthy(value: &LiteralValue) -> Result<bool, InterpreterError> {
        match value {
            LiteralValue::None => { Ok(false) }
            LiteralValue::True => { Ok(true) }
            LiteralValue::False => { Ok(false) }
            LiteralValue::StringValue(str) => {
                if str.len() > 0 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            LiteralValue::NumValue(num) => {
                if num.clone() == 0. {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            LiteralValue::IdentifierValue(_) => {
                Err(InterpreterError {
                    msg: "Tried to evaluate truthiness of identifier value".to_string()
                })
            }
        }
    }
}