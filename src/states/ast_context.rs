use crate::data::ast::{Block, NodeId, Stmt};
use crate::data::token::TokenId;
use crate::states::{AstContext, TokSpan};
use std::collections::BTreeMap;
use std::fmt::Write;

impl Default for AstContext {
    fn default() -> Self {
        Self {
            nodes: Block::new(vec![]),
            node_spans: BTreeMap::new(),
        }
    }
}

impl AstContext {
    pub fn push_stmt(&mut self, stmt: Stmt) {
        self.nodes.kind.stmts.push(stmt);
    }

    pub fn push_span(&mut self, id: NodeId, from: Option<TokenId>, to: Option<TokenId>) {
        if let (Some(from), Some(to)) = (from, to) {
            self.node_spans.insert(id, TokSpan::new(from, to));
        }
    }

    pub fn dump<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        writeln!(w, "{:#?}", self.nodes)
    }
}
