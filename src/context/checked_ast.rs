//! Context data for the checked state.
//!
//! In this state, all the AST nodes are decorated with type info.

use crate::context::ast::Ast;
use crate::data::ast::{BinOp, Expr, Stmt, UnaryOp, VarName};
use crate::data::span::Span;
use crate::data::token_span::TokSpan;
use crate::errors::error_message::Spanned;
use std::fmt;
use std::fmt::Write;

pub type CheckedStmt = Stmt<TypeInfo>;
pub type CheckedExpr = Expr<TypeInfo>;
pub type CheckedAst = Ast<TypeInfo>;
pub type CheckedVarName = VarName<TypeInfo>;
pub type CheckedBinOp = BinOp<TypeInfo>;
pub type CheckedUnaryOp = UnaryOp<TypeInfo>;

impl Ast<TypeInfo> {
    pub fn dump<W: Write>(&self, w: &mut W) -> fmt::Result {
        for stmt in self.stmts() {
            writeln!(w, "\n{:#?}", stmt)?;
        }

        Ok(())
    }
}

// Note:
// This is a massive simplification of what a real type structure would be. In a realistic case,
// `CheckedAst` would contain a vector of types (including user defined ones). Then, `CheckedInfo`
// would contain a ref-counted pointer to one of the types.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    Stmt, // stmt only
    Integer,
    Float,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Type::*;
        match self {
            Stmt => write!(f, "stmt"),
            Integer => write!(f, "int"),
            Float => write!(f, "float"),
        }
    }
}

/// AST meta-data after type checking.
#[derive(Debug)]
pub struct TypeInfo {
    /// type of the node
    pub type_: Type,

    /// token span of the node (typically copied over from the parsed state)
    pub tok_span: TokSpan,
}

impl TypeInfo {
    pub fn new(type_: Type, tok_span: TokSpan) -> Self {
        Self { type_, tok_span }
    }
}

impl fmt::Display for TypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.type_)
    }
}

impl Spanned for TypeInfo {
    fn span(&self) -> Span {
        self.tok_span.span()
    }
}
