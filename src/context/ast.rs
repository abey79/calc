use crate::data::ast::{NodeId, Stmt};
use crate::data::token::TokenId;
use crate::data::token_span::TokSpan;
use std::collections::BTreeMap;
use std::fmt::Write;

#[derive(Debug, Default)]
pub struct Ast {
    stmts: Vec<Stmt>,
    node_spans: BTreeMap<NodeId, TokSpan>,
}

impl Ast {
    pub fn stmts(&self) -> &[Stmt] {
        &self.stmts
    }

    pub fn stmts_mut(&mut self) -> &mut Vec<Stmt> {
        &mut self.stmts
    }

    pub fn push_stmt(&mut self, stmt: Stmt) {
        self.stmts.push(stmt);
    }

    pub fn push_span(&mut self, id: NodeId, from: Option<TokenId>, to: Option<TokenId>) {
        if let (Some(from), Some(to)) = (from, to) {
            self.node_spans.insert(id, TokSpan::new(from, to));
        }
    }

    pub fn copy_span(&mut self, from: NodeId, to: NodeId) {
        if from == to {
            return;
        }
        if let Some(span) = self.node_spans.get(&from) {
            self.node_spans.insert(to, *span);
        }
    }

    pub fn dump<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        for stmt in self.stmts() {
            writeln!(w, "\n{:#?}", stmt)?;
        }

        Ok(())
    }
}
