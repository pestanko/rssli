use crate::{env::Environment, parser::Value};

pub(crate) fn register(env: &mut Environment) -> () {
    // IO
    env.add_native("cast.string", cast_string, false);
    env.add_native("cast.int", cast_int, false);
    env.add_native("cast.float", cast_float, false);
    env.add_native("cast.bool", cast_bool, false);
    env.add_native("cast.list", cast_list, false);
}

fn cast_string(args: &[Value], fenv: &mut Environment) -> Value {
    let arg = fenv.eval(&args[0]);
    Value::String(arg.to_string())
}

fn cast_int(args: &[Value], fenv: &mut Environment) -> Value {
    let arg = fenv.eval(&args[0]);
    Value::Int(arg.as_int())
}

fn cast_float(args: &[Value], fenv: &mut Environment) -> Value {
    let arg = fenv.eval(&args[0]);
    Value::Float(arg.as_float())
}

fn cast_bool(args: &[Value], fenv: &mut Environment) -> Value {
    let arg = fenv.eval(&args[0]);
    Value::Bool(arg.as_bool())
}

fn cast_list(args: &[Value], fenv: &mut Environment) -> Value {
    let arg = fenv.eval(&args[0]);
    Value::List(arg.as_list())
}