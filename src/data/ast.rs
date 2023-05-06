//! This module contains the AST data structures.
//!
//! Throughout the code, the "Node" terminology is used to refer to elements of the AST. AST nodes
//! are assigned a unique `NodeId` when created. Factory functions are also provided to make it easy
//! to build AST nodes.

use crate::data::identified::{new_id, Identified};
use crate::data::token::TokenKind;
use std::fmt;

// =================================================================================================
// DATA STRUCTURES

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeId(usize);

impl NodeId {
    pub fn new() -> Self {
        Self(new_id())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOpKind {
    Pos,
    Neg,
}

pub type UnaryOp = Identified<UnaryOpKind, NodeId>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
}

pub type BinOp = Identified<BinOpKind, NodeId>;

pub type VarName = Identified<String, NodeId>;

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Variable(VarName),
    BinOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOp,
        operand: Box<Expr>,
    },
    Integer(i32),
    Float(f64),
}

pub type Expr = Identified<ExprKind, NodeId>;

#[derive(Debug, Clone, PartialEq)]
pub enum StmtKind {
    Assign { name: VarName, value: Expr },
    Print { expr: Expr },
    Expr { expr: Expr },
}

pub type Stmt = Identified<StmtKind, NodeId>;

#[derive(Debug, Clone, PartialEq)]
pub struct BlockKind {
    pub stmts: Vec<Stmt>,
}

pub type Block = Identified<BlockKind, NodeId>;

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

impl UnaryOp {
    pub fn new(kind: impl Into<UnaryOpKind>) -> Self {
        Self {
            kind: kind.into(),
            id: NodeId::new(),
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

impl BinOp {
    pub fn new(kind: impl Into<BinOpKind>) -> Self {
        Self {
            kind: kind.into(),
            id: NodeId::new(),
        }
    }
}

impl VarName {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            kind: name.into(),
            id: NodeId::new(),
        }
    }
}

impl Expr {
    pub fn variable(name: impl Into<VarName>) -> Self {
        Self {
            kind: ExprKind::Variable(name.into()),
            id: NodeId::new(),
        }
    }

    pub fn bin_op(op: BinOp, left: Expr, right: Expr) -> Self {
        Self {
            kind: ExprKind::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            },
            id: NodeId::new(),
        }
    }

    pub fn unary_op(op: UnaryOp, operand: Expr) -> Self {
        Self {
            kind: ExprKind::UnaryOp {
                op,
                operand: Box::new(operand),
            },
            id: NodeId::new(),
        }
    }

    pub fn integer(value: i32) -> Self {
        Self {
            kind: ExprKind::Integer(value),
            id: NodeId::new(),
        }
    }

    pub fn float(value: f64) -> Self {
        Self {
            kind: ExprKind::Float(value),
            id: NodeId::new(),
        }
    }
}

impl Stmt {
    pub fn assign(name: impl Into<VarName>, value: Expr) -> Self {
        Self {
            kind: StmtKind::Assign {
                name: name.into(),
                value,
            },
            id: NodeId::new(),
        }
    }

    pub fn print(expr: Expr) -> Self {
        Self {
            kind: StmtKind::Print { expr },
            id: NodeId::new(),
        }
    }

    pub fn expr(expr: Expr) -> Self {
        Self {
            kind: StmtKind::Expr { expr },
            id: NodeId::new(),
        }
    }
}

impl Block {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Self {
            kind: BlockKind { stmts },
            id: NodeId::new(),
        }
    }
}

// =================================================================================================
// UTILITY TRAITS

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

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

impl From<&str> for VarName {
    fn from(s: &str) -> Self {
        Self {
            kind: s.to_string(),
            id: NodeId::new(),
        }
    }
}
