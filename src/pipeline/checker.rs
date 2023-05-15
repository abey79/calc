use crate::context::checked_ast::{CheckedAst, CheckedExpr, CheckedStmt, Type, TypeInfo};
use crate::data::ast::{BinOp, Expr, ExprKind, Stmt, StmtKind, UnaryOp, VarName};
use crate::data::meta::Meta;
use crate::data::token_span::TokSpan;
use crate::errors::error_message::Spanned;
use crate::errors::{CheckerError, SyntaxError, TypeError};
use crate::states::{CheckedState, ParsedState};
use std::collections::HashMap;

type Result<T> = std::result::Result<T, CheckerError>;

pub(crate) fn check(input: ParsedState) -> Result<CheckedState> {
    let mut checker = Checker::new(&input);
    let checked_ast = checker.run()?;
    Ok(CheckedState {
        source: input.source,
        token_stream: input.token_stream,
        raw_ast: input.raw_ast,
        ast: checked_ast,
    })
}

struct Checker<'a> {
    input: &'a ParsedState,

    // state
    vars: HashMap<String, Type>,
}

impl<'a> Checker<'a> {
    fn new(input: &'a ParsedState) -> Self {
        Self {
            input,
            vars: HashMap::new(),
        }
    }

    fn run(&mut self) -> Result<CheckedAst> {
        let mut checked_ast = CheckedAst::new();
        for stmt in self.input.raw_ast.stmts() {
            checked_ast.push_stmt(self.check_stmt(stmt)?);
        }

        Ok(checked_ast)
    }

    fn check_stmt(&mut self, stmt: &Stmt<TokSpan>) -> Result<CheckedStmt> {
        match &stmt.kind {
            StmtKind::Assign { name, value } => {
                let checked_value = self.check_expr(value)?;
                let type_ = checked_value.meta.type_;
                self.vars.insert(name.kind.clone(), type_);
                Ok(Stmt::assign(
                    VarName::new(&name.kind, TypeInfo::new(type_, name.tok_span())),
                    checked_value,
                    TypeInfo::new(Type::Stmt, stmt.tok_span()),
                ))
            }
            StmtKind::Print { expr } => {
                let checked_expr = self.check_expr(expr)?;
                Ok(Stmt::print(
                    checked_expr,
                    TypeInfo::new(Type::Stmt, stmt.tok_span()),
                ))
            }
            StmtKind::Expr { expr } => {
                let checked_expr = self.check_expr(expr)?;
                Ok(Stmt::expr(
                    checked_expr,
                    TypeInfo::new(Type::Stmt, stmt.tok_span()),
                ))
            }
        }
    }

    fn check_expr(&mut self, expr: &Expr<TokSpan>) -> Result<CheckedExpr> {
        match &expr.kind {
            ExprKind::Variable(name) => {
                if let Some(type_) = self.vars.get(&name.kind) {
                    Ok(Expr::variable(
                        VarName::new(&name.kind, TypeInfo::new(*type_, name.tok_span())),
                        TypeInfo::new(*type_, expr.tok_span()),
                    ))
                } else {
                    Err(self.syntax_err(SyntaxError::UnknownVariable(name.kind.clone()), expr))
                }
            }
            ExprKind::BinOp { op, left, right } => {
                let checked_left = self.check_expr(left)?;
                let checked_right = self.check_expr(right)?;
                let left_type = checked_left.meta.type_;
                let right_type = checked_right.meta.type_;

                if left_type != right_type {
                    return Err(self.type_err(
                        TypeError::MismatchedTypesForBinaryOp(left_type, right_type),
                        op,
                    ));
                }

                Ok(Expr::bin_op(
                    BinOp::new(op.kind, TypeInfo::new(left_type, op.tok_span())),
                    checked_left,
                    checked_right,
                    TypeInfo::new(left_type, expr.tok_span()),
                ))
            }
            ExprKind::UnaryOp { op, operand } => {
                let checked_expr = self.check_expr(operand)?;
                let type_ = checked_expr.meta.type_;

                if !matches!(type_, Type::Integer | Type::Float) {
                    return Err(self.type_err(TypeError::InvalidTypeForUnaryOp(type_), expr));
                }

                Ok(Expr::unary_op(
                    UnaryOp::new(op.kind, TypeInfo::new(type_, op.tok_span())),
                    checked_expr,
                    TypeInfo::new(type_, expr.tok_span()),
                ))
            }
            ExprKind::Tuple(..) => todo!(),
            ExprKind::Integer(i) => Ok(Expr::integer(
                *i,
                TypeInfo::new(Type::Integer, expr.tok_span()),
            )),
            ExprKind::Float(fl) => Ok(Expr::float(
                *fl,
                TypeInfo::new(Type::Float, expr.tok_span()),
            )),
        }
    }

    fn type_err<K>(&self, err: TypeError, node: &Meta<K, TokSpan>) -> CheckerError {
        CheckerError::TypeError(err, node.to_error(&self.input.source))
    }

    fn syntax_err<K>(&self, err: SyntaxError, node: &Meta<K, TokSpan>) -> CheckerError {
        CheckerError::SyntaxError(err, node.to_error(&self.input.source))
    }
}

#[cfg(test)]
mod tests {
    use crate::pipeline::checker::check;
    use crate::pipeline::parser::parse;
    use crate::pipeline::tokenizer::tokenize;
    use crate::states::InputState;

    #[test]
    fn test_checker() {
        let input = InputState::from("a = (1.3 + 3.2) * 45.1; b = a * 3.2; print 1 + 2 * 3;");
        let tokenized = tokenize(input).unwrap();
        let parsed = parse(tokenized).unwrap();
        let checked = check(parsed).unwrap();

        insta::assert_debug_snapshot!(checked.ast);
    }
}
