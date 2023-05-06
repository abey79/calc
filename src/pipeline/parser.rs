use crate::data::ast::{BinOp, Expr, Stmt, UnaryOp, VarName};
use crate::data::token::{Token, TokenId, TokenKind};
use crate::errors::{ParserError, SyntaxError};
use crate::states::{AstContext, ParsedInput, TokenizedInput};
use std::rc::Rc;

type Result<T> = std::result::Result<T, ParserError>;

pub(crate) fn parse(input: TokenizedInput) -> Result<ParsedInput> {
    let mut parser = Parser::new(input);
    parser.run()?;
    Ok(ParsedInput {
        text_ctx: parser.input.text_ctx,
        token_ctx: parser.input.token_ctx,
        ast_ctx: parser.ast_ctx,
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
                $self
                    .input
                    .text_ctx
                    .error_context($self.input.token_ctx.span_from_id(token.id)),
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

/// Macro to reduce the boilerplate associated with saving the token span of the parsed node.
macro_rules! parse_node {
    ($self:ident, $code:block) => {{
        let start = $self.cur_id();
        let node = { $code };
        $self.ast_ctx.push_span(node.id, start, $self.prev_id());
        Ok(node)
    }};
}

struct Parser {
    input: TokenizedInput,
    ast_ctx: AstContext,

    // state
    pos: usize,
}

impl Parser {
    fn new(input: TokenizedInput) -> Self {
        Self {
            input,
            ast_ctx: AstContext::default(),
            pos: 0,
        }
    }

    #[inline]
    fn tokens(&self) -> &[Rc<Token>] {
        &self.input.tokens()
    }

    fn peek(&self) -> Option<&TokenKind> {
        self.tokens().get(self.pos).map(|t| &t.kind)
    }

    fn next(&mut self) -> Option<Rc<Token>> {
        self.pos += 1;
        let token = self.tokens().get(self.pos - 1).map(|t| t.clone());
        token
    }

    fn cur_id(&self) -> Option<TokenId> {
        self.tokens().get(self.pos).map(|t| t.id)
    }

    fn prev_id(&self) -> Option<TokenId> {
        self.tokens().get(self.pos - 1).map(|t| t.id)
    }

    fn run(&mut self) -> Result<()> {
        while self.pos < self.tokens().len() {
            let start = self.cur_id();
            let stmt = self.parse_stmt()?;

            self.ast_ctx.push_span(stmt.id, start, self.prev_id());
            self.ast_ctx.push_stmt(stmt);
        }

        Ok(())
    }

    fn parse_stmt(&mut self) -> Result<Stmt> {
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

    fn parse_print_stmt(&mut self) -> Result<Stmt> {
        parse_node!(self, {
            expect!(self, TokenKind::Print)?;
            let expr = self.parse_expr()?;
            expect!(self, TokenKind::Semi)?;

            Stmt::print(expr)
        })
    }

    fn parse_assign_stmt(&mut self) -> Result<Stmt> {
        parse_node!(self, {
            let name = self.parse_var_name()?;
            expect!(self, TokenKind::Assign)?;
            let expr = self.parse_expr()?;
            expect!(self, TokenKind::Semi)?;

            Stmt::assign(name, expr)
        })
    }

    fn parse_expr_stmt(&mut self) -> Result<Stmt> {
        parse_node!(self, {
            let expr = self.parse_expr()?;
            expect!(self, TokenKind::Semi)?;

            Stmt::expr(expr)
        })
    }

    fn parse_expr(&mut self) -> Result<Expr> {
        self.parse_add_term()
    }

    fn parse_add_term(&mut self) -> Result<Expr> {
        let mut start = self.cur_id();
        let mut lhs = self.parse_mul_term()?;
        while let Some(op_token) = accept!(self, TokenKind::Plus | TokenKind::Minus) {
            // create binop node and push span
            let op = BinOp::new(&op_token.kind);
            self.ast_ctx
                .push_span(op.id, Some(op_token.id), Some(op_token.id));

            let rhs = self.parse_mul_term()?;
            lhs = Expr::bin_op(op, lhs, rhs);
            self.ast_ctx.push_span(lhs.id, start, self.prev_id());
            start = self.cur_id();
        }
        Ok(lhs)
    }

    fn parse_mul_term(&mut self) -> Result<Expr> {
        let mut start = self.cur_id();
        let mut lhs = self.parse_factor()?;
        while let Some(op_token) = accept!(self, TokenKind::Slash | TokenKind::Star) {
            // create binop node and push span
            let op = BinOp::new(&op_token.kind);
            self.ast_ctx
                .push_span(op.id, Some(op_token.id), Some(op_token.id));

            let rhs = self.parse_factor()?;
            lhs = Expr::bin_op(op, lhs, rhs);
            self.ast_ctx.push_span(lhs.id, start, self.prev_id());
            start = self.cur_id();
        }
        Ok(lhs)
    }

    fn parse_factor(&mut self) -> Result<Expr> {
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

    fn parse_integer(&mut self) -> Result<Expr> {
        parse_node!(self, {
            let tok = expect!(self, TokenKind::Int(_))?;
            if let TokenKind::Int(ref n) = tok.kind {
                Expr::integer(n.clone())
            } else {
                unreachable!()
            }
        })
    }

    fn parse_float(&mut self) -> Result<Expr> {
        parse_node!(self, {
            let tok = expect!(self, TokenKind::Float(_))?;
            if let TokenKind::Float(ref n) = tok.kind {
                Expr::float(n.clone())
            } else {
                unreachable!()
            }
        })
    }

    fn parse_variable(&mut self) -> Result<Expr> {
        parse_node!(self, {
            let name = self.parse_var_name()?;
            Expr::variable(name)
        })
    }

    fn parse_unary_factor(&mut self) -> Result<Expr> {
        let start = self.cur_id();
        let op_token = expect!(self, TokenKind::Plus | TokenKind::Minus)?;

        // create binop node
        let op = UnaryOp::new(&op_token.kind);
        self.ast_ctx
            .push_span(op.id, Some(op_token.id), Some(op_token.id));

        let sub_expr = self.parse_factor()?;
        let expr = Expr::unary_op(op, sub_expr);
        self.ast_ctx.push_span(expr.id, start, self.prev_id());

        Ok(expr)
    }

    fn parse_grouping(&mut self) -> Result<Expr> {
        expect!(self, TokenKind::LParen)?;
        let expr = self.parse_expr()?;
        expect!(self, TokenKind::RParen)?;
        Ok(expr)
    }

    fn parse_var_name(&mut self) -> Result<VarName> {
        parse_node!(self, {
            let tok = expect!(self, TokenKind::Name(_))?;
            if let TokenKind::Name(ref n) = tok.kind {
                VarName::new(n.clone())
            } else {
                unreachable!()
            }
        })
    }

    /// Create an EOF error.
    ///
    /// For this, we create a span based on the end location of the last token.
    fn end_of_file_err(&self) -> ParserError {
        let token_id = self.tokens().last().map(|t| t.id);

        let span = if let Some(token_id) = token_id {
            self.input.token_ctx.span_from_id(token_id)
        } else {
            None
        };

        ParserError::SyntaxError(
            SyntaxError::UnexpectedEndOfFile,
            self.input.text_ctx.error_context(span),
        )
    }
}
