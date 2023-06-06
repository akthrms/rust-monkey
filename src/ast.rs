use crate::token::Token;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Ident(Ident),
    Int(i64),
    Bool(bool),
    Prefix(Prefix, Box<Expr>),
    Infix(Infix, Box<Expr>, Box<Expr>),
    If(Box<Expr>, BlockStmt, Option<BlockStmt>),
    Function(Vec<Ident>, BlockStmt),
    Call(Box<Expr>, Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Let(Ident, Expr),
    Return(Expr),
    Expr(Expr),
}

pub type BlockStmt = Vec<Stmt>;

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
            Token::LPAREN => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}
