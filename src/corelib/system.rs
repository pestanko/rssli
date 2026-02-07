use crate::{env::Environment, parser::Value};
use std::fmt;

/// Custom error type for program exit
#[derive(Debug, Clone)]
pub struct ProgramExitError {
    pub code: i32,
}

impl fmt::Display for ProgramExitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Program exited with code {}", self.code)
    }
}

impl std::error::Error for ProgramExitError {}

pub(crate) fn register(env: &mut Environment) {
    env.add_native("exit", exit_with_code, false);
    env.add_native("import", bi_import, true);
}

fn exit_with_code(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let code = if let Some(code_arg) = args.get(0) {
        fenv.eval(code_arg)?.as_int() as i32
    } else {
        0
    };
    Err(ProgramExitError { code }.into())
}

fn bi_import(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    if args.is_empty() {
        anyhow::bail!("import requires a file path argument");
    }

    let path_str = fenv.eval(&args[0])?.as_string();
    log::info!("Importing file: {}", path_str);

    fenv.import_file(&path_str)
}
