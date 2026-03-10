use std::collections::BTreeMap;
use super::ast::*;

#[derive(Debug)]
pub struct ValidateError {
    pub message: String,
}

impl std::fmt::Display for ValidateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ValidateError {}

pub fn validate_program(program: &Program) -> Result<(), ValidateError> {
    let mut funcs: BTreeMap<&str, &Function> = BTreeMap::new();

    for f in &program.functions {
        if funcs.contains_key(f.name.as_str()) {
            return Err(ValidateError { message: format!("duplicate function name: {}", f.name) });
        }
        funcs.insert(f.name.as_str(), f);

        // check duplicate params
        let mut seen = std::collections::BTreeSet::new();
        for p in &f.params {
            if seen.contains(p) {
                return Err(ValidateError { message: format!("duplicate parameter '{}' in function {}", p, f.name) });
            }
            seen.insert(p);
        }
    }

    // main must exist
    if !funcs.contains_key("main") {
        return Err(ValidateError { message: "missing required function: main".to_string() });
    }

    // Validate calls and arity
    for f in &program.functions {
        validate_exprs(&f.body, &funcs)?;
    }

    Ok(())
}

fn validate_exprs(block: &Block, funcs: &BTreeMap<&str, &Function>) -> Result<(), ValidateError> {
    for stmt in &block.statements {
        match stmt {
            Stmt::Return(expr) => validate_expr(expr, funcs)?,
        }
    }
    Ok(())
}

fn validate_expr(expr: &super::ast::Expr, funcs: &BTreeMap<&str, &Function>) -> Result<(), ValidateError> {
    match expr {
        super::ast::Expr::Number(_) => Ok(()),
        super::ast::Expr::Identifier(_) => Ok(()),
        super::ast::Expr::Binary(left, _, right) => {
            validate_expr(left, funcs)?;
            validate_expr(right, funcs)?;
            Ok(())
        }
        super::ast::Expr::Call(name, args) => {
            if let Some(target) = funcs.get(name.as_str()) {
                if args.len() != target.params.len() {
                    return Err(ValidateError { message: format!("Function '{}' expects {} arguments but received {}", name, target.params.len(), args.len()) });
                }
                for a in args {
                    validate_expr(a, funcs)?;
                }
                Ok(())
            } else {
                return Err(ValidateError { message: format!("Unknown function: {}", name) });
            }
        }
    }
}
