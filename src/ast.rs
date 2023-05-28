use crate::token::Token;
use std::fmt::Display;

#[derive(Debug)]
pub struct Ident(pub String);

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub enum Prefix {
    Minus,
    Bang,
}

impl Display for Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Prefix::Minus => write!(f, "-"),
            Prefix::Bang => write!(f, "!"),
        }
    }
}

#[derive(Debug)]
pub enum Infix {
    Plus,
    Minus,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    Ne,
}

impl Display for Infix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Infix::Plus => write!(f, "+"),
            Infix::Minus => write!(f, "-"),
            Infix::Asterisk => write!(f, "*"),
            Infix::Slash => write!(f, "/"),
            Infix::Lt => write!(f, "<"),
            Infix::Gt => write!(f, ">"),
            Infix::Eq => write!(f, "=="),
            Infix::Ne => write!(f, "!="),
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Ident(Ident),
    Int(i64),
    Bool(bool),
    Prefix(Prefix, Box<Expr>),
    Infix(Infix, Box<Expr>, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Ident(ident) => write!(f, "{}", ident),
            Expr::Int(value) => write!(f, "{}", value),
            Expr::Bool(value) => write!(f, "{}", value),
            Expr::Prefix(prefix, expr) => write!(f, "{}{}", prefix, expr),
            Expr::Infix(infix, left, right) => write!(f, "({} {} {})", left, infix, right),
        }
    }
}

#[derive(Debug)]
pub enum Stmt {
    Let(Ident, Expr),
    Return(Expr),
    Expr(Expr),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Let(ident, expr) => write!(f, "let {} = {};", ident, expr),
            Stmt::Return(expr) => write!(f, "return {};", expr),
            Stmt::Expr(expr) => write!(f, "{}", expr),
        }
    }
}

pub type Program = Vec<Stmt>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl From<&Token> for Precedence {
    fn from(token: &Token) -> Self {
        match token {
            Token::EQ | Token::NE => Precedence::Equals,
            Token::LT | Token::GT => Precedence::LessGreater,
            Token::PLUS | Token::MINUS => Precedence::Sum,
            Token::ASTERISK | Token::SLASH => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }
}
