use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum Object {
    Int(i64),
    Bool(bool),
    Null,
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Int(value) => write!(f, "{}", value),
            Object::Bool(value) => write!(f, "{}", value),
            Object::Null => write!(f, "null"),
        }
    }
}
