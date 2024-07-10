use std::cmp::Ordering;
use crate::ast::{Expr, Stmt, Visitor};
use crate::token::{LiteralValue, Token, TokenType};

pub struct Interpreter {}

pub struct InterpreterError {
    pub(crate) msg: String,
}

impl Visitor<Result<LiteralValue, InterpreterError>> for Interpreter {
    fn visit_expr(&mut self, expr: &mut Expr) -> Result<LiteralValue, InterpreterError> {
        match expr {
            Expr::Assign { .. } => { Err(InterpreterError { msg: "Not Implemented Yet".to_string() }) }
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
                            Ok(value) => {Ok(value)}
                            Err(_) => {
                                Err(InterpreterError{
                                    msg: "Invalid AND Operation".to_string()
                                })
                            }
                        }
                    }
                    TokenType::Or => {
                        match lhs | rhs {
                            Ok(value) => {Ok(value)}
                            Err(_) => {
                                Err(InterpreterError{
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
            Expr::Logical { .. } => { Err(InterpreterError { msg: "Not Implemented Yet".to_string() }) }
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
            Expr::Variable { .. } => { Err(InterpreterError { msg: "Not Implemented Yet".to_string() }) }
        }
    }

    fn visit_stmt(&mut self, stmt: &mut Stmt) -> Result<LiteralValue, InterpreterError> {
        todo!()
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    fn evaluate(&mut self, expr: &mut Box<Expr>) -> Result<LiteralValue, InterpreterError> {
        return self.visit_expr(expr);
    }
    
    pub fn interpret(&mut self, expression: Expr)->Result<LiteralValue, InterpreterError>{
        self.evaluate(&mut Box::new(expression))
    }
}