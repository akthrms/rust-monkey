use crate::{
    ast::{BlockStmt, Ident},
    environment::Environment,
};
use std::{cell::RefCell, fmt::Display, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    Int(i64),
    Bool(bool),
    String(String),
    Null,
    Return(Box<Object>),
    Error(String),
    Function(Vec<Ident>, BlockStmt, Rc<RefCell<Environment>>),
}

impl Object {
    pub fn get_type(&self) -> String {
        match self {
            Object::Int(_) => "INT".to_string(),
            Object::Bool(_) => "BOOL".to_string(),
            Object::String(_) => "STRING".to_string(),
            Object::Null => "NULL".to_string(),
            Object::Return(_) => "RETURN".to_string(),
            Object::Error(_) => "ERROR".to_string(),
            Object::Function(_, _, _) => "FUNCTION".to_string(),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Int(value) => write!(f, "{}", value),
            Object::Bool(value) => write!(f, "{}", value),
            Object::String(value) => write!(f, "{}", value),
            Object::Null => write!(f, "null"),
            Object::Return(value) => write!(f, "{}", value),
            Object::Error(value) => write!(f, "{}", value),
            Object::Function(_, _, _) => write!(f, "function"),
        }
    }
}
