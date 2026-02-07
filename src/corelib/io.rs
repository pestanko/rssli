use std::io;

use crate::{env::Environment, parser::Value};

pub(crate) fn register(env: &mut Environment) {
    // IO
    env.add_native("print", bi_print, false);
    env.add_native("io.print", bi_print, false);
    env.add_native("io.readline", bi_io_readline, false);
    env.add_native("log.debug", bi_log_debug, false);
    env.add_native("log.info", bi_log_info, false);
    env.add_native("log.warn", bi_log_warn, false);
    env.add_native("log.error", bi_log_error, false);
}

fn bi_print(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let parts: Vec<_> = fenv.eval_args(args)?.iter().map(|x| x.as_string()).collect();
    let join = parts.join(" ");
    log::trace!("Print: {}", join);
    println!("{}", join);
    Ok(Value::String(join))
}

fn bi_io_readline(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    if let Some(prompt) = args.first() {
        let prompt_val = fenv.eval(prompt)?;
        print!("{} ", prompt_val.as_string());
    }
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(Value::String(buffer.trim().to_owned()))
}

// Logging

fn bi_log_debug(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let parts: Vec<_> = fenv.eval_args(args)?.iter().map(|x| x.as_string()).collect();
    let join = parts.join(" ");
    log::debug!("Debug: {}", join);
    Ok(Value::String(join))
}

fn bi_log_info(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let parts: Vec<_> = fenv.eval_args(args)?.iter().map(|x| x.as_string()).collect();
    let join = parts.join(" ");
    log::info!("Info: {}", join);
    Ok(Value::String(join))
}

fn bi_log_warn(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let parts: Vec<_> = fenv.eval_args(args)?.iter().map(|x| x.as_string()).collect();
    let join = parts.join(" ");
    log::warn!("Warn: {}", join);
    Ok(Value::String(join))
}

fn bi_log_error(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let parts: Vec<_> = fenv.eval_args(args)?.iter().map(|x| x.as_string()).collect();
    let join = parts.join(" ");
    log::error!("Error: {}", join);
    Ok(Value::String(join))
}
