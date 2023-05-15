//! This module contains the AST data structures.
//!
//! Throughout the code, the "Node" terminology is used to refer to elements of the AST. AST nodes
//! are assigned a unique `NodeId` when created. Factory functions are also provided to make it easy
//! to build AST nodes.

use crate::data::meta::Meta;
use crate::data::token::TokenKind;
use std::fmt;
use std::fmt::{Debug, Display};

// =================================================================================================
// DATA STRUCTURES

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOpKind {
    Pos,
    Neg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind<T: Debug + Display> {
    Variable(VarName<T>),
    BinOp {
        op: BinOp<T>,
        left: Box<Expr<T>>,
        right: Box<Expr<T>>,
    },
    UnaryOp {
        op: UnaryOp<T>,
        operand: Box<Expr<T>>,
    },
    Tuple(Vec<Expr<T>>),
    Integer(i32),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum StmtKind<T: Debug + Display> {
    Assign { name: VarName<T>, value: Expr<T> },
    Print { expr: Expr<T> },
    Expr { expr: Expr<T> },
}

pub type BinOp<T> = Meta<BinOpKind, T>;
pub type UnaryOp<T> = Meta<UnaryOpKind, T>;
pub type VarName<T> = Meta<String, T>;
pub type Expr<T> = Meta<ExprKind<T>, T>;
pub type Stmt<T> = Meta<StmtKind<T>, T>;

// =================================================================================================
// FACTORIES

impl From<&TokenKind> for UnaryOpKind {
    fn from(value: &TokenKind) -> Self {
        match value {
            TokenKind::Plus => Self::Pos,
            TokenKind::Minus => Self::Neg,
            _ => panic!("Invalid token kind: {:?}", value),
        }
    }
}

impl<T> UnaryOp<T> {
    pub fn new(kind: impl Into<UnaryOpKind>, meta: impl Into<T>) -> Self {
        Self {
            kind: kind.into(),
            meta: meta.into(),
        }
    }
}

impl From<&TokenKind> for BinOpKind {
    fn from(value: &TokenKind) -> Self {
        match value {
            TokenKind::Plus => Self::Add,
            TokenKind::Minus => Self::Sub,
            TokenKind::Star => Self::Mul,
            TokenKind::Slash => Self::Div,
            _ => panic!("Invalid token kind: {:?}", value),
        }
    }
}

impl<T> BinOp<T> {
    pub fn new(kind: impl Into<BinOpKind>, meta: impl Into<T>) -> Self {
        Self {
            kind: kind.into(),
            meta: meta.into(),
        }
    }
}

impl<T> VarName<T> {
    pub fn new(name: impl Into<String>, meta: impl Into<T>) -> Self {
        Self {
            kind: name.into(),
            meta: meta.into(),
        }
    }
}

impl<T: Debug + Display> Expr<T> {
    pub fn variable(name: impl Into<VarName<T>>, meta: impl Into<T>) -> Self {
        Self {
            kind: ExprKind::Variable(name.into()),
            meta: meta.into(),
        }
    }

    pub fn bin_op(op: BinOp<T>, left: Expr<T>, right: Expr<T>, meta: impl Into<T>) -> Self {
        Self {
            kind: ExprKind::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            },
            meta: meta.into(),
        }
    }

    pub fn unary_op(op: UnaryOp<T>, operand: Expr<T>, meta: impl Into<T>) -> Self {
        Self {
            kind: ExprKind::UnaryOp {
                op,
                operand: Box::new(operand),
            },
            meta: meta.into(),
        }
    }

    pub fn tuple(exprs: Vec<Expr<T>>, meta: impl Into<T>) -> Self {
        Self {
            kind: ExprKind::Tuple(exprs),
            meta: meta.into(),
        }
    }

    pub fn integer(value: i32, meta: impl Into<T>) -> Self {
        Self {
            kind: ExprKind::Integer(value),
            meta: meta.into(),
        }
    }

    pub fn float(value: f64, meta: impl Into<T>) -> Self {
        Self {
            kind: ExprKind::Float(value),
            meta: meta.into(),
        }
    }
}

impl<T: Debug + Display> Stmt<T> {
    pub fn assign(name: impl Into<VarName<T>>, value: Expr<T>, meta: impl Into<T>) -> Self {
        Self {
            kind: StmtKind::Assign {
                name: name.into(),
                value,
            },
            meta: meta.into(),
        }
    }

    pub fn print(expr: Expr<T>, meta: impl Into<T>) -> Self {
        Self {
            kind: StmtKind::Print { expr },
            meta: meta.into(),
        }
    }

    pub fn expr(expr: Expr<T>, meta: impl Into<T>) -> Self {
        Self {
            kind: StmtKind::Expr { expr },
            meta: meta.into(),
        }
    }
}

// =================================================================================================
// UTILITY TRAITS

impl fmt::Display for BinOpKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
        }
    }
}

impl fmt::Display for UnaryOpKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Neg => write!(f, "-"),
            Self::Pos => write!(f, "+"),
        }
    }
}
