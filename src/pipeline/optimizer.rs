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

impl Optimizer {
    fn new(input: CheckedState) -> Self {
        Self { input }
    }

    fn run(mut self) -> CheckedState {
        let old_stmts: Vec<_> = self.input.checked_ast.stmts_mut().drain(..).collect();
        old_stmts.into_iter().for_each(|stmt| {
            let new_stmt = self.optimize_stmt(stmt);
            self.input.checked_ast.push_stmt(new_stmt);
        });

        self.input
    }

    fn optimize_stmt(&mut self, stmt: CheckedStmt) -> CheckedStmt {
        match stmt.kind {
            StmtKind::Expr { expr } => Stmt::expr(self.optimize_expr(expr), stmt.meta),
            _ => stmt,
        }
    }

    fn optimize_expr(&mut self, expr: CheckedExpr) -> CheckedExpr {
        use ExprKind::*;

        let new_expr = match expr.kind {
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
        };

        new_expr
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
