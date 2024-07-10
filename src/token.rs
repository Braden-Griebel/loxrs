use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;
use std::ops::{Add, BitAnd, BitOr, Div, Mul, Neg, Not, Sub};

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    None,
    True,
    False,
    StringValue(String),
    NumValue(f64),
    IdentifierValue(String),
}

pub struct TokenError; // Error type for token errors, currently empty but can be extended later

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LiteralValue::None => write!(f, "nil"),
            LiteralValue::True => write!(f, "true"),
            LiteralValue::False => write!(f, "false"),
            LiteralValue::StringValue(s) => write!(f, "{s}"),
            LiteralValue::NumValue(x) => write!(f, "{x}"),
            LiteralValue::IdentifierValue(s) => write!(f, "{s}"),
        }
    }
}

impl Neg for LiteralValue {
    type Output = Result<LiteralValue, TokenError>;

    fn neg(self) -> Self::Output {
        match self {
            LiteralValue::None => { Err(TokenError) }
            LiteralValue::True => { Err(TokenError) }
            LiteralValue::False => { Err(TokenError) }
            LiteralValue::StringValue(_) => { Err(TokenError) }
            LiteralValue::NumValue(n) => { Ok(LiteralValue::NumValue(n)) }
            LiteralValue::IdentifierValue(_) => { Err(TokenError) }
        }
    }
}

impl Not for LiteralValue {
    type Output = Result<LiteralValue, TokenError>;

    fn not(self) -> Self::Output {
        match self {
            LiteralValue::None => { Ok(LiteralValue::True) }
            LiteralValue::True => { Ok(LiteralValue::False) }
            LiteralValue::False => { Ok(LiteralValue::True) }
            LiteralValue::StringValue(s) => {
                if s.len() > 0 {
                    Ok(LiteralValue::True)
                } else {
                    Ok(LiteralValue::False)
                }
            }
            LiteralValue::NumValue(n) => {
                if n == 0f64 {
                    Ok(LiteralValue::False)
                } else {
                    Ok(LiteralValue::True)
                }
            }
            LiteralValue::IdentifierValue(_)=>{Err(TokenError)}
        }
    }
}

impl Add for LiteralValue {
    type Output = Result<LiteralValue, TokenError>;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            LiteralValue::None => {Err(TokenError)}
            LiteralValue::True => {LiteralValue::NumValue(1f64)+rhs}
            LiteralValue::False => {LiteralValue::NumValue(0f64)+rhs}
            LiteralValue::StringValue(lhs_str) => {
                match rhs {
                    LiteralValue::None => {Err(TokenError)}
                    LiteralValue::True => {Err(TokenError)}
                    LiteralValue::False => {Err(TokenError)}
                    LiteralValue::StringValue(rhs_str) => {
                        let mut res_str = lhs_str.clone();
                        res_str.push_str(&rhs_str);
                        Ok(LiteralValue::StringValue(res_str))
                    }
                    LiteralValue::NumValue(_) => {Err(TokenError)}
                    LiteralValue::IdentifierValue(_) => {Err(TokenError)}
                }
            }
            LiteralValue::NumValue(lhs_num) => {
                match rhs {
                    LiteralValue::None => {Err(TokenError)}
                    LiteralValue::True => {Err(TokenError)}
                    LiteralValue::False => {Err(TokenError)}
                    LiteralValue::StringValue(_) => {Err(TokenError)}
                    LiteralValue::NumValue(rhs_num) => {
                        Ok(LiteralValue::NumValue(lhs_num+rhs_num))
                    }
                    LiteralValue::IdentifierValue(_) => {Err(TokenError)}
                }
            }
            LiteralValue::IdentifierValue(_) => {Err(TokenError)}
        }

    }
}

impl Sub for LiteralValue {
    type Output = Result<LiteralValue, TokenError>;
    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            LiteralValue::None => {Err(TokenError)}
            LiteralValue::True => {LiteralValue::NumValue(1f64)-rhs}
            LiteralValue::False => {LiteralValue::NumValue(0f64)-rhs}
            LiteralValue::StringValue(_) => {Err(TokenError)}
            LiteralValue::NumValue(lhs_num) => {
                match rhs {
                    LiteralValue::None => {Err(TokenError)}
                    LiteralValue::True => {self - LiteralValue::NumValue(1f64)}
                    LiteralValue::False => {self-LiteralValue::NumValue(0f64)}
                    LiteralValue::StringValue(_) => {Err(TokenError)}
                    LiteralValue::NumValue(rhs_num) => {
                        Ok(LiteralValue::NumValue(lhs_num-rhs_num))
                    }
                    LiteralValue::IdentifierValue(_) => {Err(TokenError)}
                }
            }
            LiteralValue::IdentifierValue(_) => {Err(TokenError)}
        }
    }
}

impl Mul for LiteralValue {
    type Output = Result<LiteralValue, TokenError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            LiteralValue::None => {Err(TokenError)}
            LiteralValue::True => {LiteralValue::NumValue(1f64)*rhs}
            LiteralValue::False => {LiteralValue::NumValue(0f64)*rhs}
            LiteralValue::StringValue(lhs_str) => {
                match rhs {
                    LiteralValue::None => {Err(TokenError)}
                    LiteralValue::True => {LiteralValue::StringValue(lhs_str)*LiteralValue::NumValue(1f64)}
                    LiteralValue::False => {LiteralValue::StringValue(lhs_str)*LiteralValue::NumValue(0f64)}
                    LiteralValue::StringValue(_) => {Err(TokenError)}
                    LiteralValue::NumValue(rhs_num) => {
                        Ok(LiteralValue::StringValue(lhs_str.repeat(rhs_num as usize)))
                    }
                    LiteralValue::IdentifierValue(_) => {Err(TokenError)}
                }
            }
            LiteralValue::NumValue(lhs_num) => {
                match rhs {
                    LiteralValue::None => {Err(TokenError)}
                    LiteralValue::True => {self*LiteralValue::NumValue(1f64)}
                    LiteralValue::False => {self*LiteralValue::NumValue(0f64)}
                    LiteralValue::StringValue(_) => {Err(TokenError)}
                    LiteralValue::NumValue(rhs_num) => {
                        Ok(LiteralValue::NumValue(lhs_num*rhs_num))
                    }
                    LiteralValue::IdentifierValue(_) => {Err(TokenError)}
                }
            }
            LiteralValue::IdentifierValue(_) => {Err(TokenError)}
        }
    }
}

impl Div for LiteralValue {
    type Output = Result<LiteralValue, TokenError>;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            LiteralValue::None => {Err(TokenError)}
            LiteralValue::True => {LiteralValue::NumValue(1f64) / rhs}
            LiteralValue::False => {LiteralValue::NumValue(0f64)/ rhs}
            LiteralValue::StringValue(_) => {Err(TokenError)}
            LiteralValue::NumValue(lhs_num) => {
                match rhs {
                    LiteralValue::None => {Err(TokenError)}
                    LiteralValue::True => {self / LiteralValue::NumValue(1f64)}
                    LiteralValue::False => {self / LiteralValue::NumValue(0f64)}
                    LiteralValue::StringValue(_) => {Err(TokenError)}
                    LiteralValue::NumValue(rhs_num) => {
                        Ok(LiteralValue::NumValue(lhs_num / rhs_num))
                    }
                    LiteralValue::IdentifierValue(_) => {Err(TokenError)}
                }
            }
            LiteralValue::IdentifierValue(_) => {Err(TokenError)}
        }
    }
}

impl BitAnd for LiteralValue {
    type Output = Result<LiteralValue, TokenError>;

    fn bitand(self, rhs: Self) -> Self::Output {
        match self {
            LiteralValue::None => {Err(TokenError)}
            LiteralValue::True => {
                match rhs {
                    LiteralValue::None => {Err(TokenError)}
                    LiteralValue::True => {
                        Ok(LiteralValue::True)
                    }
                    LiteralValue::False => {
                        Ok(LiteralValue::False)
                    }
                    LiteralValue::StringValue(s) => {
                        if s.len() == 0 {
                            self & LiteralValue::False
                        } else {
                            self & LiteralValue::True
                        }
                    }
                    LiteralValue::NumValue(num) => {
                        if num == 0. {
                            self & LiteralValue::False
                        } else {
                            self & LiteralValue::True
                        }
                    }
                    LiteralValue::IdentifierValue(_) => {Err(TokenError)}
                }
            }
            LiteralValue::False => {
                match rhs {
                    LiteralValue::None => {Err(TokenError)}
                    LiteralValue::True => {
                        Ok(LiteralValue::False)
                    }
                    LiteralValue::False => {
                        Ok(LiteralValue::False)
                    }
                    LiteralValue::StringValue(s) => {
                        if s.len() == 0 {
                            self & LiteralValue::False
                        } else {
                            self & LiteralValue::True
                        }
                    }
                    LiteralValue::NumValue(num) => {
                        if num == 0. {
                            self & LiteralValue::False
                        } else {
                            self & LiteralValue::True
                        }
                    }
                    LiteralValue::IdentifierValue(_) => {Err(TokenError)}
                }
            }
            LiteralValue::StringValue(s) => {
                if s.len() == 0usize {
                    LiteralValue::False & rhs
                } else {
                    LiteralValue::True & rhs
                }
            }
            LiteralValue::NumValue(num) => {
                if num == 0. {
                    LiteralValue::False & rhs
                } else {
                    LiteralValue::True & rhs
                }
            }
            LiteralValue::IdentifierValue(_) => {Err(TokenError)}
        }
    }
}

impl BitOr for LiteralValue {
    type Output = Result<LiteralValue, TokenError>;

    fn bitor(self, rhs: Self) -> Self::Output {
        match self {
            LiteralValue::None => {Err(TokenError)}
            LiteralValue::True => {
                match rhs {
                    LiteralValue::None => {Err(TokenError)}
                    LiteralValue::True => {Ok(LiteralValue::True)}
                    LiteralValue::False => {Ok(LiteralValue::True)}
                    LiteralValue::StringValue(s) => {
                        if s.len() == 0usize{
                            self | LiteralValue::False
                        } else {
                            self | LiteralValue::True
                        }
                    }
                    LiteralValue::NumValue(num) => {
                        if num == 0.{
                            self | LiteralValue::False
                        } else {
                            self | LiteralValue::True
                        }
                    }
                    LiteralValue::IdentifierValue(_) => {Err(TokenError)}
                }
            }
            LiteralValue::False => {
                match rhs {
                    LiteralValue::None => {Err(TokenError)}
                    LiteralValue::True => {Ok(LiteralValue::True)}
                    LiteralValue::False => {Ok(LiteralValue::False)}
                    LiteralValue::StringValue(s) => {
                        if s.len() == 0usize{
                            self | LiteralValue::False
                        } else {
                            self | LiteralValue::True
                        }
                    }
                    LiteralValue::NumValue(num) => {
                        if num == 0.{
                            self | LiteralValue::False
                        } else {
                            self | LiteralValue::True
                        }
                    }
                    LiteralValue::IdentifierValue(_) => {Err(TokenError)}
                }
            }
            LiteralValue::StringValue(s) => {
                if s.len() > 0usize {
                    LiteralValue::False | rhs
                } else {
                    LiteralValue::True | rhs
                }
            }
            LiteralValue::NumValue(num) => {
                if num == 0.  {
                    LiteralValue::False | rhs
                } else {
                    LiteralValue::True | rhs
                }
            }
            LiteralValue::IdentifierValue(_) => {Err(TokenError)}
        }
    }
}


impl PartialOrd for LiteralValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            LiteralValue::None => {None}
            LiteralValue::True => {LiteralValue::partial_cmp(&LiteralValue::NumValue(1f64), other)}
            LiteralValue::False => {LiteralValue::partial_cmp(&LiteralValue::NumValue(0f64), other)}
            LiteralValue::StringValue(_) => {None}
            LiteralValue::NumValue(lhs_num) => {
                match other {
                    LiteralValue::None => {None}
                    LiteralValue::True => {LiteralValue::partial_cmp(&self, &LiteralValue::NumValue(1f64))}
                    LiteralValue::False => {LiteralValue::partial_cmp(&self, &LiteralValue::NumValue(0f64))}
                    LiteralValue::StringValue(_) => {None}
                    LiteralValue::NumValue(rhs_num) => {
                        if lhs_num == rhs_num {
                            Some(Ordering::Equal)
                        } else if lhs_num < rhs_num {
                            Some(Ordering::Less)
                        } else if lhs_num> rhs_num {
                            Some(Ordering::Greater)
                        } else {
                            None
                        }
                    }
                    LiteralValue::IdentifierValue(_) => {None}
                }
            }
            LiteralValue::IdentifierValue(_) => {None}
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::Comma => write!(f, ","),
            TokenType::Dot => write!(f, "."),
            TokenType::Minus => write!(f, "-"),
            TokenType::Plus => write!(f, "+"),
            TokenType::SemiColon => write!(f, ";"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Star => write!(f, "*"),
            // One or two character tokens
            TokenType::Bang => write!(f, "!"),
            TokenType::BangEqual => write!(f, "!="),
            TokenType::Equal => write!(f, "="),
            TokenType::EqualEqual => write!(f, "=="),
            TokenType::Greater => write!(f, ">"),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::Less => write!(f, "<"),
            TokenType::LessEqual => write!(f, "<="),
            // Literals
            TokenType::Identifier => write!(f, "Identifier"),
            TokenType::StringToken => write!(f, "String"),
            TokenType::Number => write!(f, "Number"),
            // Keywords
            TokenType::And => write!(f, "&"),
            TokenType::Class => write!(f, "CLASS"),
            TokenType::Else => write!(f, "ELSE"),
            TokenType::False => write!(f, "FALSE"),
            TokenType::Fun => write!(f, "FUN"),
            TokenType::For => write!(f, "FOR"),
            TokenType::If => write!(f, "IF"),
            TokenType::Nil => write!(f, "NIL"),
            TokenType::Or => write!(f, "OR"),
            TokenType::Print => write!(f, "PRINT"),
            TokenType::Return => write!(f, "RETURN"),
            TokenType::Super => write!(f, "SUPER"),
            TokenType::This => write!(f, "THIS"),
            TokenType::True => write!(f, "TRUE"),
            TokenType::Var => write!(f, "VAR"),
            TokenType::While => write!(f, "WHILE"),
            TokenType::Eof => write!(f, "EOF")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: LiteralValue, // Object in jlox
    pub(crate) line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: LiteralValue, line: i32) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{0} {1} {2}", self.token_type, self.lexeme, self.literal)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0} {1} {2}", self.token_type, self.lexeme, self.literal)
    }
}