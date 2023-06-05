use crate::{
    ast::{BlockStmt, Expr, Ident, Infix, Prefix, Program, Stmt},
    environment::Environment,
    object::Object,
};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Evaluator {
    env: Rc<RefCell<Environment>>,
}

impl Evaluator {
    pub fn new(env: Rc<RefCell<Environment>>) -> Self {
        Evaluator { env }
    }

    pub fn eval(&mut self, program: Program) -> Option<Object> {
        let mut result = None;
        for stmt in program {
            match self.eval_stmt(stmt) {
                Some(Object::Return(object)) => return Some(*object),
                object @ Some(Object::Error(_)) => return object,
                object => result = object,
            }
        }
        result
    }

    fn eval_stmts(&mut self, stmts: BlockStmt) -> Option<Object> {
        let mut result = None;
        for stmt in stmts {
            match self.eval_stmt(stmt) {
                object @ (Some(Object::Return(_)) | Some(Object::Error(_))) => return object,
                object => result = object,
            }
        }
        result
    }

    fn eval_stmt(&mut self, stmt: Stmt) -> Option<Object> {
        match stmt {
            Stmt::Expr(expr) => self.eval_expr(expr),
            Stmt::Return(expr) => {
                let object = self.eval_expr(expr)?;
                if is_error(&object) {
                    return Some(object);
                }
                Some(Object::Return(Box::new(object)))
            }
            Stmt::Let(name, expr) => {
                let Ident(name) = name;
                let object = self.eval_expr(expr)?;
                if is_error(&object) {
                    return Some(object);
                }
                self.env.borrow_mut().set(name, &object);
                None
            }
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> Option<Object> {
        match expr {
            Expr::Ident(name) => {
                let Ident(name) = name;
                self.env
                    .borrow_mut()
                    .get(name.clone())
                    .or(Some(Object::Error(format!(
                        "identifier not found: {}",
                        name,
                    ))))
            }
            Expr::Int(value) => Some(Object::Int(value)),
            Expr::Bool(value) => Some(Object::Bool(value)),
            Expr::Prefix(prefix, right) => {
                let right = self.eval_expr(*right)?;
                if is_error(&right) {
                    return Some(right);
                }
                self.eval_prefix_expr(prefix, right)
            }
            Expr::Infix(infix, left, right) => {
                let left = self.eval_expr(*left)?;
                if is_error(&left) {
                    return Some(left);
                }
                let right = self.eval_expr(*right)?;
                if is_error(&right) {
                    return Some(right);
                }
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
                object => Some(Object::Error(format!("unknown operator: -{}", object))),
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
            (Object::Bool(left), Object::Bool(right)) => match infix {
                Infix::Eq => Some(Object::Bool(left == right)),
                Infix::Ne => Some(Object::Bool(left != right)),
                operator => Some(Object::Error(format!(
                    "unknown operator: {} {} {}",
                    Object::Bool(left),
                    operator,
                    Object::Bool(right)
                ))),
            },
            (left, right) => Some(Object::Error(format!(
                "type mismatch: {} {} {}",
                left, infix, right
            ))),
        }
    }

    fn eval_if_expr(
        &mut self,
        cond: Expr,
        cons: BlockStmt,
        alt: Option<BlockStmt>,
    ) -> Option<Object> {
        let cond = self.eval_expr(cond)?;
        if is_error(&cond) {
            return Some(cond);
        }
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

fn is_error(object: &Object) -> bool {
    matches!(object, Object::Error(_))
}
