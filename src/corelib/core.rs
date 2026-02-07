use crate::func::{FuncDef, FuncMetadata};
use crate::parser::FuncValue;
use crate::{env::Environment, func::FuncKind, parser::Value};

pub(crate) fn register(env: &mut Environment) {
    env.add_native("fn", bi_func_def, true);
    env.add_native("def", bi_setvar, true);
    env.add_native("undef", bi_unsetvar, true);
    env.add_native("if", bi_cond_if, true);
    env.add_native("while", cycle_while, true);
    env.add_native("for", cycle_for, true);
}

// Core
fn bi_func_def(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let (name, start_from) = if args[0].is_list() {
        ("anonymous".to_owned(), 0)
    } else {
        (args[0].as_string(), 1)
    };

    log::info!("Defining function: {:?}", name);
    let args_list = args
        .get(start_from)
        .ok_or_else(|| anyhow::anyhow!("Function definition missing arguments list"))?
        .as_list();
    let func_args: Vec<_> = args_list
        .iter()
        .map(|x| x.as_string())
        .collect();

    let body = args
        .get(start_from + 1)
        .ok_or_else(|| anyhow::anyhow!("Function definition missing body"))?
        .clone();
    let func = FuncValue {
        args: func_args,
        body: Box::new(body),
    };

    let kind = FuncKind::Closure(func, fenv.clone());

    if name == "anonymous" {
        return Ok(Value::Func(kind));
    }

    let df = FuncDef {
        metadata: FuncMetadata {
            name: name.to_string(),
            same_env: false,
        },
        kind: kind.clone(),
    };

    fenv.funcs.set(&name, &df);
    Ok(Value::Func(kind))
}

fn bi_setvar(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let nameval = &args[0];

    if !nameval.is_symbol() && nameval.is_string() {
        log::error!("Unable to set variable name is not string or symbol");
        return Ok(Value::Nil); // error
    }

    let name = nameval.as_string();
    let value = if let Some(value) = args.get(1) {
        fenv.eval(value)?
    } else {
        Value::Nil
    };
    log::trace!("Setting variable: {} to {}", name, value);
    fenv.vars.set_or_update(&name, &value);

    Ok(value)
}

fn bi_unsetvar(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let nameval = &args[0];

    if !nameval.is_symbol() && nameval.is_string() {
        log::error!("Unable to set variable name is not string or symbol");
        return Ok(Value::Nil); // error
    }

    let name = nameval.as_string();

    fenv.vars.unset(&name);
    Ok(Value::Nil)
}

fn bi_cond_if(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let cond = fenv.eval(&args[0])?.as_bool();
    if cond {
        fenv.eval(&args[1])
    } else if let Some(else_branch) = args.get(2) {
        fenv.eval(else_branch)
    } else {
        Ok(Value::Nil)
    }
}

fn cycle_while(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    while fenv.eval(&args[0])?.as_bool() {
        fenv.eval(&args[1])?;
    }
    Ok(Value::Nil)
}

/**
 * (for (i) (list.seq 1 10) (body) )
 */
fn cycle_for(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let it = args[0].as_string();
    let seq = fenv.eval(&args[1])?.as_list();
    for i in &seq {
        fenv.vars.set(&it, i);
        fenv.eval(&args[2])?;
    }

    Ok(Value::Nil)
}
