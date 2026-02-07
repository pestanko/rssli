use rand::Rng;

use crate::{env::Environment, parser::Value};


pub(crate) fn register(env: &mut Environment) {
    env.add_native("rnd.int", random_int, false);
}

/**
 * Usage: (rnd.int [min] [max])
 * If no min is given, it will default to 0
 * If no max is given, it will default to 100
 * It will return a random integer between min and max
 */
fn random_int(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let min = if let Some(min_val) = args.get(0) {
        fenv.eval(min_val)?.as_int()
    } else {
        0
    };
    let max = if let Some(max_val) = args.get(1) {
        fenv.eval(max_val)?.as_int()
    } else {
        100
    };
    Ok(Value::Int(rand::rng().random_range(min..=max)))
}
