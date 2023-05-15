//! Formatter
//!
//! This pipeline stage operates on a generic `Ast<T>` context data, as it doesn't require any AST
//! metadata. This means that it can be run on either `ParsedState` or `CheckedState`.

use crate::context::ast::Ast;
use crate::data::ast::{Expr, ExprKind, Stmt, StmtKind};
use std::fmt;
use std::fmt::{Debug, Display, Write};

pub(crate) fn format<T: Debug + Display, W: Write>(
    input: &Ast<T>,
    writer: &mut W,
) -> Result<(), fmt::Error> {
    let mut formatter = Formatter::new(input, writer);
    formatter.format()?;
    Ok(())
}

struct Formatter<'a, T: Debug + Display, W: Write> {
    input: &'a Ast<T>,
    writer: &'a mut W,
    // here there would be additional state, e.g. indentation level
    // nothing because indentation is not needed for this toy language
}

impl<'a, T: Debug + Display, W: Write> Formatter<'a, T, W> {
    fn new(input: &'a Ast<T>, writer: &'a mut W) -> Self {
        Self { input, writer }
    }

    fn format(&mut self) -> fmt::Result {
        for stmt in self.input.stmts() {
            self.format_stmt(stmt)?;
            writeln!(self.writer)?;
        }

        Ok(())
    }

    fn format_stmt(&mut self, stmt: &Stmt<T>) -> fmt::Result {
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

    fn format_expr(&mut self, expr: &Expr<T>) -> fmt::Result {
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

    fn format_expr_paren(&mut self, expr: &Expr<T>) -> fmt::Result {
        write!(self.writer, "(")?;
        self.format_expr(expr)?;
        write!(self.writer, ")")
    }
}

#[cfg(test)]
mod test {
    use crate::pipeline::checker::check;
    use crate::pipeline::formatter::format;
    use crate::pipeline::optimizer::optimize;
    use crate::pipeline::parser::parse;
    use crate::pipeline::tokenizer::tokenize;
    use crate::states::InputState;

    #[test]
    fn test_formatter_from_raw_ast() {
        let input = InputState::from("a = (1.3 + 3.2) * 45.1; b = a * 3.2; print 1 + 2 * 3;");
        let tokenized = tokenize(input).unwrap();
        let parsed = parse(tokenized).unwrap();

        let mut output = String::new();
        format(&parsed.raw_ast, &mut output).unwrap();

        insta::assert_debug_snapshot!(output);
    }

    #[test]
    fn test_formatter_from_checked_ast() {
        let input = InputState::from("a = (1.3 + 3.2) * 45.1; b = a * 3.2; print 1 + 2 * 3;");
        let tokenized = tokenize(input).unwrap();
        let parsed = parse(tokenized).unwrap();
        let checked = check(parsed).unwrap();

        let mut output = String::new();
        format(&checked.ast, &mut output).unwrap();

        insta::assert_debug_snapshot!(output);
    }

    #[test]
    fn test_formatter_from_optimized_ast() {
        let input = InputState::from("a = (1.3 + 3.2) * 45.1; b = a * 3.2; print 1 + 2 * 3;");
        let tokenized = tokenize(input).unwrap();
        let parsed = parse(tokenized).unwrap();
        let checked = check(parsed).unwrap();
        let optimized = optimize(checked);

        let mut output = String::new();
        format(&optimized.ast, &mut output).unwrap();

        insta::assert_debug_snapshot!(output);
    }
}
