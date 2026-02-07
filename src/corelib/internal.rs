use crate::{env::Environment, parser::Value};

pub(crate) fn register(env: &mut Environment) {
    // internal
    env.add_native("internal.func.nat.call", bi_internal_func_nat_call, true);
    env.add_native("internal.func.list", bi_internal_func_list, true);
    env.add_native("internal.printenv", internal_print_env, true);
}

fn bi_internal_func_nat_call(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    fenv.eval_func(&args[0].as_string(), &args[1..])
}

fn bi_internal_func_list(_args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    for k in fenv.funcs.keys() {
        println!("Function: {}", k);
    }
    Ok(Value::Nil)
}

fn internal_print_env(_args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    for k in fenv.funcs.keys() {
        if let Some(func_def) = fenv.funcs.get(&k) {
            println!("fn {}: {:?}", k, func_def);
        }
    }

    for v in fenv.vars.keys() {
        if let Some(var_val) = fenv.vars.get(&v) {
            println!("var {}: {:?}", v, var_val);
        }
    }

    Ok(Value::Nil)
}
