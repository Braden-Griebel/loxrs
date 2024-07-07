use std::{cell::RefCell, rc::Rc};
use std::fmt::Arguments;
use crate::scanner::Token;

enum LiteralValue {
    None,
    StringValue(String),
    NumValue(f64),
    IdentifierValue(String),
}


// region Expression
pub enum ExprKind {
    Assign(Assign),
    Binary(Binary),
    Call(Call),
    Get(Get),
    Grouping(Grouping),
    Literal(Literal),
    Logical(Logical),
    Set(Set),
    Super(Super),
    This(This),
    Unary(Unary),
    Variable(Variable),
}

pub type ExprNode = Option<Rc<RefCell<ExprKind>>> ;

pub trait ExprVisitor<T> {
    fn visit_assign_expr(&mut self, expr:&mut Assign)->T;
    fn visit_binary_expr(&mut self, expr:&mut Binary)->T;
    fn visit_call_expr(&mut self, expr:&mut Call)->T;
    fn visit_get_expr(&mut self, expr:&mut Get)->T;
    fn visit_grouping_expr(&mut self, expr:&mut Grouping)->T;
    fn visit_literal_expr(&mut self, expr:&mut Literal)->T;
    fn visit_logical_expr(&mut self, expr:&mut Logical)->T;
    fn visit_set_expr(&mut self, expr:&mut Set)->T;
    fn visit_super_expr(&mut self, expr:&mut Super)->T;
    fn visit_this_expr(&mut self, expr:&mut This)->T;
    fn visit_unary_expr(&mut self, expr:&mut Unary)->T;
    fn visit_variable_expr(&mut self, expr:&mut Variable)->T;

}

pub trait Expr {
    fn accept<T>(&mut self, visitor:&mut impl ExprVisitor<T>) ->T;
}


// TODO: Implement a macro for defining Expressions
// Which would take in the name, and the properties, and generate the definition
// region Assign Expression
struct Assign {
    name: Token,
    value: ExprNode
}

impl Expr for Assign {
    fn accept<T>(&mut self, visitor: &mut impl ExprVisitor<T>) -> T {
        return visitor.visit_assign_expr(self);
    }
}

impl Assign {
    fn new(name:Token, value:ExprNode)->Assign{
        Assign{
            name,
            value
        }
    }
}
// endregion Assign Expression

// region Binary Expression
struct Binary {
    left: ExprNode,
    operator: Token,
    right: ExprNode
}

impl Expr for Binary{
    fn accept<T>(&mut self, visitor: &mut impl ExprVisitor<T>) -> T {
        visitor.visit_binary_expr(self)
    }
}

impl Binary {
    fn new(left:ExprNode, operator:Token, right:ExprNode)->Binary {
        Binary {
            left,
            operator,
            right,
        }
    }
}
// endregion Binary Expression

// region Call Expression
struct Call {
    callee: ExprNode,
    paren: Token,
    arguments: Vec<ExprNode>
}

impl Expr for Call {
    fn accept<T>(&mut self, visitor: &mut impl ExprVisitor<T>) -> T {
        visitor.visit_call_expr(self)
    }
}

impl Call {
    fn new(callee:ExprNode, paren:Token, arguments: Vec<ExprNode>)->Call{
        Call {
            callee,
            paren,
            arguments,
        }
    }
}
// endregion Call Expression

// region Get Expression
struct Get {
    object: ExprNode,
    name:Token
}

impl Expr for Get {
    fn accept<T>(&mut self, visitor: &mut impl ExprVisitor<T>) -> T {
        visitor.visit_get_expr(self)
    }
}

impl Get {
    fn new(object: ExprNode, name:Token)->Get{
        Get{
            object,
            name,
        }
    }
}
// endregion Get Expression

// region Grouping Expression
struct Grouping {
    expression: ExprNode
}

impl Expr for Grouping {
    fn accept<T>(&mut self, visitor: &mut impl ExprVisitor<T>) -> T {
        visitor.visit_grouping_expr(self)
    }
}

impl Grouping {
    fn new(expression: ExprNode)->Grouping{
        Grouping{
            expression
        }
    }
}
// endregion Grouping Expression

// region Literal Expression
struct Literal {
    value: LiteralValue
}

impl Expr for Literal {
    fn accept<T>(&mut self, visitor: &mut impl ExprVisitor<T>) -> T {
        visitor.visit_literal_expr(self)
    }
}

impl Literal {
    fn new(value:LiteralValue)->Literal{
        Literal{
            value
        }
    }
}
// endregion Literal Expression

// region Logical Expression
struct Logical {
    left: ExprNode,
    operator: Token,
    right: ExprNode,
}

impl Expr for Logical {
    fn accept<T>(&mut self, visitor: &mut impl ExprVisitor<T>) -> T {
        visitor.visit_logical_expr(self)
    }
}

impl Logical {
    fn new(left:ExprNode, operator:Token, right:ExprNode)->Logical{
        Logical{
            left,
            operator,
            right,
        }
    }
}
// endregion Logical Expression

// region Set Expression
struct Set {
    object: ExprNode,
    name: Token,
    value: ExprNode,
}

impl Expr for Set {
    fn accept<T>(&mut self, visitor: &mut impl ExprVisitor<T>) -> T {
        visitor.visit_set_expr(self)
    }
}

impl Set {
    fn new(object:ExprNode, name:Token, value:ExprNode)->Set{
        Set{
            object,
            name,
            value,
        }
    }
}
// endregion Set Expression

// region Super Expression
struct Super {
    keyword: Token,
    method:Token,
}

impl Expr for Super {
    fn accept<T>(&mut self, visitor: &mut impl ExprVisitor<T>) -> T {
        visitor.visit_super_expr(self)
    }
}

impl Super{
    fn new(keyword:Token, method:Token)->Super{
        Super{
            keyword,
            method,
        }
    }
}
// endregion Super Expression

// region This Expression
struct This {
    keyword: Token
}

impl Expr for This {
    fn accept<T>(&mut self, visitor: &mut impl ExprVisitor<T>) -> T {
        visitor.visit_this_expr(self)
    }
}

impl This {
    fn new(keyword:Token)->This{
        This{
            keyword
        }
    }
}
// endregion This Expression

// region Unary Expression
struct Unary {
    operator:Token,
    right: ExprNode,
}

impl Expr for Unary {
    fn accept<T>(&mut self, visitor: &mut impl ExprVisitor<T>) -> T {
        visitor.visit_unary_expr(self)
    }
}

impl Unary {
    fn new(operator:Token, right:ExprNode)->Unary{
        Unary{
            operator,
            right
        }
    }
}

// endregion Unary Expression

// region Variable Expression
struct Variable {
    name:Token
}

impl Expr for Variable {
    fn accept<T>(&mut self, visitor: &mut impl ExprVisitor<T>) -> T {
        visitor.visit_variable_expr(self)
    }
}

impl Variable {
    fn new(name:Token)->Variable{
        Variable {
            name
        }
    }
}
// endregion Variable Expression

// endregion Expression

// region Statement
pub enum StmtKind{
    Block(Block),
    Class(Class),
    Expression(Expression),
    Function(Function),
    If(If),
    Print(Print),
    Return(Return),
    Var(Var),
    While(While),
}

pub type StmtNode = Option<Rc<RefCell<StmtKind>>>;

pub trait StmtVisitor<T> {
    fn visit_block_stmt(&mut self, stmt: &mut Block)->T;
    fn visit_class_stmt(&mut self, stmt: &mut Class)->T;
    fn visit_expression_stmt(&mut self, stmt: &mut Expression)->T;
    fn visit_function_stmt(&mut self, stmt: &mut Function)->T;
    fn visit_if_stmt(&mut self, stmt: &mut If)->T;
    fn visit_print_stmt(&mut self, stmt: &mut Print)->T;
    fn visit_return_stmt(&mut self, stmt: &mut Return)->T;
    fn visit_var_stmt(&mut self, stmt: &mut Var)->T;
    fn visit_while_stmt(&mut self, stmt: &mut While)->T;
}

pub trait Stmt {
    fn accept<T>(&mut self, visitor:&mut impl StmtVisitor<T>) ->T;
}

// region Block Statement
struct Block{
    statements: Vec<StmtNode>
}

impl Stmt for Block {
    fn accept<T>(&mut self, visitor: &mut impl StmtVisitor<T>) -> T {
        visitor.visit_block_stmt(self)
    }
}

impl Block {
    fn new(statements: Vec<StmtNode>)->Block{
        Block{
            statements
        }
    }
}
// endregion Block Statement

// region Class Statement
struct Class {
    name: Token,
    superclass: Variable,
    methods: Vec<Function>
}

impl Stmt for Class {
    fn accept<T>(&mut self, visitor: &mut impl StmtVisitor<T>) -> T {
        visitor.visit_class_stmt(self)
    }
}

impl Class {
    fn new(name:Token, superclass: Variable, methods: Vec<Function>)->Class{
        Class{
            name,
            superclass,
            methods,
        }
    }
}

// endregion Class Statement

// region Expression Statement
struct Expression {
    expression: ExprKind,
}

impl Stmt for Expression {
    fn accept<T>(&mut self, visitor: &mut impl StmtVisitor<T>) -> T {
        visitor.visit_expression_stmt(self)
    }
}

impl Expression {
    fn new(expression: ExprKind)->Expression{
        Expression {
            expression
        }
    }
}

// endregion Expression Statement

// region Function Statement
struct Function {
    name: Token,
    params: Vec<Token>,
    body: Vec<StmtKind>
}

impl Stmt for Function {
    fn accept<T>(&mut self, visitor: &mut impl StmtVisitor<T>) -> T {
        visitor.visit_function_stmt(self)
    }
}

impl Function{
    fn new(name:Token, params: Vec<Token>, body: Vec<StmtKind>)->Function{
        Function{
            name,
            params,
            body,
        }
    }
}
// endregion Function Statement

// region If Statement

struct If {
    condition: ExprKind,
    then_branch: StmtNode,
    else_branch: StmtNode,
}

impl Stmt for If{
    fn accept<T>(&mut self, visitor: &mut impl StmtVisitor<T>) -> T {
        visitor.visit_if_stmt(self)
    }
}

impl If{
    fn new(condition:ExprKind, then_branch:StmtNode, else_branch:StmtNode)->If{
        If{
            condition, then_branch, else_branch
        }
    }
}

// endregion If Statement

// region Print Statement

struct Print{
    expression: ExprKind
}

impl Stmt for Print {
    fn accept<T>(&mut self, visitor: &mut impl StmtVisitor<T>) -> T {
        visitor.visit_print_stmt(self)
    }
}

impl Print {
    fn new(expression: ExprKind)->Print{
        Print{
            expression
        }
    }
}

// endregion Print Statement

// region Return Statement
struct Return {
    keyword:Token,
    value: ExprKind,
}

impl Stmt for Return{
    fn accept<T>(&mut self, visitor: &mut impl StmtVisitor<T>) -> T {
        visitor.visit_return_stmt(self)
    }
}

impl Return {
    fn new(keyword:Token, value:ExprKind)->Return{
        Return{
            keyword, value
        }
    }
}
// endregion Return Statement

// region Variable Statement
struct Var {
    name:Token,
    initializer: ExprKind
}

impl Stmt for Var{
    fn accept<T>(&mut self, visitor: &mut impl StmtVisitor<T>) -> T {
        visitor.visit_var_stmt(self)
    }
}

impl Var{
    fn new(name:Token, initializer:ExprKind)->Var{
        Var{
            name, initializer
        }
    }
}

// endregion Variable Statement

// region While Statement

struct While{
    condition: ExprNode,
    body: StmtNode
}

impl Stmt for While {
    fn accept<T>(&mut self, visitor: &mut impl StmtVisitor<T>) -> T {
        visitor.visit_while_stmt(self)
    }
}

impl While{
    fn new(condition: ExprNode, body:StmtNode)->While{
        While{
            condition, body
        }
    }
}
// endregion While Statement

// endregion Statement
