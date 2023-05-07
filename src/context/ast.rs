use crate::data::ast::Stmt;
use crate::data::meta::Meta;
use crate::data::span::Span;
use crate::data::token_span::TokSpan;
use std::fmt::Write;

#[derive(Debug, Default)]
pub struct Ast {
    stmts: Vec<Stmt<TokSpan>>,
}

impl Ast {
    pub fn stmts(&self) -> &[Stmt<TokSpan>] {
        &self.stmts
    }

    pub fn stmts_mut(&mut self) -> &mut Vec<Stmt<TokSpan>> {
        &mut self.stmts
    }

    pub fn push_stmt(&mut self, stmt: Stmt<TokSpan>) {
        self.stmts.push(stmt);
    }

    pub fn dump<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        for stmt in self.stmts() {
            writeln!(w, "\n{:#?}", stmt)?;
        }

        Ok(())
    }
}

// Provide convenient methods for AST nodes parametrized by a token span.
impl<K> Meta<K, TokSpan> {
    pub fn tok_span(&self) -> TokSpan {
        self.meta.clone()
    }

    pub fn span(&self) -> Span {
        self.meta.span()
    }
}
