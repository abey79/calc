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
    Tuple(Vec<Value>),
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
            (Self::Tuple(t1), Self::Tuple(t2)) => {
                // tuple addition and subtraction are element-wise
                if t1.len() != t2.len() || !matches!(op, BinOpKind::Add | BinOpKind::Sub) {
                    None
                } else {
                    let mut res = Vec::new();
                    for (v1, v2) in t1.iter().zip(t2) {
                        res.push(v1.bin_op(op, v2)?);
                    }
                    Some(Self::Tuple(res))
                }
            }
            (Self::Tuple(t), Self::Int(i)) | (Self::Int(i), Self::Tuple(t)) => {
                // tuple multiplication and division is scalar multiplication
                if !matches!(op, BinOpKind::Mul | BinOpKind::Div) {
                    None
                } else {
                    let mut res = Vec::new();
                    for v in t {
                        res.push(v.bin_op(op, &Self::Int(*i))?);
                    }
                    Some(Self::Tuple(res))
                }
            }
            (Self::Tuple(t), Self::Float(fl)) | (Self::Float(fl), Self::Tuple(t)) => {
                // tuple multiplication and division is scalar multiplication
                if !matches!(op, BinOpKind::Mul | BinOpKind::Div) {
                    None
                } else {
                    let mut res = Vec::new();
                    for v in t {
                        res.push(v.bin_op(op, &Self::Float(*fl))?);
                    }
                    Some(Self::Tuple(res))
                }
            }
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
            Self::Tuple(values) => match op {
                UnaryOpKind::Pos => Self::Tuple(values.clone()),
                UnaryOpKind::Neg => Self::Tuple(values.iter().map(|v| v.unary_op(op)).collect()),
            },
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(i) => i.fmt(f),
            Value::Float(fl) => write!(f, "{:?}", fl),
            Value::Tuple(values) => {
                write!(f, "(")?;
                for (i, value) in values.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, ")")
            }
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
                        TypeError::MismatchedTypesForBinaryOp(
                            left.meta.type_.clone(),
                            right.meta.type_.clone(),
                        ),
                        op.to_error(&self.input.source),
                    )
                })?;
                Ok(value)
            }
            ExprKind::UnaryOp { op, operand } => {
                let value = self.run_expr(operand)?;
                Ok(value.unary_op(&op.kind))
            }
            ExprKind::Tuple(exprs) => {
                let mut values = Vec::new();
                for expr in exprs {
                    values.push(self.run_expr(expr)?);
                }
                Ok(Value::Tuple(values))
            }
            ExprKind::Integer(i) => Ok(Value::Int(*i)),
            ExprKind::Float(fl) => Ok(Value::Float(*fl)),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::pipeline::checker::check;
    use crate::pipeline::interpreter::interpret;
    use crate::pipeline::parser::parse;
    use crate::pipeline::tokenizer::tokenize;
    use crate::states::InputState;

    #[test]
    fn test_interpreter() {
        let input = InputState::from(
            r###"
                a = (1.3 + 3.2) * 45.1;
                print a;
                b = a * 3.2;
                print b;
                print 1 + 2 * 3;
            "###,
        );
        let tokenized = tokenize(input).unwrap();
        let parsed = parse(tokenized).unwrap();
        let checked = check(parsed).unwrap();

        let mut output = String::new();
        interpret(&checked, &mut output).unwrap();

        insta::assert_snapshot!(output);
    }
}
