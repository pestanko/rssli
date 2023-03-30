use crate::{env::Environment, parser::Value};

pub(crate) fn register(env: &mut Environment) {
    // Arithmetic
    env.add_native("+", bi_add, false);
    env.add_native("-", bi_sub, false);
    env.add_native("*", bi_mul, false);
    env.add_native("/", bi_div, false);

    // logical

    env.add_native("==", bi_cmp_eq, false);
    env.add_native("!=", bi_cmp_neq, false);
    env.add_native("!=", bi_cmp_neq, false);
    env.add_native("<", bi_cmp_less, false);
    env.add_native(">", bi_cmp_great, false);
    env.add_native("&&", bi_land, false);
    env.add_native("||", bi_lor, false);
}

// Arithmetic

pub fn bi_add(args: &[Value], fenv: &mut Environment) -> Value {
    let evl = fenv.eval_args(args);
    let mut resolved_type = 'i';
    for i in &evl {
        if i.is_string() {
            resolved_type = 's';
            break;
        }
        if i.is_float() {
            resolved_type = 'f';
        }
    }

    match resolved_type {
        's' => Value::String(
            evl.iter()
                .map(|x| x.as_string())
                .collect::<Vec<_>>()
                .concat(),
        ),
        'f' => Value::Float(evl.iter().map(|x| x.as_float()).sum()),
        _ => Value::Int(evl.iter().map(|x| x.as_int()).sum()),
    }
}

pub fn bi_mul(args: &[Value], fenv: &mut Environment) -> Value {
    let evl = fenv.eval_args(args);
    let mut resolved_type = 'i';

    for i in &evl {
        if i.is_float() {
            resolved_type = 'f';
            break;
        }
    }

    match resolved_type {
        'f' => Value::Float(
            evl.iter()
                .map(|x| x.as_float())
                .reduce(|a, b| a * b)
                .unwrap(),
        ),
        _ => Value::Int(evl.iter().map(|x| x.as_int()).reduce(|a, b| a * b).unwrap()),
    }
}

pub fn bi_sub(args: &[Value], fenv: &mut Environment) -> Value {
    let evl = fenv.eval_args(args);
    log::debug!("Subtracting: {:?}", args);

    let mut resolved_type = 'i';

    for i in &evl {
        if i.is_float() {
            resolved_type = 'f';
            break;
        }
    }

    match resolved_type {
        'f' => Value::Float(
            evl.iter()
                .map(|x| x.as_float())
                .reduce(|a, b| a - b)
                .unwrap(),
        ),
        _ => Value::Int(evl.iter().map(|x| x.as_int()).reduce(|a, b| a - b).unwrap()),
    }
}

pub fn bi_div(args: &[Value], fenv: &mut Environment) -> Value {
    let evl = fenv.eval_args(args);
    let mut resolved_type = 'i';

    for i in &evl {
        if i.is_float() {
            resolved_type = 'f';
            break;
        }
    }

    match resolved_type {
        'f' => Value::Float(
            evl.iter()
                .map(|x| x.as_float())
                .reduce(|a, b| a / b)
                .unwrap(),
        ),
        _ => Value::Int(evl.iter().map(|x| x.as_int()).reduce(|a, b| a / b).unwrap()),
    }
}

pub fn bi_cmp_eq(args: &[Value], fenv: &mut Environment) -> Value {
    let fst = fenv.eval(&args[0]);

    for cond in args.iter() {
        let evl = fenv.eval(cond);
        if fst != evl {
            return Value::Bool(false);
        }
    }
    Value::Bool(true)
}

pub fn bi_cmp_neq(args: &[Value], fenv: &mut Environment) -> Value {
    let fst = fenv.eval(&args[0]);

    for cond in args.iter() {
        let evl = fenv.eval(cond);
        if fst == evl {
            return Value::Bool(false);
        }
    }
    Value::Bool(true)
}

pub fn bi_cmp_less(args: &[Value], fenv: &mut Environment) -> Value {
    let fst = fenv.eval(&args[0]);

    for cond in args.iter().skip(1) {
        let evl = fenv.eval(cond);
        if fst >= evl {
            return Value::Bool(false);
        }
    }
    Value::Bool(true)
}

pub fn bi_cmp_great(args: &[Value], fenv: &mut Environment) -> Value {
    let fst = fenv.eval(&args[0]);

    for cond in args.iter().skip(1) {
        let evl = fenv.eval(cond);
        if fst <= evl {
            return Value::Bool(false);
        }
    }
    Value::Bool(true)
}

pub fn bi_land(args: &[Value], fenv: &mut Environment) -> Value {
    for cond in args.iter() {
        if !fenv.eval(cond).as_bool() {
            return Value::Bool(false);
        }
    }
    Value::Bool(true)
}

pub fn bi_lor(args: &[Value], fenv: &mut Environment) -> Value {
    for cond in args.iter() {
        if fenv.eval(cond).as_bool() {
            return Value::Bool(true);
        }
    }
    Value::Bool(false)
}
