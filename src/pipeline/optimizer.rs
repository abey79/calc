use crate::data::ast::{BinOp, BinOpKind, Expr, ExprKind, Stmt, StmtKind};
use crate::errors::OptimizerError;
use crate::states::ParsedInput;
use std::ops::{Add, Div, Mul, Sub};

type Result<T> = std::result::Result<T, OptimizerError>;

pub(crate) fn optimize(input: ParsedInput) -> ParsedInput {
    let optimizer = Optimizer::new(input);
    optimizer.run()
}

struct Optimizer {
    input: ParsedInput,
}

impl Optimizer {
    fn new(input: ParsedInput) -> Self {
        Self { input }
    }

    fn run(mut self) -> ParsedInput {
        let old_stmts: Vec<_> = self.input.ast_ctx.stmts_mut().drain(..).collect();
        old_stmts.into_iter().for_each(|stmt| {
            let new_stmt = self.optimize_stmt(stmt);
            self.input.ast_ctx.push_stmt(new_stmt);
        });

        self.input
    }

    fn optimize_stmt(&mut self, stmt: Stmt) -> Stmt {
        let new_stmt = match stmt.kind {
            StmtKind::Expr { expr } => Stmt::expr(self.optimize_expr(expr)),
            _ => stmt,
        };

        return new_stmt;
    }

    fn optimize_expr(&mut self, expr: Expr) -> Expr {
        use ExprKind::*;

        let id = expr.id;
        let new_expr = match expr.kind {
            BinOp { op, left, right } => {
                let new_left = self.optimize_expr(*left);
                let new_right = self.optimize_expr(*right);

                match (&new_left.kind, &new_right.kind) {
                    (Integer(a), Integer(b)) => Expr::integer(op.eval(*a, *b)),
                    (Float(a), Float(b)) => Expr::float(op.eval(*a, *b)),
                    _ => Expr::bin_op(op, new_left, new_right),
                }
            }
            _ => expr,
        };

        self.input.ast_ctx.copy_span(id, new_expr.id);

        return new_expr;
    }
}

impl BinOp {
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
