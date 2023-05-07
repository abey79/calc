use crate::data::ast::{Expr, ExprKind, Stmt, StmtKind};
use crate::data::token_span::TokSpan;
use crate::states::ParsedState;
use std::fmt;
use std::fmt::Write;

pub(crate) fn format<W: Write>(input: &ParsedState, writer: &mut W) -> Result<String, fmt::Error> {
    let mut formatter = Formatter::new(input, writer);
    formatter.format()?;
    Ok(String::new())
}

struct Formatter<'a, W: Write> {
    input: &'a ParsedState,
    writer: &'a mut W,
    // here there would be additional state, e.g. indentation level
    // nothing because indentation is not needed for this toy language
}

impl<'a, W: Write> Formatter<'a, W> {
    fn new(input: &'a ParsedState, writer: &'a mut W) -> Self {
        Self { input, writer }
    }

    fn format(&mut self) -> fmt::Result {
        for stmt in self.input.ast.stmts() {
            self.format_stmt(stmt)?;
            writeln!(self.writer)?;
        }

        Ok(())
    }

    fn format_stmt(&mut self, stmt: &Stmt<TokSpan>) -> fmt::Result {
        match &stmt.kind {
            StmtKind::Assign { name, value } => {
                write!(self.writer, "{} = ", name)?;
                self.format_expr(value)?;
                write!(self.writer, ";")?;
            }
            StmtKind::Print { expr } => {
                write!(self.writer, "print ")?;
                self.format_expr(expr)?;
                write!(self.writer, ";")?;
            }
            StmtKind::Expr { expr } => {
                self.format_expr(expr)?;
                write!(self.writer, ";")?;
            }
        }

        Ok(())
    }

    fn format_expr(&mut self, expr: &Expr<TokSpan>) -> fmt::Result {
        match &expr.kind {
            ExprKind::Variable(name) => write!(self.writer, "{}", name)?,
            ExprKind::BinOp { op, left, right } => {
                let left_precedence = left.kind.precedence();
                let right_precedence = right.kind.precedence();
                let op_precedence = op.kind.precedence();
                if left_precedence < op_precedence {
                    self.format_expr_paren(left)?;
                } else {
                    self.format_expr(left)?;
                }
                write!(self.writer, " {} ", op)?;
                if right_precedence < op_precedence {
                    self.format_expr_paren(right)?;
                } else {
                    self.format_expr(right)?;
                }
            }
            ExprKind::UnaryOp { op, operand } => {
                let op_precedence = op.kind.precedence();
                let operand_precedence = operand.kind.precedence();
                write!(self.writer, "{}", op)?;

                if operand_precedence < op_precedence {
                    self.format_expr_paren(operand)?;
                } else {
                    self.format_expr(operand)?;
                }
            }
            ExprKind::Integer(i) => write!(self.writer, "{}", i)?,
            ExprKind::Float(f) => write!(self.writer, "{:?}", f)?,
        }

        Ok(())
    }

    fn format_expr_paren(&mut self, expr: &Expr<TokSpan>) -> fmt::Result {
        write!(self.writer, "(")?;
        self.format_expr(expr)?;
        write!(self.writer, ")")
    }
}
