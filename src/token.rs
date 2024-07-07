use std::fmt;

#[derive(Debug, Clone)]
pub enum LiteralValue {
    None,
    StringValue(String),
    NumValue(f64),
    IdentifierValue(String),
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
        match self {
            LiteralValue::None=>write!(f, "nil"),
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

#[derive(Debug, Clone)]
pub struct Token {
    token_type:TokenType,
    pub(crate) lexeme: String,
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