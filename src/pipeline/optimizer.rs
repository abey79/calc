use crate::context::checked_ast::{CheckedExpr, CheckedStmt};
use crate::data::ast::{BinOp, BinOpKind, Expr, ExprKind, Stmt, StmtKind};
use crate::errors::OptimizerError;
use crate::states::CheckedState;
use std::ops::{Add, Div, Mul, Sub};

type Result<T> = std::result::Result<T, OptimizerError>;

pub(crate) fn optimize(input: CheckedState) -> CheckedState {
    let optimizer = Optimizer::new(input);
    optimizer.run()
}

struct Optimizer {
    input: CheckedState,
}

// Note:
// As it stands, this object is useless as not local state is needed, and could be replaced by a
// functions. Clippy rightly complains about this, thus the #[allow(only_used_in_recursion)].
// However, improved optimisation would require state (e.g. variable substitution).

#[allow(clippy::only_used_in_recursion)]
impl Optimizer {
    fn new(input: CheckedState) -> Self {
        Self { input }
    }

    fn run(mut self) -> CheckedState {
        let old_stmts: Vec<_> = self.input.ast.stmts_mut().drain(..).collect();
        old_stmts.into_iter().for_each(|stmt| {
            let new_stmt = self.optimize_stmt(stmt);
            self.input.ast.push_stmt(new_stmt);
        });

        self.input
    }

    fn optimize_stmt(&mut self, stmt: CheckedStmt) -> CheckedStmt {
        match stmt.kind {
            StmtKind::Expr { expr } => Stmt::expr(self.optimize_expr(expr), stmt.meta),
            StmtKind::Assign { name, value } => {
                Stmt::assign(name, self.optimize_expr(value), stmt.meta)
            }
            StmtKind::Print { expr } => Stmt::print(self.optimize_expr(expr), stmt.meta),
        }
    }

    fn optimize_expr(&mut self, expr: CheckedExpr) -> CheckedExpr {
        use ExprKind::*;

        match expr.kind {
            BinOp { op, left, right } => {
                let new_left = self.optimize_expr(*left);
                let new_right = self.optimize_expr(*right);

                match (&new_left.kind, &new_right.kind) {
                    (Integer(a), Integer(b)) => Expr::integer(op.eval(*a, *b), expr.meta),
                    (Float(a), Float(b)) => Expr::float(op.eval(*a, *b), expr.meta),
                    _ => Expr::bin_op(op, new_left, new_right, expr.meta),
                }
            }
            _ => expr,
        }
    }
}

impl<M> BinOp<M> {
    fn eval<T>(&self, a: T, b: T) -> T
    where
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
    {
        match self.kind {
            BinOpKind::Add => a + b,
            BinOpKind::Sub => a - b,
            BinOpKind::Mul => a * b,
            BinOpKind::Div => a / b,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::pipeline::checker::check;
    use crate::pipeline::optimizer::optimize;
    use crate::pipeline::parser::parse;
    use crate::pipeline::tokenizer::tokenize;
    use crate::states::InputState;

    #[test]
    fn test_checker() {
        let input = InputState::from("print (1 + 4) * 3 / (3 + 2);");
        let tokenized = tokenize(input).unwrap();
        let parsed = parse(tokenized).unwrap();
        let checked = check(parsed).unwrap();
        let optimized = optimize(checked);

        insta::assert_debug_snapshot!(optimized.ast);
    }
}
