use crate::data::ast::Stmt;
use crate::data::meta::Meta;
use crate::data::span::Span;
use crate::data::token_span::TokSpan;
use crate::pipeline;
use std::fmt;
use std::fmt::{Debug, Display, Write};

#[derive(Debug, Default)]
pub struct Ast<M: Debug + Display> {
    stmts: Vec<Stmt<M>>,
}

impl<M: Debug + Display> Ast<M> {
    pub fn new() -> Self {
        Self { stmts: Vec::new() }
    }

    pub fn stmts(&self) -> &[Stmt<M>] {
        &self.stmts
    }

    pub fn stmts_mut(&mut self) -> &mut Vec<Stmt<M>> {
        &mut self.stmts
    }

    pub fn push_stmt(&mut self, stmt: Stmt<M>) {
        self.stmts.push(stmt);
    }

    pub fn format<W: fmt::Write>(&self, w: &mut W) -> Result<String, fmt::Error> {
        pipeline::formatter::format(self, w)
    }
}

impl Ast<TokSpan> {
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
