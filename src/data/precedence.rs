//! Central place for precedence values.
//!
//! Values are taken from my original Wabbit implementation, which explains the "holes".

use crate::data::ast::{BinOpKind, ExprKind, UnaryOpKind};
use std::fmt::{Debug, Display};

impl BinOpKind {
    pub const fn precedence(&self) -> u8 {
        match self {
            Self::Add | BinOpKind::Sub => 4,
            Self::Mul | BinOpKind::Div => 5,
        }
    }
}

impl UnaryOpKind {
    pub const fn precedence(&self) -> u8 {
        6
    }
}

impl<T: Debug + Display> ExprKind<T> {
    pub fn precedence(&self) -> u8 {
        match self {
            Self::BinOp { op, .. } => op.kind.precedence(),
            Self::UnaryOp { op, .. } => op.kind.precedence(),
            Self::Variable(_) | Self::Integer(_) | Self::Float(_) | Self::Tuple(_) => 255,
        }
    }
}
