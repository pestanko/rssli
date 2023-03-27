use crate::{env::Environment, parser::Value};

pub(crate) fn register(env: &mut Environment) -> () {
    // internal
    env.add_native("internal.func.nat.call", bi_internal_func_nat_call, true);
    env.add_native("internal.func.list", bi_internal_func_list, true);
}


fn bi_internal_func_nat_call(args: &[Value], fenv: &mut Environment) -> Value {
    fenv.eval_func(&&args[0].as_string(), &args[1..])
}

fn bi_internal_func_list(_args: &[Value], fenv: &mut Environment) -> Value {
    for k in fenv.funcs.keys() {
        println!("Function: {}", k);
    }

    Value::Nil
}