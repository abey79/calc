use crate::context::checked_ast::{CheckedAst, CheckedExpr, CheckedStmt, Type, TypeInfo};
use crate::data::ast::{BinOp, BinOpKind, Expr, ExprKind, Stmt, StmtKind, UnaryOp, VarName};
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
    vars: HashMap<String, Type>, //TODO: custom types may be duplicated there
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
                let type_ = checked_value.meta.type_.clone();
                self.vars.insert(name.kind.clone(), type_.clone());
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
                        VarName::new(&name.kind, TypeInfo::new(type_.clone(), name.tok_span())),
                        TypeInfo::new(type_.clone(), expr.tok_span()),
                    ))
                } else {
                    Err(self.syntax_err(SyntaxError::UnknownVariable(name.kind.clone()), expr))
                }
            }
            ExprKind::BinOp { op, left, right } => {
                let checked_left = self.check_expr(left)?;
                let checked_right = self.check_expr(right)?;
                let left_type = checked_left.meta.type_.clone();
                let right_type = checked_right.meta.type_.clone();

                let res_type = self.check_bin_op_type(op, &left_type, &right_type)?;

                Ok(Expr::bin_op(
                    BinOp::new(op.kind, TypeInfo::new(res_type.clone(), op.tok_span())),
                    checked_left,
                    checked_right,
                    TypeInfo::new(res_type, expr.tok_span()),
                ))
            }
            ExprKind::UnaryOp { op, operand } => {
                let checked_expr = self.check_expr(operand)?;
                let type_ = checked_expr.meta.type_.clone();

                if !matches!(type_, Type::Integer | Type::Float) {
                    return Err(self.type_err(TypeError::InvalidTypeForUnaryOp(type_), expr));
                }

                Ok(Expr::unary_op(
                    UnaryOp::new(op.kind, TypeInfo::new(type_.clone(), op.tok_span())),
                    checked_expr,
                    TypeInfo::new(type_, expr.tok_span()),
                ))
            }
            ExprKind::Tuple(exprs) => {
                // check homogeneous

                if exprs.is_empty() {
                    return Err(self.syntax_err(SyntaxError::EmptyTuple, expr));
                }

                let mut checked_exprs = Vec::new();
                let ref_expr = self.check_expr(&exprs[0])?;
                let type_ = ref_expr.meta.type_.clone();
                checked_exprs.push(ref_expr);

                for expr in &exprs[1..] {
                    let checked_expr = self.check_expr(expr)?;
                    if checked_expr.meta.type_ != type_ {
                        return Err(self.type_err(TypeError::HeterogeneousTuple, expr));
                    } else {
                        checked_exprs.push(checked_expr);
                    }
                }

                Ok(Expr::tuple(
                    checked_exprs,
                    TypeInfo::new(
                        Type::Tuple {
                            type_: Box::new(type_),
                            len: exprs.len(),
                        },
                        expr.tok_span(),
                    ),
                ))
            }
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

    fn check_bin_op_type(
        &mut self,
        op: &BinOp<TokSpan>,
        left: &Type,
        right: &Type,
    ) -> Result<Type> {
        let res_type = match (left, right) {
            (Type::Integer, Type::Integer) => Some(Type::Integer),
            (Type::Float, Type::Float) => Some(Type::Float),
            // Element-wise addition/subtraction
            (Type::Tuple { type_: t1, len: l1 }, Type::Tuple { type_: t2, len: l2 }) => {
                if matches!(op.kind, BinOpKind::Add | BinOpKind::Sub) && t1 == t2 && l1 == l2 {
                    Some(left.clone())
                } else {
                    None
                }
            }
            // Scalar multiplication/division
            //TODO: ugly duplication
            (Type::Tuple { type_, len }, Type::Integer | Type::Float) => {
                if matches!(op.kind, BinOpKind::Mul | BinOpKind::Div) {
                    let new_type = self.check_bin_op_type(op, type_, right)?;
                    Some(Type::Tuple {
                        type_: Box::new(new_type),
                        len: *len,
                    })
                } else {
                    None
                }
            }
            (Type::Integer | Type::Float, Type::Tuple { type_, len }) => {
                if matches!(op.kind, BinOpKind::Mul | BinOpKind::Div) {
                    let new_type = self.check_bin_op_type(op, left, type_)?;
                    Some(Type::Tuple {
                        type_: Box::new(new_type),
                        len: *len,
                    })
                } else {
                    None
                }
            }
            _ => None,
        };

        res_type.ok_or(self.type_err(
            TypeError::MismatchedTypesForBinaryOp(left.clone(), right.clone()),
            op,
        ))
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
