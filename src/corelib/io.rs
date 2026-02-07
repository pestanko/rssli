use std::io::{self, Write};

use crate::{corelib::strings::strings_format, env::Environment, parser::Value};

pub(crate) fn register(env: &mut Environment) {
    // IO
    env.add_native("print", bi_print, false);
    env.add_native("io.print", bi_print, false);
    env.add_native("io.printf", bi_io_printf, false);
    env.add_native("io.readline", bi_io_readline, false);
    env.add_native("log.debug", bi_log_debug, false);
    env.add_native("log.info", bi_log_info, false);
    env.add_native("log.warn", bi_log_warn, false);
    env.add_native("log.error", bi_log_error, false);
}

fn bi_print(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let parts: Vec<_> = fenv
        .eval_args(args)?
        .iter()
        .map(|x| x.as_string())
        .collect();
    let join = parts.join(" ");
    log::trace!("Print: {}", join);
    println!("{}", join);
    Ok(Value::String(join))
}

fn bi_io_printf(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let processed: Value = strings_format(args, fenv)?;
    print!("{}", processed.as_string());
    io::stdout().flush()?;
    Ok(processed)
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
    let parts: Vec<_> = fenv
        .eval_args(args)?
        .iter()
        .map(|x| x.as_string())
        .collect();
    let join = parts.join(" ");
    log::debug!("Debug: {}", join);
    Ok(Value::String(join))
}

fn bi_log_info(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let parts: Vec<_> = fenv
        .eval_args(args)?
        .iter()
        .map(|x| x.as_string())
        .collect();
    let join = parts.join(" ");
    log::info!("Info: {}", join);
    Ok(Value::String(join))
}

fn bi_log_warn(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let parts: Vec<_> = fenv
        .eval_args(args)?
        .iter()
        .map(|x| x.as_string())
        .collect();
    let join = parts.join(" ");
    log::warn!("Warn: {}", join);
    Ok(Value::String(join))
}

fn bi_log_error(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let parts: Vec<_> = fenv
        .eval_args(args)?
        .iter()
        .map(|x| x.as_string())
        .collect();
    let join = parts.join(" ");
    log::error!("Error: {}", join);
    Ok(Value::String(join))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::Runtime;

    #[test]
    fn test_printf_single_value() {
        let mut runtime = Runtime::new_default();
        let result = runtime.eval_string(r#"(io.printf "x is %v" 10)"#).unwrap();
        assert_eq!(result, Value::String("x is 10".to_string()));
    }

    #[test]
    fn test_printf_multiple_values() {
        let mut runtime = Runtime::new_default();
        let result = runtime
            .eval_string(r#"(io.printf "x is %v and y is %v" 10 20)"#)
            .unwrap();
        assert_eq!(result, Value::String("x is 10 and y is 20".to_string()));
    }

    #[test]
    fn test_printf_percent_escape() {
        let mut runtime = Runtime::new_default();
        let result = runtime
            .eval_string(r#"(io.printf "100%% is %v" 100)"#)
            .unwrap();
        assert_eq!(result, Value::String("100% is 100".to_string()));
    }

    #[test]
    fn test_printf_with_variable() {
        let mut runtime = Runtime::new_default();
        let result = runtime
            .eval_string(
                r#"
            (
                (def x 42)
                (io.printf "x is %v" x)
            )
            "#,
            )
            .unwrap();
        assert_eq!(result, Value::String("x is 42".to_string()));
    }

    #[test]
    fn test_printf_more_specifiers_than_args() {
        let mut runtime = Runtime::new_default();
        let result = runtime
            .eval_string(r#"(io.printf "x is %v and y is %v" 10)"#)
            .unwrap();
        assert_eq!(result, Value::String("x is 10 and y is %v".to_string()));
    }

    #[test]
    fn test_printf_more_args_than_specifiers() {
        let mut runtime = Runtime::new_default();
        let result = runtime
            .eval_string(r#"(io.printf "x is %v" 10 20 30)"#)
            .unwrap();
        assert_eq!(result, Value::String("x is 10".to_string()));
    }

    #[test]
    fn test_printf_no_specifiers() {
        let mut runtime = Runtime::new_default();
        let result = runtime.eval_string(r#"(io.printf "hello world")"#).unwrap();
        assert_eq!(result, Value::String("hello world".to_string()));
    }

    #[test]
    fn test_printf_percent_at_end() {
        let mut runtime = Runtime::new_default();
        let result = runtime.eval_string(r#"(io.printf "x is %v%" 10)"#).unwrap();
        assert_eq!(result, Value::String("x is 10%".to_string()));
    }

    #[test]
    fn test_printf_multiple_percent_escapes() {
        let mut runtime = Runtime::new_default();
        let result = runtime
            .eval_string(r#"(io.printf "50%% and 100%%" 10)"#)
            .unwrap();
        assert_eq!(result, Value::String("50% and 100%".to_string()));
    }
}
