use crate::context::ast::Ast;
use crate::data::ast::{BinOp, Expr, Stmt, UnaryOp, VarName};
use crate::data::token::{Token, TokenKind};
use crate::data::token_span::TokSpan;
use crate::errors::{ParserError, Spanned, SyntaxError};
use crate::states::{ParsedState, TokenizedState};
use std::rc::Rc;

type Result<T> = std::result::Result<T, ParserError>;

pub(crate) fn parse(input: TokenizedState) -> Result<ParsedState> {
    let mut parser = Parser::new(input);
    parser.run()?;
    Ok(ParsedState {
        source: parser.input.source,
        token_stream: parser.input.token_stream,
        raw_ast: parser.ast,
    })
}

/// Returns the next token if it matches and advances the parser, or returns an error.
macro_rules! expect {
    ($self:ident, $kind:pat) => {{
        let token = $self
            .tokens()
            .get($self.pos)
            .ok_or_else(|| $self.end_of_file_err())?;

        if matches!(token.kind, $kind) {
            Ok($self.next().unwrap())
        } else {
            Err(ParserError::SyntaxError(
                SyntaxError::UnexpectedToken(token.kind.clone()),
                token.to_error(&$self.input.source),
            ))
        }
    }};
}

/// Returns an `Option<Token>` if the token matches, in which case the parser advances
macro_rules! accept {
    ($self:ident, $kind:pat) => {{
        $self.pos += 1; // must be moved before the get to avoid double-borrowing
        let token = $self.tokens().get($self.pos - 1);
        if let Some(token) = token {
            if matches!(token.kind, $kind) {
                Some(token)
            } else {
                $self.pos -= 1;
                None
            }
        } else {
            $self.pos -= 1;
            None
        }
    }};
}

struct Parser {
    input: TokenizedState,
    ast: Ast<TokSpan>,

    // state
    pos: usize,

    /// stack to easily keep track of start/end tokens to compute TokSpan
    token_stack: Vec<Rc<Token>>,
}

impl Parser {
    fn new(input: TokenizedState) -> Self {
        Self {
            input,
            ast: Ast::new(),
            pos: 0,
            token_stack: Vec::new(),
        }
    }

    #[inline]
    fn tokens(&self) -> &[Rc<Token>] {
        self.input.tokens()
    }

    fn peek(&self) -> Option<&TokenKind> {
        self.tokens().get(self.pos).map(|t| &t.kind)
    }

    fn next(&mut self) -> Option<Rc<Token>> {
        self.pos += 1;
        let token = self.tokens().get(self.pos - 1).cloned();
        token
    }

    fn cur_tok(&self) -> Result<&Rc<Token>> {
        self.tokens().get(self.pos).ok_or(self.end_of_file_err())
    }

    fn prev_tok(&self) -> Result<&Rc<Token>> {
        self.tokens()
            .get(self.pos - 1)
            .ok_or(ParserError::InternalError)
    }

    fn mark_start(&mut self) -> Result<()> {
        self.token_stack.push(self.cur_tok()?.clone());
        Ok(())
    }

    fn mark_end(&mut self) -> Result<TokSpan> {
        let start = self.token_stack.pop().ok_or(ParserError::InternalError)?;
        Ok(TokSpan::new(start, self.prev_tok()?.clone()))
    }

    fn run(&mut self) -> Result<()> {
        while self.pos < self.tokens().len() {
            let stmt = self.parse_stmt()?;
            self.ast.push_stmt(stmt);
        }

        Ok(())
    }

    fn parse_stmt(&mut self) -> Result<Stmt<TokSpan>> {
        let start_pos = self.pos;
        match self.peek() {
            Some(TokenKind::Print) => self.parse_print_stmt(),
            Some(TokenKind::Name(_)) => {
                // here an expr stmt could be confused with an assignment stmt
                let res = self.parse_assign_stmt();
                if res.is_ok() {
                    res
                } else {
                    self.pos = start_pos;
                    self.parse_expr_stmt()
                }
            }
            Some(_) => self.parse_expr_stmt(),
            None => Err(self.end_of_file_err()),
        }
    }

    fn parse_print_stmt(&mut self) -> Result<Stmt<TokSpan>> {
        self.mark_start()?;

        expect!(self, TokenKind::Print)?;
        let expr = self.parse_expr()?;
        expect!(self, TokenKind::Semi)?;

        Ok(Stmt::print(expr, self.mark_end()?))
    }

    fn parse_assign_stmt(&mut self) -> Result<Stmt<TokSpan>> {
        self.mark_start()?;

        let name = self.parse_var_name()?;
        expect!(self, TokenKind::Assign)?;
        let expr = self.parse_expr()?;
        expect!(self, TokenKind::Semi)?;

        Ok(Stmt::assign(name, expr, self.mark_end()?))
    }

    fn parse_expr_stmt(&mut self) -> Result<Stmt<TokSpan>> {
        self.mark_start()?;

        let expr = self.parse_expr()?;
        expect!(self, TokenKind::Semi)?;

        Ok(Stmt::expr(expr, self.mark_end()?))
    }

    fn parse_expr(&mut self) -> Result<Expr<TokSpan>> {
        self.parse_add_term()
    }

    fn parse_add_term(&mut self) -> Result<Expr<TokSpan>> {
        let mut start = self.cur_tok()?.clone();
        let mut lhs = self.parse_mul_term()?;
        while let Some(op_token) = accept!(self, TokenKind::Plus | TokenKind::Minus) {
            let op = BinOp::new(
                &op_token.kind,
                TokSpan::new(op_token.clone(), op_token.clone()),
            );

            let rhs = self.parse_mul_term()?;

            lhs = Expr::bin_op(
                op,
                lhs,
                rhs,
                TokSpan::new(start.clone(), self.prev_tok()?.clone()),
            );

            start = self.cur_tok()?.clone();
        }
        Ok(lhs)
    }

    fn parse_mul_term(&mut self) -> Result<Expr<TokSpan>> {
        // Note:
        // This code is 100% duplicated from parse_add_term, except for:
        // - the operators pattern
        // - the parse sub-function called
        // This should be cleaned up with a macro if we were to add more stages
        let mut start = self.cur_tok()?.clone();
        let mut lhs = self.parse_factor()?;
        while let Some(op_token) = accept!(self, TokenKind::Star | TokenKind::Slash) {
            let op = BinOp::new(
                &op_token.kind,
                TokSpan::new(op_token.clone(), op_token.clone()),
            );

            let rhs = self.parse_factor()?;

            lhs = Expr::bin_op(
                op,
                lhs,
                rhs,
                TokSpan::new(start.clone(), self.prev_tok()?.clone()),
            );

            start = self.cur_tok()?.clone();
        }
        Ok(lhs)
    }

    fn parse_factor(&mut self) -> Result<Expr<TokSpan>> {
        match self.peek() {
            Some(TokenKind::Int(_)) => self.parse_integer(),
            Some(TokenKind::Float(_)) => self.parse_float(),
            Some(TokenKind::Name(_)) => self.parse_variable(),
            Some(TokenKind::Minus) | Some(TokenKind::Plus) => self.parse_unary_factor(),
            Some(TokenKind::LParen) => self.parse_grouping(),
            Some(_) => unimplemented!(), //TODO: error handling
            None => Err(self.end_of_file_err()),
        }
    }

    fn parse_integer(&mut self) -> Result<Expr<TokSpan>> {
        self.mark_start()?;
        let tok = expect!(self, TokenKind::Int(_))?;
        if let TokenKind::Int(ref n) = tok.kind {
            Ok(Expr::integer(*n, self.mark_end()?))
        } else {
            unreachable!()
        }
    }

    fn parse_float(&mut self) -> Result<Expr<TokSpan>> {
        self.mark_start()?;
        let tok = expect!(self, TokenKind::Float(_))?;
        if let TokenKind::Float(ref n) = tok.kind {
            Ok(Expr::float(*n, self.mark_end()?))
        } else {
            unreachable!()
        }
    }

    fn parse_variable(&mut self) -> Result<Expr<TokSpan>> {
        self.mark_start()?;
        let name = self.parse_var_name()?;
        Ok(Expr::variable(name, self.mark_end()?))
    }

    fn parse_unary_factor(&mut self) -> Result<Expr<TokSpan>> {
        self.mark_start()?;
        let op_token = expect!(self, TokenKind::Plus | TokenKind::Minus)?;

        // create binop node
        let op = UnaryOp::new(
            &op_token.kind,
            TokSpan::new(op_token.clone(), op_token.clone()),
        );

        let sub_expr = self.parse_factor()?;

        Ok(Expr::unary_op(op, sub_expr, self.mark_end()?))
    }

    fn parse_grouping(&mut self) -> Result<Expr<TokSpan>> {
        expect!(self, TokenKind::LParen)?;
        let expr = self.parse_expr()?;
        expect!(self, TokenKind::RParen)?;
        Ok(expr)
    }

    fn parse_var_name(&mut self) -> Result<VarName<TokSpan>> {
        self.mark_start()?;
        let tok = expect!(self, TokenKind::Name(_))?;
        if let TokenKind::Name(ref n) = tok.kind {
            Ok(VarName::new(n.clone(), self.mark_end()?))
        } else {
            unreachable!()
        }
    }

    /// Create an EOF error.
    ///
    /// For this, we create a span based on the end location of the last token.
    fn end_of_file_err(&self) -> ParserError {
        let msg = self
            .tokens()
            .last()
            .map(|t| t.to_error(&self.input.source))
            .unwrap_or_default();

        ParserError::SyntaxError(SyntaxError::UnexpectedEndOfFile, msg)
    }
}
