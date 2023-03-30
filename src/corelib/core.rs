use crate::func::{FuncDef, FuncMetadata};
use crate::parser::FuncValue;
use crate::{env::Environment, func::FuncKind, parser::Value};

pub(crate) fn register(env: &mut Environment) {
    env.add_native("fn", bi_func_def, true);
    env.add_native("def", bi_setvar, true);
    env.add_native("undef", bi_unsetvar, true);
    env.add_native("if", bi_cond_if, true);
}

// Core
fn bi_func_def(args: &[Value], fenv: &mut Environment) -> Value {
    let (name, start_from) = if args[0].is_list() {
        ("anonymous".to_owned(), 0)
    } else {
        (args[0].as_string(), 1)
    };

    log::info!("Defining function: {:?}", name);
    let func_args: Vec<_> = args
        .get(start_from)
        .unwrap()
        .as_list()
        .iter()
        .map(|x| x.as_string())
        .collect();

    let func = FuncValue {
        args: func_args,
        body: Box::new(args[start_from + 1].clone()),
    };
    let kind = FuncKind::Defined(func);

    if name == "anonymous" {
        return Value::Func(kind);
    }

    let df = FuncDef {
        metadata: FuncMetadata {
            name: name.to_string(),
            same_env: false,
        },
        kind: kind.clone(),
    };

    fenv.funcs.set(&name, &df);
    Value::Func(kind)
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
