use crate::token::{LiteralValue, Token};

#[derive(Clone, Debug)]
pub enum Expr {
    Assign{
        name: Token,
        value: Box<Expr>
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Box<Expr>>,
    },
    Get {
        object: Box<Expr>,
        name: Token,
    },
    Grouping {
        expression: Box<Expr>
    },
    Literal {
        value: LiteralValue
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Set {
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>
    },
    Super {
        keyword: Token,
        method: Token,
    },
    This {
        keyword: Token,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
}


impl Expr {
    pub fn new_assign(name: Token, value:Expr)->Expr{
        Expr::Assign {
            name,
            value: Box::new(value),
        }
    }
    pub fn new_binary(left:Expr, operator:Token, right:Expr)->Expr{
        Expr::Binary {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
    pub fn new_call(callee:Expr, paren:Token, arguments: Vec<Expr>)->Expr{
        Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments: arguments.into_iter().map(|x| Box::new(x)).collect()
        }
    }
    pub fn new_get(object:Expr, name:Token)->Expr{
        Expr::Get {
            object: Box::new(object),
            name
        }
    }
    pub fn new_grouing(expression:Expr)->Expr{
        Expr::Grouping {
            expression: Box::new(expression)
        }
    }
    pub fn new_literal(value:LiteralValue)->Expr{
        Expr::Literal {
            value
        }
    }
    pub fn new_logical(left:Expr, operator:Token, right:Expr)->Expr{
        Expr::Logical {
            left:Box::new(left),
            operator,
            right: Box::new(right)
        }
    }
    pub fn new_set(object:Expr, name:Token, value: Expr)->Expr{
        Expr::Set {
            object:Box::new(object),
            name,
            value: Box::new(value)
        }
    }
    pub fn new_super(keyword:Token, method:Token)->Expr{
        Expr::Super {
            keyword,
            method
        }
    }
    pub fn new_this(keyword:Token)->Expr{
        Expr::This {
            keyword
        }
    }
    pub fn new_unary(operator:Token, right:Expr)->Expr{
        Expr::Unary {
            operator,
            right:Box::new(right)
        }
    }

    pub fn new_variable(name:Token)->Expr{
        Expr::Variable {
            name
        }
    }
}

pub enum Stmt {
    Block {
        statements: Vec<Box<Stmt>>
    },
    Class {
        name: Token,
        superclass: Box<Expr>,
        methods: Vec<Box<Stmt>>,
    },
    Expression {
        expression: Box<Expr>,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Box<Stmt>>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Box<Stmt>,
    },
    Print {
        expression: Box<Expr>,
    },
    Return {
        keyword: Token,
        value: Box<Expr>,
    },
    Variable {
        name: Token,
        initializer: Box<Expr>,
    },
    While {
        condition: Box<Expr>,
        body: Box<Stmt>,
    },
}

impl Stmt {
    pub fn new_block(statements: Vec<Stmt>)->Stmt{
        Stmt::Block {
            statements: statements.into_iter().map(|x| Box::new(x)).collect()
        }
    }
    pub fn new_class(name:Token, superclass:Expr, methods:Vec<Stmt>)->Stmt{
        Stmt::Class {
            name,
            superclass: Box::new(superclass),
            methods:methods.into_iter().map(|x| Box::new(x)).collect()
        }
    }
    pub fn new_expression(expression:Expr)->Stmt{
        Stmt::Expression {
            expression: Box::new(expression)
        }
    }
    pub fn new_function(name:Token, params:Vec<Token>, body: Vec<Stmt>)->Stmt{
        Stmt::Function {
            name,
            params,
            body: body.into_iter().map(|x| Box::new(x)).collect()
        }
    }
    pub fn new_if(condition:Expr, then_branch:Stmt, else_branch:Stmt)->Stmt{
        Stmt::If {
            condition:Box::new(condition),
            then_branch:Box::new(then_branch),
            else_branch:Box::new(else_branch)
        }
    }
    pub fn new_print(expression:Expr)->Stmt{
        Stmt::Print {
            expression:Box::new(expression)
        }
    }
    pub fn new_return(keyword:Token, value:Expr)->Stmt{
        Stmt::Return {
            keyword,
            value:Box::new(value)
        }
    }
    pub fn new_variable(name:Token, initializer: Expr)->Stmt{
        Stmt::Variable {
            name,
            initializer: Box::new(initializer)
        }
    }
    pub fn new_while(condition:Expr, body:Stmt)->Stmt{
        Stmt::While {
            condition:Box::new(condition),
            body: Box::new(body)
        }
    }
}

pub trait Visitor<T> {
    fn visit_expr(&mut self, expr: &mut Expr)->T;
    fn visit_stmt(&mut self, stmt: &mut Stmt)->T;
}