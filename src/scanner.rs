use std::collections::VecDeque;
use crate::lox::Lox;
use std::collections::HashMap;
use crate::token::{LiteralValue, Token, TokenType};

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
        while !self.is_at_end() {
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
            _=> Lox::error(self.line, "Unexpected character.")
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
            Lox::error(self.line, "Unterminated string.");
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
        while Lexer::is_alphanumeric(self.peek()) {
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
        if self.current+1 >= self.source.len() {
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
        if self.is_at_end() { return false};
        if self.source[self.current]!=expected {return false};
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
                (String::from("var"), TokenType::Var),
                (String::from("while"), TokenType::While),
            ])
        }
    }
}