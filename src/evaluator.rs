use crate::{
    ast::{BlockStmt, Expr, Infix, Prefix, Program, Stmt},
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
            match self.eval_stmt(stmt) {
                Some(Object::Return(object)) => return Some(*object),
                object => result = object,
            }
        }
        result
    }

    fn eval_stmts(&mut self, stmts: BlockStmt) -> Option<Object> {
        let mut result = None;
        for stmt in stmts {
            match self.eval_stmt(stmt) {
                Some(Object::Return(object)) => return Some(Object::Return(object)),
                object => result = object,
            }
        }
        result
    }

    fn eval_stmt(&mut self, stmt: Stmt) -> Option<Object> {
        match stmt {
            Stmt::Expr(expr) => self.eval_expr(expr),
            Stmt::Return(expr) => Some(Object::Return(Box::new(self.eval_expr(expr)?))),
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
            Expr::If(cond, cons, alt) => self.eval_if_expr(*cond, cons, alt),
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

    fn eval_if_expr(
        &mut self,
        cond: Expr,
        cons: BlockStmt,
        alt: Option<BlockStmt>,
    ) -> Option<Object> {
        let cond = self.eval_expr(cond)?;
        if is_truthy(cond) {
            self.eval_stmts(cons)
        } else if alt.is_some() {
            self.eval_stmts(alt.unwrap())
        } else {
            Some(Object::Null)
        }
    }
}

fn is_truthy(object: Object) -> bool {
    !matches!(object, Object::Bool(false) | Object::Null)
}
