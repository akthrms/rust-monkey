use crate::{
    ast::{Expr, Infix, Prefix, Program, Stmt},
    object::Object,
};

#[derive(Debug)]
pub struct Evaluator {}

impl Evaluator {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Evaluator {}
    }

    pub fn eval(&mut self, program: Program) -> Option<Object> {
        let mut result = None;
        for stmt in program {
            result = self.eval_stmt(stmt);
        }
        result
    }

    fn eval_stmt(&mut self, stmt: Stmt) -> Option<Object> {
        match stmt {
            Stmt::Expr(expr) => self.eval_expr(expr),
            _ => unimplemented!(),
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Option<Object> {
        match expr {
            Expr::Int(value) => Some(Object::Int(value)),
            Expr::Bool(value) => Some(Object::Bool(value)),
            Expr::Prefix(prefix, right) => {
                let right = self.eval_expr(*right)?;
                self.eval_prefix_expr(prefix, right)
            }
            Expr::Infix(infix, left, right) => {
                let left = self.eval_expr(*left)?;
                let right = self.eval_expr(*right)?;
                self.eval_infix_expr(infix, left, right)
            }
            _ => unimplemented!(),
        }
    }

    fn eval_prefix_expr(&mut self, prefix: Prefix, right: Object) -> Option<Object> {
        match prefix {
            Prefix::Bang => match right {
                Object::Bool(value) => Some(Object::Bool(!value)),
                Object::Null => Some(Object::Bool(true)),
                _ => Some(Object::Bool(false)),
            },
            Prefix::Minus => match right {
                Object::Int(value) => Some(Object::Int(-value)),
                _ => Some(Object::Null),
            },
        }
    }

    fn eval_infix_expr(&mut self, infix: Infix, left: Object, right: Object) -> Option<Object> {
        match (left, right) {
            (Object::Int(left), Object::Int(right)) => match infix {
                Infix::Plus => Some(Object::Int(left + right)),
                Infix::Minus => Some(Object::Int(left - right)),
                Infix::Asterisk => Some(Object::Int(left * right)),
                Infix::Slash => Some(Object::Int(left / right)),
                Infix::Lt => Some(Object::Bool(left < right)),
                Infix::Gt => Some(Object::Bool(left > right)),
                Infix::Eq => Some(Object::Bool(left == right)),
                Infix::Ne => Some(Object::Bool(left != right)),
            },
            (left, right) => match infix {
                Infix::Eq => Some(Object::Bool(left == right)),
                Infix::Ne => Some(Object::Bool(left != right)),
                _ => Some(Object::Null),
            },
        }
    }
}
