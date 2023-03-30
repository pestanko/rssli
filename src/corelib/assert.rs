use crate::{env::Environment, parser::Value};

pub(crate) fn register(env: &mut Environment) {
    // internal
    env.add_native("assert", assert_cond, true);
    env.add_native("assert.eq", assert_eq, true);
}

fn assert_cond(args: &[Value], fenv: &mut Environment) -> Value {
    let cond = fenv.eval(&args[0]);
    if !cond.as_bool() {
        panic!("Condition validation failed for assert({cond}): {:?}", args)
    } else {
        log::info!("assert({cond}) passed");
    }
    Value::Nil
}

fn assert_eq(args: &[Value], fenv: &mut Environment) -> Value {
    let fst = fenv.eval(&args[0]);
    let snd = fenv.eval(&args[1]);
    if fst != snd {
        panic!(
            "Condition validation failed for assert({fst} == {snd}): {:?}",
            args
        )
    } else {
        log::info!("assert({fst} == {snd}) passed");
    }
    Value::Nil
}
