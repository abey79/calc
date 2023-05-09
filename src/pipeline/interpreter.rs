use crate::context::checked_ast::{CheckedExpr, CheckedStmt};
use crate::data::ast::{BinOpKind, ExprKind, StmtKind, UnaryOpKind};
use crate::errors::{InterpreterError, Spanned, SyntaxError, TypeError};
use crate::states::CheckedState;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;

type Result<T> = std::result::Result<T, InterpreterError>;

pub(crate) fn interpret<W: Write>(input: &CheckedState, writer: &mut W) -> Result<()> {
    let mut interpreter = Interpreter::new(input, writer);
    interpreter.run()
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Float(f64),
}

impl Value {
    fn bin_op(&self, op: &BinOpKind, other: &Value) -> Option<Value> {
        match (self, other) {
            (Self::Int(i1), Self::Int(i2)) => match op {
                BinOpKind::Add => Some(Self::Int(i1 + i2)),
                BinOpKind::Sub => Some(Self::Int(i1 - i2)),
                BinOpKind::Mul => Some(Self::Int(i1 * i2)),
                BinOpKind::Div => Some(Self::Int(i1 / i2)),
            },
            (Self::Float(f1), Self::Float(f2)) => match op {
                BinOpKind::Add => Some(Self::Float(f1 + f2)),
                BinOpKind::Sub => Some(Self::Float(f1 - f2)),
                BinOpKind::Mul => Some(Self::Float(f1 * f2)),
                BinOpKind::Div => Some(Self::Float(f1 / f2)),
            },
            _ => None,
        }
    }

    fn unary_op(&self, op: &UnaryOpKind) -> Value {
        match self {
            Self::Int(i) => match op {
                UnaryOpKind::Pos => Self::Int(*i),
                UnaryOpKind::Neg => Self::Int(-*i),
            },
            Self::Float(f) => match op {
                UnaryOpKind::Pos => Self::Float(*f),
                UnaryOpKind::Neg => Self::Float(-*f),
            },
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(i) => i.fmt(f),
            Value::Float(fl) => write!(f, "{:?}", fl),
        }
    }
}

struct Interpreter<'a, W: Write> {
    input: &'a CheckedState,
    writer: &'a mut W,

    // state
    vars: HashMap<String, Value>,
}

impl<'a, W: Write> Interpreter<'a, W> {
    fn new(input: &'a CheckedState, writer: &'a mut W) -> Self {
        Self {
            input,
            writer,
            vars: HashMap::new(),
        }
    }

    fn run(&mut self) -> Result<()> {
        for stmt in self.input.ast.stmts() {
            self.run_stmt(stmt)?;
        }
        Ok(())
    }

    fn run_stmt(&mut self, stmt: &CheckedStmt) -> Result<()> {
        match &stmt.kind {
            StmtKind::Assign { name, value } => {
                let value = self.run_expr(value)?;
                self.vars.insert(name.kind.clone(), value);
            }
            StmtKind::Print { expr } => {
                let value = self.run_expr(expr)?;
                writeln!(self.writer, "{}", value)?;
            }
            StmtKind::Expr { expr } => {
                // somewhat pointless as no side effects as possible in this language
                let _ = self.run_expr(expr)?;
            }
        }
        Ok(())
    }

    fn run_expr(&mut self, expr: &CheckedExpr) -> Result<Value> {
        match &expr.kind {
            ExprKind::Variable(name) => {
                let value = self.vars.get::<String>(name.as_ref()).ok_or_else(|| {
                    InterpreterError::SyntaxError(
                        SyntaxError::UnknownVariable(name.to_string()),
                        name.to_error(&self.input.source),
                    )
                })?;
                Ok(value.clone())
            }
            ExprKind::BinOp { op, left, right } => {
                let left_val = self.run_expr(left)?;
                let right_val = self.run_expr(right)?;
                let value = left_val.bin_op(&op.kind, &right_val).ok_or_else(|| {
                    // this should never happen as the type checker should have caught this
                    InterpreterError::TypeError(
                        TypeError::MismatchedTypesForBinaryOp(left.meta.type_, right.meta.type_),
                        op.to_error(&self.input.source),
                    )
                })?;
                Ok(value)
            }
            ExprKind::UnaryOp { op, operand } => {
                let value = self.run_expr(operand)?;
                Ok(value.unary_op(&op.kind))
            }
            ExprKind::Integer(i) => Ok(Value::Int(*i)),
            ExprKind::Float(fl) => Ok(Value::Float(*fl)),
        }
    }
}