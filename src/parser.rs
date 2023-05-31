use crate::{
    ast::{BlockStmt, Expr, Ident, Infix, Precedence, Prefix, Program, Stmt},
    lexer::Lexer,
    token::Token,
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            cur_token: Token::EOF,
            peek_token: Token::EOF,
            errors: Vec::new(),
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn is_cur_token(&self, token: &Token) -> bool {
        self.cur_token == *token
    }

    fn is_peek_token(&self, token: &Token) -> bool {
        self.peek_token == *token
    }

    fn expect_peek(&mut self, token: Token) -> bool {
        if self.is_peek_token(&token) {
            self.next_token();
            true
        } else {
            self.peek_error(token);
            false
        }
    }

    fn peek_error(&mut self, token: Token) {
        self.errors.push(format!(
            "expected next token to be {}, got {} instead",
            token, self.peek_token
        ));
    }

    fn no_prefix_error(&mut self) {
        self.errors.push(format!(
            "no prefix parse function for {} found",
            self.cur_token
        ));
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Vec::new();
        while !self.is_cur_token(&Token::EOF) {
            if let Some(stmt) = self.parse_stmt() {
                program.push(stmt);
            }
            self.next_token();
        }
        program
    }

    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.cur_token {
            Token::LET => self.parse_let_stmt(),
            Token::RETURN => self.parse_return_stmt(),
            _ => self.parse_expr_stmt(),
        }
    }

    fn parse_let_stmt(&mut self) -> Option<Stmt> {
        let ident = match self.peek_token {
            Token::IDENT(_) => {
                self.next_token();
                self.parse_ident()?
            }
            _ => return None,
        };
        if !self.expect_peek(Token::ASSIGN) {
            return None;
        }
        self.next_token();
        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            _ => return None,
        };
        while self.is_peek_token(&Token::SEMICOLON) {
            self.next_token();
        }
        Some(Stmt::Let(ident, expr))
    }

    fn parse_return_stmt(&mut self) -> Option<Stmt> {
        self.next_token();
        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            _ => return None,
        };
        while self.is_peek_token(&Token::SEMICOLON) {
            self.next_token();
        }
        Some(Stmt::Return(expr))
    }

    fn parse_expr_stmt(&mut self) -> Option<Stmt> {
        match self.parse_expr(Precedence::Lowest) {
            Some(expr) => {
                while self.is_peek_token(&Token::SEMICOLON) {
                    self.next_token();
                }
                Some(Stmt::Expr(expr))
            }
            _ => None,
        }
    }

    fn parse_block_stmt(&mut self) -> BlockStmt {
        self.next_token();
        let mut stmts = Vec::new();
        while !self.is_cur_token(&Token::RBRACE) && !self.is_cur_token(&Token::EOF) {
            if let Some(stmt) = self.parse_stmt() {
                stmts.push(stmt);
            }
            self.next_token();
        }
        stmts
    }

    fn parse_expr(&mut self, precedence: Precedence) -> Option<Expr> {
        let mut left = match self.cur_token {
            Token::IDENT(_) => self.parse_ident_expr()?,
            Token::INT(_) => self.parse_int_expr()?,
            Token::BOOL(_) => self.parse_bool_expr()?,
            Token::MINUS | Token::BANG => self.parse_prefix_expr()?,
            Token::LPAREN => self.parse_grouped_expr()?,
            Token::IF => self.parse_if_expr()?,
            _ => {
                self.no_prefix_error();
                return None;
            }
        };
        while !self.expect_peek(Token::SEMICOLON) && precedence < (&self.peek_token).into() {
            match self.peek_token {
                Token::PLUS
                | Token::MINUS
                | Token::ASTERISK
                | Token::SLASH
                | Token::LT
                | Token::GT
                | Token::EQ
                | Token::NE => {
                    self.next_token();
                    left = self.parse_infix_expr(left)?;
                }
                _ => break,
            };
        }
        Some(left)
    }

    fn parse_prefix_expr(&mut self) -> Option<Expr> {
        let prefix = match self.cur_token {
            Token::MINUS => Prefix::Minus,
            Token::BANG => Prefix::Bang,
            _ => unreachable!(),
        };
        self.next_token();
        self.parse_expr(Precedence::Prefix)
            .map(|expr| Expr::Prefix(prefix, Box::new(expr)))
    }

    fn parse_infix_expr(&mut self, left: Expr) -> Option<Expr> {
        let infix = match self.cur_token {
            Token::PLUS => Infix::Plus,
            Token::MINUS => Infix::Minus,
            Token::ASTERISK => Infix::Asterisk,
            Token::SLASH => Infix::Slash,
            Token::LT => Infix::Lt,
            Token::GT => Infix::Gt,
            Token::EQ => Infix::Eq,
            Token::NE => Infix::Ne,
            _ => unreachable!(),
        };
        let precedence = (&self.cur_token).into();
        self.next_token();
        self.parse_expr(precedence)
            .map(|right| Expr::Infix(infix, Box::new(left), Box::new(right)))
    }

    fn parse_grouped_expr(&mut self) -> Option<Expr> {
        self.next_token();
        let expr = self.parse_expr(Precedence::Lowest)?;
        if !self.expect_peek(Token::RPAREN) {
            return None;
        }
        Some(expr)
    }

    fn parse_if_expr(&mut self) -> Option<Expr> {
        if !self.expect_peek(Token::LPAREN) {
            return None;
        }
        self.next_token();
        let cond = self.parse_expr(Precedence::Lowest)?;
        if !self.expect_peek(Token::RPAREN) {
            return None;
        }
        if !self.expect_peek(Token::LBRACE) {
            return None;
        }
        let cons = self.parse_block_stmt();
        if !self.is_cur_token(&Token::RBRACE) {
            return None;
        }
        let mut alt = None;
        if self.is_peek_token(&Token::ELSE) {
            self.next_token();
            if !self.expect_peek(Token::LBRACE) {
                return None;
            }
            alt = Some(self.parse_block_stmt());
            if !self.is_cur_token(&Token::RBRACE) {
                return None;
            }
        }
        Some(Expr::If(Box::new(cond), cons, alt))
    }

    fn parse_ident(&mut self) -> Option<Ident> {
        match self.cur_token {
            Token::IDENT(ref ident) => Some(Ident(ident.clone())),
            _ => unreachable!(),
        }
    }

    fn parse_ident_expr(&mut self) -> Option<Expr> {
        match self.parse_ident() {
            Some(ident) => Some(Expr::Ident(ident)),
            _ => unreachable!(),
        }
    }

    fn parse_int_expr(&mut self) -> Option<Expr> {
        match self.cur_token {
            Token::INT(value) => Some(Expr::Int(value)),
            _ => unreachable!(),
        }
    }

    fn parse_bool_expr(&mut self) -> Option<Expr> {
        match self.cur_token {
            Token::BOOL(value) => Some(Expr::Bool(value)),
            _ => unreachable!(),
        }
    }
}
