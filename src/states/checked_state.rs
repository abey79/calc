//! State after the type checker.
//!
//! In this state, all the AST nodes are decorated with type info.

use crate::context::ast::Ast;
use crate::context::checked_ast::CheckedAst;
use crate::context::source::Source;
use crate::context::token_stream::TokenStream;
use crate::data::token_span::TokSpan;
use crate::errors::InterpreterError;
use crate::pipeline;
use std::fmt;
use std::fmt::Write;

pub struct CheckedState {
    pub(crate) source: Source,
    pub(crate) token_stream: TokenStream,
    pub(crate) raw_ast: Ast<TokSpan>,
    pub(crate) ast: CheckedAst,
}

impl CheckedState {
    pub fn optimize(self) -> Self {
        pipeline::optimizer::optimize(self)
    }

    pub fn interpret<W: Write>(&self, writer: &mut W) -> Result<(), InterpreterError> {
        pipeline::interpreter::interpret(self, writer)
    }

    pub fn llvm_codegen<W: Write>(&self, writer: &mut W) -> Result<(), fmt::Error> {
        pipeline::llvm::llvm_codegen(self, writer)
    }
}
