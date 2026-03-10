use anyhow::Result;
use std::collections::BTreeMap;

use super::ast::*;

use wasm_encoder::{
    CodeSection, ExportSection, Function, FunctionSection, Instruction, Module, TypeSection,
    ValType,
};

pub fn compile_program(program: &Program) -> Result<Vec<u8>> {
    let mut types = TypeSection::new();

    // emit all function types (params -> [i32], result -> i32)
    for func in &program.functions {
        let param_types: Vec<ValType> = vec![ValType::I32; func.params.len()];
        let result_types: Vec<ValType> = vec![ValType::I32];
        types.function(param_types.clone(), result_types.clone());
    }

    let mut functions = FunctionSection::new();
    for i in 0..program.functions.len() {
        functions.function(i as u32);
    }

    // function index mapping (no imports -> function indices start at 0)
    let mut func_index: BTreeMap<String, u32> = BTreeMap::new();
    for (i, f) in program.functions.iter().enumerate() {
        func_index.insert(f.name.clone(), i as u32);
    }

    let mut exports = ExportSection::new();
    // export the function named `main` as the module entrypoint
    if let Some(main_idx) = func_index.get("main") {
        exports.export("main", wasm_encoder::ExportKind::Func, *main_idx);
    }

    let mut code = CodeSection::new();

    for f in program.functions.iter() {
        let mut function = Function::new(vec![]);

        // Only support a single return statement per function for v0.0.1
        if let Some(stmt) = f.body.statements.get(0) {
            match stmt {
                Stmt::Return(expr) => {
                    emit_expr(expr, &mut function, &func_index, &f.params)?;
                    function.instruction(&Instruction::Return);
                }
            }
        } else {
            // default return 0
            function.instruction(&Instruction::I32Const(0));
            function.instruction(&Instruction::Return);
        }

        function.instruction(&Instruction::End);
        code.function(&function);
    }

    let mut module = Module::new();
    module.section(&types);
    module.section(&functions);
    module.section(&exports);
    module.section(&code);

    let bytes = module.finish();
    Ok(bytes)
}

fn emit_expr(
    expr: &Expr,
    func: &mut Function,
    func_index: &BTreeMap<String, u32>,
    params: &Vec<String>,
) -> Result<()> {
    match expr {
        Expr::Number(n) => {
            func.instruction(&Instruction::I32Const(*n));
        }
        Expr::Identifier(name) => {
            // parameter lookup
            if let Some(pos) = params.iter().position(|p| p == name) {
                func.instruction(&Instruction::LocalGet(pos as u32));
            } else {
                // unknown identifier; push 0
                func.instruction(&Instruction::I32Const(0));
            }
        }
        Expr::Binary(left, op, right) => {
            emit_expr(left, func, func_index, params)?;
            emit_expr(right, func, func_index, params)?;
            match op {
                BinOp::Add => func.instruction(&Instruction::I32Add),
                BinOp::Sub => func.instruction(&Instruction::I32Sub),
                BinOp::Mul => func.instruction(&Instruction::I32Mul),
                BinOp::Div => func.instruction(&Instruction::I32DivS),
            };
        }
        Expr::Call(name, args) => {
            // push args
            for arg in args {
                emit_expr(arg, func, func_index, params)?;
            }
            if let Some(&target) = func_index.get(name) {
                func.instruction(&Instruction::Call(target));
            } else {
                // unknown call -> push 0
                func.instruction(&Instruction::I32Const(0));
            }
        }
    }
    Ok(())
}
