use crate::{env::Environment, parser::Value};

pub(crate) fn register(env: &mut Environment) {
    // internal
    env.add_native("internal.func.nat.call", bi_internal_func_nat_call, true);
    env.add_native("internal.func.list", bi_internal_func_list, true);
    env.add_native("internal.printenv", internal_print_env, true);
}

fn bi_internal_func_nat_call(args: &[Value], fenv: &mut Environment) -> Value {
    fenv.eval_func(&args[0].as_string(), &args[1..])
}

fn bi_internal_func_list(_args: &[Value], fenv: &mut Environment) -> Value {
    for k in fenv.funcs.keys() {
        println!("Function: {}", k);
    }

    Value::Nil
}

fn internal_print_env(_args: &[Value], fenv: &mut Environment) -> Value {
    for k in fenv.funcs.keys() {
        println!("fn {}: {:?}", k, fenv.funcs.get(&k).unwrap());
    }

    for v in fenv.vars.keys() {
        println!("var {}: {:?}", v, fenv.vars.get(&v).unwrap());
    }

    Value::Nil
}
