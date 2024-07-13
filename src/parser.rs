use crate::ast::{Expr, Stmt};
use crate::ast::Expr::Literal;
use crate::interpreter::InterpreterError;
use crate::token::{LiteralValue, Token, TokenType};
use crate::lox::Lox;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
        }
    }
    
    pub fn parse(&mut self)->Result<Vec<Stmt>, ParseError> {
        let mut statements:Vec<Stmt> = vec![]; 
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }
    
    fn declaration(&mut self)-> Result<Stmt, ParseError>{
        if self.match_token(vec![TokenType::Var]) {
            match  self.var_declaration(){
                Ok(stmt) => {Ok(stmt)}
                Err(e) => {
                    self.synchronize();
                    return Err(e)
                }
            }
        } else {
            return self.statement();
        }
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        return Ok(self.assignment()?);
    }
    
    fn statement(&mut self) -> Result<Stmt, ParseError>{
        if self.match_token(vec![TokenType::Print]) {return self.print_statement();}
        if self.match_token(vec![TokenType::LeftBrace]) {
            return Ok(Stmt::new_block(self.block()?))
        }
        
        return self.expression_statement();
    }
    
    fn assignment(&mut self)-> Result<Expr, ParseError>{
        let expr: Expr = self.equality()?;
        
        if self.match_token(vec![TokenType::Equal]){
            let equals: Token = self.previous();
            let value:Expr = self.assignment()?;
            
            match expr {
                Expr::Variable { name } => {
                    return Ok(Expr::new_assign(name, value));
                }
                _=>{return Err(ParseError{ token: equals, message: "Invalid assignment target.".to_string() })}
            }
        }
        Ok(expr)
    }
    
    fn var_declaration(&mut self)->Result<Stmt, ParseError>{
        let name:Token = self.consume(TokenType::Identifier, "Expect variable name.");
        
        let mut initializer: Expr;
        
        if self.match_token(vec![TokenType::Equal]){
            initializer = self.expression()?;
            self.consume(TokenType::SemiColon, "Expect ';' after variable declaration.");

            return Ok(Stmt::new_variable_initialized(name, initializer));
        }
        
        self.consume(TokenType::SemiColon, "Expect ';' after variable declaration.");
        Ok(Stmt::new_variable_uninitialized(name))
    }
    
    fn print_statement(&mut self)->Result<Stmt, ParseError>{
        let value: Expr = self.expression()?;
        self.consume(TokenType::SemiColon, "Expect ';' after value.");
        return Ok(Stmt::new_print(value))
    }
    
    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr: Expr =self.expression()?;
        self.consume(TokenType::SemiColon, "Expect ';' after expression.");
        Ok(Stmt::new_expression(expr))
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.comparison()?;

        while self.match_token(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = self.previous();
            let right: Expr = self.comparison()?;
            expr = Expr::new_binary(expr, operator, right); // Expr may need to be cloned
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.term()?; // Get left hand side

        while self.match_token(vec![TokenType::Greater, TokenType::GreaterEqual,
                                    TokenType::Less, TokenType::LessEqual]) {
            let operator: Token = self.previous();
            let right: Expr = self.term()?;
            expr = Expr::new_binary(expr, operator, right);
        }
        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.factor()?;

        while self.match_token(vec![TokenType::Minus, TokenType::Plus]) {
            let operator: Token = self.previous();
            let right: Expr = self.factor()?;
            expr = Expr::new_binary(expr, operator, right);
        }
        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr: Expr = self.unary()?;

        while self.match_token(vec![TokenType::Slash, TokenType::Star]) {
            let operator: Token = self.previous();
            let right: Expr = self.unary()?;
            expr = Expr::new_binary(expr, operator, right);
        }
        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(vec![TokenType::Bang, TokenType::Minus]) {
            let operator: Token = self.previous();
            let right: Expr = self.unary()?;
            return Ok(Expr::new_unary(operator, right));
        }
        return Ok(self.primary()?);
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(vec![TokenType::False]) { return Ok(Expr::new_literal(LiteralValue::False)); }
        if self.match_token(vec![TokenType::True]) { return Ok(Expr::new_literal(LiteralValue::True)); }
        if self.match_token(vec![TokenType::Nil]) { return Ok(Expr::new_literal(LiteralValue::None)); }

        if self.match_token(vec![TokenType::Number, TokenType::StringToken]) {
            return Ok(Expr::new_literal(self.previous().literal));
        }
        
        if self.match_token(vec![TokenType::Identifier]) {
            return Ok(Expr::new_variable(self.previous()));
        }

        if self.match_token(vec![TokenType::LeftParen]) {
            let expr: Expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Ok(Expr::new_grouing(expr));
        }

        Err(Parser::error(self.peek(), "Expect expression."))
    }
    
    fn block(&mut self) -> Result<Vec<Stmt>, ParseError>{
        let mut statements: Vec<Stmt> = Vec::new();
        
        while (!self.check(TokenType::RightBrace) && !self.is_at_end()){
            statements.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.");
        return Ok(statements)
    }

    fn match_token(&mut self, types: Vec<TokenType>) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() { return false; };
        return self.peek().token_type == token_type;
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() { self.current += 1 };
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token_type == TokenType::Eof;
    }

    fn peek(&self) -> Token {
        return self.tokens[self.current].clone();
    }

    fn previous(&self) -> Token {
        return self.tokens[self.current - 1].clone();
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        if self.check(token_type) { return self.advance(); }

        panic!("{}: {}", self.peek(), message)
    }

    fn error(token: Token, message: &str) -> ParseError {
        Lox::error_token(token.clone(), message);
        return ParseError {token, message: message.to_string()};
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SemiColon {
                return;
            }
            match self.peek().token_type {
                TokenType::Class => { return }
                TokenType::Fun => { return }
                TokenType::Var => { return }
                TokenType::For => { return }
                TokenType::If => { return }
                TokenType::While => { return }
                TokenType::Print => { return }
                TokenType::Return => { return }
                _ => {}
            }
            self.advance();
        }
    }
}

pub struct ParseError {
    token:Token, 
    message:String,
}