use crate::{env::Environment, parser::Value};


pub(crate) fn register(env: &mut Environment) {
    env.add_native("exit", exit_with_code, false);
    env.add_native("import", bi_import, true);
}

fn exit_with_code(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    if let Some(code) = args.get(0) {
        let code = fenv.eval(code)?.as_int();
        std::process::exit(code as i32);
    } else {
        std::process::exit(0);
    }
}

fn bi_import(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    if args.is_empty() {
        anyhow::bail!("import requires a file path argument");
    }

    let path_str = fenv.eval(&args[0])?.as_string();
    log::info!("Importing file: {}", path_str);

    fenv.import_file(&path_str)
}
