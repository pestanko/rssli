use crate::{env::Environment, parser::Value};

pub(crate) fn register(env: &mut Environment) {
    // internal
    env.add_native("assert", assert_cond, true);
    env.add_native("assert.eq", assert_eq, true);
}

fn assert_cond(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let cond = fenv.eval(&args[0])?;
    if !cond.as_bool() {
        anyhow::bail!("Condition validation failed for assert({cond}): {:?}", args)
    } else {
        log::info!("assert({cond}) passed");
    }
    Ok(Value::Nil)
}

fn assert_eq(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let fst = fenv.eval(&args[0])?;
    let snd = fenv.eval(&args[1])?;
    if fst != snd {
        anyhow::bail!(
            "Condition validation failed for assert({fst} == {snd}): {:?}",
            args
        )
    } else {
        log::info!("assert({fst} == {snd}) passed");
    }
    Ok(Value::Nil)
}
