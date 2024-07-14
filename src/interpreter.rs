use std::cell::{Ref, RefCell};
use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::ops::Deref;
use std::rc::Rc;

use crate::ast::{Expr, Stmt, Visitor};
use crate::environment::Environment;
use crate::lox_callable::LoxCallable;
use crate::token::{FunctionEnum, LiteralValue, TokenType, UserDefinedFunction};
use crate::token::NativeFunctions::Clock;

pub struct Interpreter {
    pub environment: Rc<RefCell<Environment>>,
    pub globals: Rc<RefCell<Environment>>,
}

pub struct InterpreterError {
    pub(crate) msg: String,
    pub(crate) returning: bool,
    pub(crate) value: Option<LiteralValue>,
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
                            msg: err.msg,
                            returning: false,
                            value: None,
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
                                Err(InterpreterError {
                                    msg: "Invalid Subtraction".to_string(),
                                    returning: false,
                                    value: None,
                                })
                            }
                        }
                    }
                    TokenType::Plus => {
                        match lhs + rhs {
                            Ok(literal_value) => {
                                Ok(literal_value)
                            }
                            Err(_) => {
                                Err(InterpreterError {
                                    msg: "Invalid Addition".to_string(),
                                    returning: false,
                                    value: None,
                                })
                            }
                        }
                    }
                    TokenType::Slash => {
                        match lhs / rhs {
                            Ok(literal_value) => {
                                Ok(literal_value)
                            }
                            Err(_) => {
                                Err(InterpreterError {
                                    msg: "Invalid Division".to_string(),
                                    returning: false,
                                    value: None,
                                })
                            }
                        }
                    }
                    TokenType::Star => {
                        match lhs * rhs {
                            Ok(literal_value) => {
                                Ok(literal_value)
                            }
                            Err(_) => {
                                Err(InterpreterError {
                                    msg: "".to_string(),
                                    returning: false,
                                    value: None,
                                })
                            }
                        }
                    }
                    TokenType::BangEqual => {
                        match LiteralValue::partial_cmp(&lhs, &rhs) {
                            None => {
                                Err(
                                    InterpreterError {
                                        msg: "Invalid Inequality Comparison".to_string(),
                                        returning: false,
                                        value: None,
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
                                        msg: "Invalid Equality Comparison".to_string(),
                                        returning: false,
                                        value: None,
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
                                        msg: "Invalid Greater Comparison".to_string(),
                                        returning: false,
                                        value: None,
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
                                        msg: "Invalid Greater/Equal Comparison".to_string(),
                                        returning: false,
                                        value: None,
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
                                        msg: "Invalid Less Comparison".to_string(),
                                        returning: false,
                                        value: None,
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
                                        msg: "Invalid Less/Equal Comparison".to_string(),
                                        returning: false,
                                        value: None,
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
                                    msg: "Invalid AND Operation".to_string(),
                                    returning: false,
                                    value: None,
                                })
                            }
                        }
                    }
                    TokenType::Or => {
                        match lhs | rhs {
                            Ok(value) => { Ok(value) }
                            Err(_) => {
                                Err(InterpreterError {
                                    msg: "Invalid OR Operation".to_string(),
                                    returning: false,
                                    value: None,
                                })
                            }
                        }
                    }
                    _ => {
                        Err(InterpreterError {
                            msg: "Invalid Binary Operator".to_string(),
                            returning: false,
                            value: None,
                        })
                    }
                }
            }
            Expr::Call { callee, paren, arguments } => {
                let callee: LiteralValue = self.evaluate(callee)?;

                let mut args: Vec<LiteralValue> = Vec::new();
                for argument in arguments {
                    args.push(self.evaluate(argument)?);
                }

                let function = match callee {
                    LiteralValue::Function(fun) => {
                        if args.len() as u8 != fun.arity()? {
                            return Err(InterpreterError {
                                msg: "Incorrect number of arguments".to_string(),
                                returning: false,
                                value: None,
                            });
                        }
                        fun.call(self, args)
                    }
                    _ => Err(InterpreterError {
                        msg: "Tried to call non-callable".to_string(),
                        returning: false,
                        value: None,
                    })
                };

                function
            }
            Expr::Get { .. } => {
                Err(InterpreterError {
                    msg: "Not Implemented Yet".to_string(),
                    returning: false,
                    value: None,
                })
            }
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
            Expr::Set { .. } => {
                Err(InterpreterError {
                    msg: "Not Implemented Yet".to_string(),
                    returning: false,
                    value: None,
                })
            }
            Expr::Super { .. } => {
                Err(InterpreterError {
                    msg: "Not Implemented Yet".to_string(),
                    returning: false,
                    value: None,
                })
            }
            Expr::This { .. } => {
                Err(InterpreterError {
                    msg: "Not Implemented Yet".to_string(),
                    returning: false,
                    value: None,
                })
            }
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
                                            msg: "Invalid Negative Operation".to_string(),
                                            returning: false,
                                            value: None,
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
                                            msg: "Invalid Not Operation".to_string(),
                                            returning: false,
                                            value: None,
                                        })
                                    }
                                }
                            }
                            _ => Err(InterpreterError {
                                msg: "Invalid Unary Operator".to_string(),
                                returning: false,
                                value: None,
                            })
                        }
                    }
                    Err(err) => {
                        Err(InterpreterError {
                            msg:
                            err.msg,
                            returning: false,
                            value: None,
                        })
                    }
                }
            }
            Expr::Variable { name } => {
                match self.environment.borrow().get(name) {
                    Ok(val) => { Ok(val) }
                    Err(_) => {
                        Err(InterpreterError {
                            msg: format!("{} not defined", name.lexeme),
                            returning: false,
                            value: None,
                        })
                    }
                }
            }
        }
    }

    fn visit_stmt(&mut self, stmt: &mut Stmt) -> Result<LiteralValue, InterpreterError> {
        match stmt {
            Stmt::Block { statements } => {
                let _ = self.execute_block(statements, Rc::new(RefCell::new(Environment::new_local(self.environment.clone()))))?;
                return Ok(LiteralValue::None);
            }
            Stmt::Class { .. } => {
                Err(InterpreterError {
                    msg: "Not Implemented Yet".to_string(),
                    returning: false,
                    value: None,
                })
            }
            Stmt::Expression { expression } => {
                self.evaluate(expression)
            }
            Stmt::Function { name, .. } => {
                let func_name: String = name.lexeme.clone();
                let new_fun = LiteralValue::Function(FunctionEnum::User(UserDefinedFunction {
                    closure: Rc::new(RefCell::new(Environment::new_local(self.environment.clone()))),
                    declaration: Rc::new(RefCell::new(stmt.clone())), //This is really hacky, and might desync state
                }));
                self.environment.borrow_mut().define(func_name, new_fun);
                Ok(LiteralValue::None)
            }
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
                    Err(_) => {
                        return Err(InterpreterError {
                            msg: "Error Flushing StdOut in Print Statement".to_string(),
                            returning: false,
                            value: None,
                        })
                    }
                };
                return Ok(LiteralValue::None);
            }
            Stmt::Return { keyword, value } => {
                let new_value = match value {
                    None => { None }
                    Some(val) => { Some(self.evaluate(val)?) }
                };

                return Err(
                    InterpreterError {
                        msg: "".to_string(),
                        returning: true,
                        value: new_value,
                    }
                );
            }
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
                while Interpreter::is_truthy(&self.evaluate(condition)?)? {
                    let _ = self.execute(body)?;
                }
                return Ok(LiteralValue::None);
            }
        }
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let globals = Rc::new(RefCell::new(Environment::new_global()));

        globals.borrow_mut().define("clock".to_string(),
                                    LiteralValue::Function(FunctionEnum::Native(Clock)));

        let environment = globals.clone();
        Interpreter {
            environment,
            globals,
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
                Err(_) => {
                    return Err(InterpreterError {
                        msg: "Error Executing Statemment".to_string(),
                        returning: false,
                        value: None,
                    })
                }
            };
        }
        return Ok(last_val);
    }

    fn execute(&mut self, stmt: &mut Stmt) -> Result<LiteralValue, InterpreterError> {
        self.visit_stmt(stmt)
    }

    pub(crate) fn execute_block(&mut self, statements: &mut Vec<Box<Stmt>>, environment: Rc<RefCell<Environment>>) -> Result<LiteralValue, InterpreterError> {
        let previous: Rc<RefCell<Environment>> = self.environment.clone();
        self.environment = environment;
        let mut last_val: LiteralValue;
        for stmt in statements {
            last_val = match self.execute(stmt) {
                Ok(value) => { value }
                Err(err) => {
                    if err.returning {
                        self.environment = previous;
                        return Err(err);
                    }
                    self.environment = previous;
                    return Err(InterpreterError {
                        msg: "Error executing Block".to_string(),
                        returning: false,
                        value: None,
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
                    msg: "Tried to evaluate truthiness of identifier value".to_string(),
                    returning: false,
                    value: None,
                })
            }
            LiteralValue::Function(_) => {
                Err(InterpreterError {
                    msg: "Tried to evaluate truthiness of callable".to_string(),
                    returning: false,
                    value: None,
                })
            }
        }
    }

    fn unwrap_interpreter_result(result: Result<LiteralValue, InterpreterError>) -> Result<LiteralValue, InterpreterError> {
        match result {
            Ok(val) => { Ok(val) }
            Err(error) => {
                if error.returning {
                    return match error.value.clone() {
                        None => { Ok(LiteralValue::None) }
                        Some(v) => { Ok(v) }
                    }
                }
                Err(error)
            }
        }
    }
}