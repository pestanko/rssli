use crate::func::{FuncDef, FuncMetadata};
use crate::{env::Environment, func::FuncKind, parser::Value};

pub(crate) fn register(env: &mut Environment) -> () {
    env.add_native("fn", bi_func_def, true);
    env.add_native("def", bi_setvar, true);
    env.add_native("undef", bi_unsetvar, true);
    env.add_native("if", bi_cond_if, true);

}

// Core
fn bi_func_def(args: &[Value], fenv: &mut Environment) -> Value {
    let name = args[0].as_string();
    log::info!("Defining function: {:?}", name);
    let func_args: Vec<_> = args
        .get(1)
        .unwrap()
        .as_list()
        .iter()
        .map(|x| x.as_string())
        .collect();
    let body = args[2].clone();

    fenv.funcs.set(
        &name,
        &FuncKind::Defined {
            metadata: FuncMetadata {
                name: name.to_string(),
                same_env: false,
            },
            func: FuncDef {
                func_args,
                func: body,
            },
        },
    );
    Value::Symbol(name)
}

fn bi_setvar(args: &[Value], fenv: &mut Environment) -> Value {
    let nameval = &args[0];

    if !nameval.is_symbol() && nameval.is_string() {
        log::error!("Unable to set variable name is not string or symbol");
        return Value::Nil; // error
    }

    let name = nameval.as_string();
    let value = fenv.eval(&args[1]);
    log::trace!("Setting variable: {} to {}", name, value);
    fenv.vars.set(&name, &value);

    value
}

fn bi_unsetvar(args: &[Value], fenv: &mut Environment) -> Value {
    let nameval = &args[0];

    if !nameval.is_symbol() && nameval.is_string() {
        log::error!("Unable to set variable name is not string or symbol");
        return Value::Nil; // error
    }

    let name = nameval.as_string();

    fenv.vars.unset(&name);
    Value::Nil
}

fn bi_cond_if(args: &[Value], fenv: &mut Environment) -> Value {
    let cond = fenv.eval(&args[0]).as_bool();
    if cond {
        fenv.eval(&args[1])
    } else if let Some(else_branch) = args.get(2) {
        fenv.eval(else_branch)
    } else {
        Value::Nil
    }
}
