use crate::context::checked_ast::{CheckedBinOp, CheckedExpr, CheckedStmt, CheckedUnaryOp, Type};
use crate::data::ast::{BinOpKind, ExprKind, StmtKind, UnaryOpKind};
use crate::states::CheckedState;
use std::collections::BTreeMap;
use std::fmt;

pub(crate) fn llvm_codegen<W: fmt::Write>(input: &CheckedState, writer: &mut W) -> fmt::Result {
    let mut codegen = LlvmCodegen::new(input, writer);
    codegen.run()
}

#[derive(Debug, Clone)]
enum LlvmType {
    Builtin(Type),
    // more types here, e.g. tuple
}

impl LlvmType {
    pub fn init_val(&self) -> &'static str {
        match self {
            Self::Builtin(Type::Float) => "0.0",
            Self::Builtin(Type::Integer) => "0",
            Self::Builtin(Type::Stmt) => unreachable!(),
        }
    }
}

impl fmt::Display for LlvmType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LlvmType::Builtin(type_) => match type_ {
                Type::Integer => write!(f, "i32"),
                Type::Float => write!(f, "double"),
                Type::Stmt => unreachable!(),
            },
        }
    }
}

struct LlvmValue {
    // TODO: the representation of the value must be changed to an enum for tuple
    pub register: String,
    pub type_: LlvmType,
}

impl LlvmValue {
    fn new(register: String, type_: LlvmType) -> Self {
        Self { register, type_ }
    }
}

struct LlvmCodegen<'a, W: fmt::Write> {
    input: &'a CheckedState,
    writer: &'a mut W,

    // state
    code: Vec<String>,
    globals: BTreeMap<String, LlvmType>,
    id: usize,
}

impl<'a, W: fmt::Write> LlvmCodegen<'a, W> {
    fn new(input: &'a CheckedState, writer: &'a mut W) -> Self {
        Self {
            input,
            writer,
            code: Vec::new(),
            globals: BTreeMap::new(),
            id: 0,
        }
    }

    fn out<S: Into<String>>(&mut self, s: S) {
        self.code.push(s.into());
    }

    fn next_id(&mut self) -> usize {
        let id = self.id;
        self.id += 1;
        id
    }

    fn next_reg(&mut self) -> String {
        format!("%r{}", self.next_id())
    }

    fn run(&mut self) -> fmt::Result {
        // generate code for each statement
        for stmt in self.input.ast.stmts() {
            self.codegen_stmt(stmt)?;
        }

        // write output
        writeln!(self.writer, "declare void @_print_int(i32 %x)")?;
        writeln!(self.writer, "declare void @_print_float(double %x)")?;
        writeln!(self.writer)?;

        // declare global variables
        for (name, ltype) in &self.globals {
            writeln!(
                self.writer,
                "@{} = global {} {}",
                name,
                ltype,
                ltype.init_val()
            )?;
        }

        writeln!(self.writer)?;
        writeln!(self.writer, "define void @calc_main() {{")?;

        for line in &self.code {
            writeln!(self.writer, "    {line}")?;
        }

        writeln!(self.writer, "    ret void")?;
        writeln!(self.writer, "}}")?;

        Ok(())
    }

    fn codegen_stmt(&mut self, stmt: &CheckedStmt) -> fmt::Result {
        match &stmt.kind {
            StmtKind::Assign { name, value } => {
                let llvm_value = self.codegen_expr(value)?;
                self.out(format!(
                    "store {} {}, {}* @{}",
                    llvm_value.type_, llvm_value.register, llvm_value.type_, name
                ));
                self.globals.insert(name.to_string(), llvm_value.type_);
            }
            StmtKind::Print { expr } => {
                let llvm_value = self.codegen_expr(expr)?;

                let func = match expr.meta.type_ {
                    Type::Stmt => unreachable!("expression cannot have Stmt type"),
                    Type::Integer => "_print_int",
                    Type::Float => "_print_float",
                };

                self.out(format!(
                    "call void @{}({} {})",
                    func, llvm_value.type_, llvm_value.register
                ));
            }
            StmtKind::Expr { expr } => {
                // pointless since no possibly side effects
                self.codegen_expr(expr)?;
            }
        }
        Ok(())
    }

    fn codegen_expr(&mut self, expr: &CheckedExpr) -> Result<LlvmValue, fmt::Error> {
        match &expr.kind {
            ExprKind::Variable(name) => self.codegen_variable(name.as_ref()),
            ExprKind::UnaryOp { op, operand } => self.codegen_unary_op(op, operand),
            ExprKind::BinOp { op, left, right } => self.codegen_bin_op(op, left, right),
            ExprKind::Tuple(..) => todo!(),
            ExprKind::Integer(i) => Ok(LlvmValue::new(
                i.to_string(),
                LlvmType::Builtin(Type::Integer),
            )),
            ExprKind::Float(f) => Ok(LlvmValue::new(
                format!("{:?}", f),
                LlvmType::Builtin(Type::Float),
            )),
        }
    }

    fn codegen_variable(&mut self, name: &str) -> Result<LlvmValue, fmt::Error> {
        let reg = self.next_reg();
        let type_ = self
            .globals
            .get(name)
            .expect("type checker should have checked this")
            .clone();

        self.out(format!("{0} = load {1}, {1}* @{2}", reg, type_, name));
        Ok(LlvmValue::new(reg, type_))
    }

    fn codegen_unary_op(
        &mut self,
        op: &CheckedUnaryOp,
        operand: &CheckedExpr,
    ) -> Result<LlvmValue, fmt::Error> {
        let operand = self.codegen_expr(operand)?;

        //TODO: should match on operand.type_ when it's properly supported
        match operand.type_ {
            LlvmType::Builtin(type_) => self.codegen_unary_op_builtin(&type_, op.kind, operand),
        }
    }

    fn codegen_unary_op_builtin(
        &mut self,
        type_: &Type,
        op: UnaryOpKind,
        value: LlvmValue,
    ) -> Result<LlvmValue, fmt::Error> {
        match op {
            UnaryOpKind::Neg => {
                let reg = self.next_reg();

                let (opcode, cst) = match type_ {
                    Type::Integer => ("sub", "0"),
                    Type::Float => ("fsub", "0.0"),
                    Type::Stmt => unreachable!(),
                };

                self.out(format!(
                    "{} = {} {} {}, {}",
                    reg, opcode, value.type_, cst, value.register
                ));

                Ok(LlvmValue::new(reg, value.type_))
            }
            UnaryOpKind::Pos => Ok(value),
        }
    }

    fn codegen_bin_op(
        &mut self,
        op: &CheckedBinOp,
        left: &CheckedExpr,
        right: &CheckedExpr,
    ) -> Result<LlvmValue, fmt::Error> {
        let left = self.codegen_expr(left)?;
        let right = self.codegen_expr(right)?;

        //TODO: should match on operand.type_ when it's properly supported
        match left.type_ {
            LlvmType::Builtin(type_) => self.codegen_bin_op_builtin(&type_, op.kind, left, right),
        }
    }

    fn codegen_bin_op_builtin(
        &mut self,
        type_: &Type,
        op: BinOpKind,
        left: LlvmValue,
        right: LlvmValue,
    ) -> Result<LlvmValue, fmt::Error> {
        let opcode = match type_ {
            Type::Integer => match op {
                BinOpKind::Add => "add",
                BinOpKind::Sub => "sub",
                BinOpKind::Mul => "mul",
                BinOpKind::Div => "sdiv",
            },
            Type::Float => match op {
                BinOpKind::Add => "fadd",
                BinOpKind::Sub => "fsub",
                BinOpKind::Mul => "fmul",
                BinOpKind::Div => "fdiv",
            },
            Type::Stmt => unreachable!(),
        };

        let reg = self.next_reg();

        self.out(format!(
            "{} = {} {} {}, {}",
            reg, opcode, left.type_, left.register, right.register
        ));

        Ok(LlvmValue::new(reg, left.type_))
    }
}

#[cfg(test)]
mod test {
    use crate::pipeline::checker::check;
    use crate::pipeline::llvm::llvm_codegen;
    use crate::pipeline::parser::parse;
    use crate::pipeline::tokenizer::tokenize;
    use crate::states::InputState;

    #[test]
    fn test_llvm_codegen() {
        let input = InputState::from("a = (1.3 + 3.2) * 45.1; b = a * 3.2; print 1 + 2 * 3;");
        let tokenized = tokenize(input).unwrap();
        let parsed = parse(tokenized).unwrap();
        let checked = check(parsed).unwrap();

        let mut output = String::new();
        llvm_codegen(&checked, &mut output).unwrap();

        insta::assert_snapshot!(output);
    }
}
