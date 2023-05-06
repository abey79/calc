use crate::data::ast::{NodeId, Stmt};
use crate::data::token::TokenId;
use crate::states::{AstContext, TokSpan};
use std::collections::BTreeMap;
use std::fmt::Write;

impl Default for AstContext {
    fn default() -> Self {
        Self {
            stmts: vec![],
            node_spans: BTreeMap::new(),
        }
    }
}

impl AstContext {
    pub fn stmts(&self) -> &[Stmt] {
        &self.stmts
    }

    pub fn push_stmt(&mut self, stmt: Stmt) {
        self.stmts.push(stmt);
    }

    pub fn push_span(&mut self, id: NodeId, from: Option<TokenId>, to: Option<TokenId>) {
        if let (Some(from), Some(to)) = (from, to) {
            self.node_spans.insert(id, TokSpan::new(from, to));
        }
    }

    pub fn dump<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        for stmt in self.stmts() {
            writeln!(w, "\n{:#?}", stmt)?;
        }

        Ok(())
    }
}
