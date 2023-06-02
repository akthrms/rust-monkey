use crate::{
    ast::{Expr, Program, Stmt},
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
            _ => unimplemented!(),
        }
    }
}
