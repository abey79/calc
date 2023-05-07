use crate::context::ast::Ast;
use crate::context::checked_ast::CheckedAst;
use crate::context::source::Source;
use crate::context::token_stream::TokenStream;
use crate::pipeline;

pub struct CheckedState {
    pub(crate) source: Source,
    pub(crate) token_stream: TokenStream,
    pub(crate) ast: Ast,
    pub(crate) checked_ast: CheckedAst,
}

impl CheckedState {
    pub fn optimize(self) -> Self {
        pipeline::optimizer::optimize(self)
    }
}
