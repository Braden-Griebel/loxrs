use std::fmt;
use std::io::{Write};
use std::collections::VecDeque;
use std::fmt::Debug;
use crate::interpreter::Interpreter;
use std::collections::HashMap;


enum LiteralValue {
    None,
    StringValue(String),
    NumValue(f64),
    IdentifierValue(String),
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
        match self {
            LiteralValue::None=>write!(f, ""),
            LiteralValue::StringValue(s)=>write!(f, "{s}"),
            LiteralValue::NumValue(x)=>write!(f, "{x}"),
            LiteralValue::IdentifierValue(s)=>write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,
    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier,
    StringToken,
    Number,
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
        match self {
            TokenType::LeftParen=>write!(f, "("),
            TokenType::RightParen=>write!(f, ")"),
            TokenType::LeftBrace=>write!(f, "{{"),
            TokenType::RightBrace=>write!(f, "}}"),
            TokenType::Comma=>write!(f, ","),
            TokenType::Dot=>write!(f, "."),
            TokenType::Minus=>write!(f, "-"),
            TokenType::Plus=>write!(f, "+"),
            TokenType::SemiColon=>write!(f, ";"),
            TokenType::Slash=>write!(f, "/"),
            TokenType::Star=>write!(f, "*"),
            // One or two character tokens
            TokenType::Bang=>write!(f, "!"),
            TokenType::BangEqual=>write!(f, "!="),
            TokenType::Equal=>write!(f, "="),
            TokenType::EqualEqual=>write!(f, "=="),
            TokenType::Greater=>write!(f, ">"),
            TokenType::GreaterEqual=>write!(f, ">="),
            TokenType::Less=>write!(f, "<"),
            TokenType::LessEqual=>write!(f, "<="),
            // Literals
            TokenType::Identifier=>write!(f, "Identifier"),
            TokenType::StringToken=>write!(f, "String"),
            TokenType::Number=>write!(f, "Number"),
            // Keywords
            TokenType::And=>write!(f, "&"),
            TokenType::Class=>write!(f, "CLASS"),
            TokenType::Else=>write!(f, "ELSE"),
            TokenType::False=>write!(f, "FALSE"),
            TokenType::Fun=>write!(f, "FUN"),
            TokenType::For=>write!(f, "FOR"),
            TokenType::If=>write!(f, "IF"),
            TokenType::Nil=>write!(f, "NIL"),
            TokenType::Or=>write!(f, "OR"),
            TokenType::Print=>write!(f, "PRINT"),
            TokenType::Return=>write!(f, "RETURN"),
            TokenType::Super=>write!(f, "SUPER"),
            TokenType::This=>write!(f, "THIS"),
            TokenType::True=>write!(f, "TRUE"),
            TokenType::Var=>write!(f, "VAR"),
            TokenType::While=>write!(f, "WHILE"),
            TokenType::Eof=>write!(f,"EOF")
        }
    }
}

pub struct Token {
    token_type:TokenType,
    lexeme: String,
    literal: LiteralValue, // Object in jlox
    line:i32,
}

impl Token{
    pub fn new(token_type: TokenType, lexeme: String, literal: LiteralValue, line:i32) ->Token{
        Token{
            token_type,
            lexeme,
            literal,
            line
        }
    }

    pub fn to_string(&self)->String{
        format!("{0} {1} {2}", self.token_type, self.lexeme, self.literal)
    }
}

impl fmt::Display for Token{
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        write!(f,"{0} {1} {2}", self.token_type, self.lexeme, self.literal)
    }
}

pub struct Lexer{
    source: Vec<char>,
    pub tokens: VecDeque<Token>,
    at_end: bool,
    start:usize,
    current:usize,
    line:i32,
    keywords:HashMap<String, TokenType>,
}

impl Lexer{
    pub fn scan_tokens(&mut self)->&VecDeque<Token>{
        while(!self.is_at_end()){
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push_back(Token::new(TokenType::Eof, String::new(), LiteralValue::None, self.line));
        return &self.tokens;
    }

    fn scan_token(&mut self){
        let c:char = self.advance();
        match c {
            // Single Character Lexemes
            '('=>self.add_token(TokenType::LeftParen, LiteralValue::None),
            ')'=>self.add_token(TokenType::RightParen, LiteralValue::None),
            '{'=>self.add_token(TokenType::LeftBrace, LiteralValue::None),
            '}'=>self.add_token(TokenType::RightBrace, LiteralValue::None),
            ','=>self.add_token(TokenType::Comma, LiteralValue::None),
            '.'=>self.add_token(TokenType::Dot, LiteralValue::None),
            '-'=>self.add_token(TokenType::Minus, LiteralValue::None),
            '+'=>self.add_token(TokenType::Plus, LiteralValue::None),
            ';'=>self.add_token(TokenType::SemiColon, LiteralValue::None),
            '*'=>self.add_token(TokenType::Star, LiteralValue::None),
            '!'=>{
                let next_match = self.check_next('=');
                if next_match {
                    self.add_token(TokenType::BangEqual, LiteralValue::None);
                } else {
                    self.add_token(TokenType::Bang, LiteralValue::None);
                }
                },
            '='=>{
                let next_match = self.check_next('=');
                if next_match {
                    self.add_token(TokenType::EqualEqual, LiteralValue::None);
                } else {
                    self.add_token(TokenType::Equal, LiteralValue::None);
                }
            },
            '<'=>{
                let next_match = self.check_next('=');
                if next_match {
                    self.add_token(TokenType::LessEqual, LiteralValue::None);
                } else {
                    self.add_token(TokenType::Less, LiteralValue::None);
                }
            },
            '>'=>{
                let next_match = self.check_next('=');
                if next_match {
                    self.add_token(TokenType::GreaterEqual, LiteralValue::None);
                } else {
                    self.add_token(TokenType::Greater, LiteralValue::None);
                }
            },
            '/'=>{
                if self.check_next('/'){
                    while self.peek() != '\n' && ! self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, LiteralValue::None);
                }
            },
            ' '=>{},
            '\r'=>{},
            '\t'=>{},
            '\n'=>{self.line+=1},
            '"'=>self.read_string(),
            '\0'=>{},
            '0'..='9'=>{self.read_number()},
            'a'..='z'|'A'..='Z'|'_'=>{self.read_identifier()},
            _=>Interpreter::error(self.line, "Unexpected character.")
        }
    }

    fn advance(&mut self)->char{
        let char_at_current = self.source[self.current];
        self.current+=1;
        char_at_current
    }

    fn read_string(&mut self){
        while self.peek()!='"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end(){
            Interpreter::error(self.line, "Unterminated string.");
            return ;
        }
        self.advance();

        // Trim quotes
        let str_value: String = self.source[(self.start+1)..(self.current-1)].iter().collect();
        self.add_token(TokenType::StringToken, LiteralValue::StringValue(str_value))
    }

    fn read_number(&mut self){
        while Lexer::is_digit(self.peek()){
            self.advance();
        }

        if self.peek()=='.' && Lexer::is_digit(self.peek_next()){
            self.advance();
            while Lexer::is_digit(self.peek()){self.advance();}
        }

        let num_str: String = self.source[self.start..self.current].iter().collect();
        let num_float:f64 = num_str.parse().unwrap();

        self.add_token(TokenType::Number, LiteralValue::NumValue(num_float));
    }

    fn read_identifier(&mut self){
        while (Lexer::is_alphanumeric(self.peek())){
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();

        match self.keywords.get(&text) {
            Some(token_type) =>{self.add_token(token_type.clone(), LiteralValue::None)},
            None=>self.add_token(TokenType::Identifier,
                                 LiteralValue::IdentifierValue(text))
        }
    }

    fn peek_next(&self)->char{
        if (self.current+1 >= self.source.len()){
            return '\0';
        }
        self.source[self.current+1]
    }

    fn is_digit(c:char)->bool{
        match c {
            '0'..='9'=>true,
            _=>false
        }
    }

    fn is_alpha(c: char)->bool{
        match c {
            'a'..='z'|'A'..='Z'|'_'=>true,
            _=>false,
        }
    }

    fn is_alphanumeric(c: char)->bool{
        Lexer::is_alpha(c) || Lexer::is_digit(c)
    }

    fn check_next(&mut self, expected:char)->bool{
        if(self.is_at_end()){ return false};
        if(self.source[self.current]!=expected){return false};
        self.current+=1;
        return true;
    }

    fn peek(&self)->char{
        if self.at_end {
            return '\0';
        }
        return self.source[self.current];
    }

    fn add_token(&mut self, token_type: TokenType, literal: LiteralValue){
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push_back(Token::new(token_type, text, literal, self.line));
    }

    fn is_at_end(&self)->bool {
        self.current >= self.source.len()
    }

    pub fn new(source: String)-> Lexer{
        Lexer{
            source:source.chars().collect(),
            tokens: VecDeque::new(),
            at_end:false,
            start:0,
            current:0,
            line:1,
            keywords: HashMap::from([
                (String::from("and"), TokenType::And),
                (String::from("class"), TokenType::Class),
                (String::from("else"), TokenType::Else),
                (String::from("false"), TokenType::False),
                (String::from("for"), TokenType::For),
                (String::from("fun"), TokenType::Fun),
                (String::from("if"), TokenType::If),
                (String::from("nil"), TokenType::Nil),
                (String::from("or"), TokenType::Or),
                (String::from("print"), TokenType::Print),
                (String::from("return"), TokenType::Return),
                (String::from("super"), TokenType::Super),
                (String::from("this"), TokenType::This),
                (String::from("true"), TokenType::True),
                (String::from("Var"), TokenType::Var),
                (String::from("While"), TokenType::While),
            ])
        }
    }
}