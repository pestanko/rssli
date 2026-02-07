use crate::{env::Environment, parser::Value};

pub(crate) fn register(env: &mut Environment) {
    // Strings
    env.add_native("str.trim", strings_trim, false);
    env.add_native("str.trim_left", strings_trim_left, false);
    env.add_native("str.trim_right", strings_trim_right, false);
    env.add_native("str.to_lower", strings_to_lower, false);
    env.add_native("str.to_upper", strings_to_upper, false);
    env.add_native("str.format", strings_format, false);
    env.add_native("str.split", strings_split, false);
    env.add_native("str.join", strings_join, false);
    env.add_native("str.contains", strings_contains, false);
    env.add_native("str.starts_with", strings_starts_with, false);
    env.add_native("str.ends_with", strings_ends_with, false);
    env.add_native("str.replace", strings_replace, false);
    env.add_native("str.len", strings_len, false);
    env.add_native("char.ord", char_ord, false);
    env.add_native("char.chr", char_chr, false);
}

fn strings_trim(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let arg = fenv.eval(&args[0])?.as_string();
    Ok(Value::String(arg.trim().to_string()))
}

fn strings_trim_left(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let arg = fenv.eval(&args[0])?.as_string();
    Ok(Value::String(arg.trim_start().to_string()))
}

fn strings_trim_right(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let arg = fenv.eval(&args[0])?.as_string();
    Ok(Value::String(arg.trim_end().to_string()))
}

fn strings_to_lower(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let arg = fenv.eval(&args[0])?.as_string();
    Ok(Value::String(arg.to_lowercase()))
}

fn strings_to_upper(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let arg = fenv.eval(&args[0])?.as_string();
    Ok(Value::String(arg.to_uppercase()))
}

fn strings_split(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let arg = fenv.eval(&args[0])?.as_string();
    if args.len() == 1 {
        return Ok(Value::List(arg.split_whitespace().map(|x| Value::String(x.to_string())).collect()));
    }
    let separator = fenv.eval(&args[1])?.as_string();
    Ok(Value::List(
        arg.split(&separator)
            .map(|x| Value::String(x.to_string()))
            .collect(),
    ))
}

fn strings_join(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let list = fenv.eval(&args[0])?.as_list();
    let separator = fenv.eval(&args[1])?.as_string();
    Ok(Value::String(
        list.iter()
            .map(|x| x.as_string())
            .collect::<Vec<_>>()
            .join(&separator),
    ))
}

fn strings_contains(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let arg = fenv.eval(&args[0])?.as_string();
    let substring = fenv.eval(&args[1])?.as_string();
    Ok(Value::Bool(arg.contains(&substring)))
}

fn strings_starts_with(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let arg = fenv.eval(&args[0])?.as_string();
    let substring = fenv.eval(&args[1])?.as_string();
    Ok(Value::Bool(arg.starts_with(&substring)))
}

fn strings_ends_with(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let arg = fenv.eval(&args[0])?.as_string();
    let substring = fenv.eval(&args[1])?.as_string();
    Ok(Value::Bool(arg.ends_with(&substring)))
}

fn strings_replace(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let arg = fenv.eval(&args[0])?.as_string();
    let old = fenv.eval(&args[1])?.as_string();
    let new = fenv.eval(&args[2])?.as_string();
    Ok(Value::String(arg.replace(&old, &new)))
}

fn strings_len(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let arg = fenv.eval(&args[0])?.as_string();
    Ok(Value::Int(arg.len() as i64))
}

pub fn strings_format(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let mut parts = fenv
        .eval_args(args)?
        .iter()
        .map(|x| x.as_string())
        .collect::<Vec<_>>();
    let format = parts.remove(0);
    let mut processed = String::new();
    let mut arg_index = 0;
    let mut chars = format.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '%' {
            if let Some(&next_ch) = chars.peek() {
                if next_ch == '%' {
                    // Handle %% escape sequence - output a single %
                    processed.push('%');
                    chars.next(); // consume the second %
                } else if next_ch == 'v' {
                    // Handle %v format specifier
                    chars.next(); // consume the 'v'
                    if arg_index < parts.len() {
                        processed.push_str(&parts[arg_index]);
                        arg_index += 1;
                    } else {
                        // More %v specifiers than arguments - leave %v as-is
                        processed.push_str("%v");
                    }
                } else {
                    // % followed by something else - preserve it
                    processed.push(ch);
                }
            } else {
                // % at end of string - preserve it
                processed.push(ch);
            }
        } else {
            processed.push(ch);
        }
    }
    Ok(Value::String(processed))
}


// Character operations
fn char_ord(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let arg = fenv.eval(&args[0])?.as_string();
    let char = arg.chars().next().unwrap();
    Ok(Value::Int(char as u64 as i64))
}

fn char_chr(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let arg = fenv.eval(&args[0])?.as_int();
    Ok(Value::String(char::from_u32(arg as u32).unwrap().to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::Runtime;

    #[test]
    fn test_strings_format_single_value() {
        let mut runtime = Runtime::new_default();
        let result = runtime.eval_string(r#"(str.format "x is %v" 10)"#).unwrap();
        assert_eq!(result, Value::String("x is 10".to_string()));
    }

    #[test]
    fn test_strings_format_multiple_values() {
        let mut runtime = Runtime::new_default();
        let result = runtime
            .eval_string(r#"(str.format "x is %v and y is %v" 10 20)"#)
            .unwrap();
        assert_eq!(result, Value::String("x is 10 and y is 20".to_string()));
    }

    #[test]
    fn test_strings_format_percent_escape() {
        let mut runtime = Runtime::new_default();
        let result = runtime
            .eval_string(r#"(str.format "100%% is %v" 100)"#)
            .unwrap();
        assert_eq!(result, Value::String("100% is 100".to_string()));
    }

    #[test]
    fn test_strings_format_with_variable() {
        let mut runtime = Runtime::new_default();
        let result = runtime
            .eval_string(
                r#"
            (
                (def x 42)
                (str.format "x is %v" x)
            )
            "#,
            )
            .unwrap();
        assert_eq!(result, Value::String("x is 42".to_string()));
    }

    #[test]
    fn test_strings_format_more_specifiers_than_args() {
        let mut runtime = Runtime::new_default();
        let result = runtime
            .eval_string(r#"(str.format "x is %v and y is %v" 10)"#)
            .unwrap();
        assert_eq!(result, Value::String("x is 10 and y is %v".to_string()));
    }

    #[test]
    fn test_strings_format_more_args_than_specifiers() {
        let mut runtime = Runtime::new_default();
        let result = runtime
            .eval_string(r#"(io.printf "x is %v" 10 20 30)"#)
            .unwrap();
        assert_eq!(result, Value::String("x is 10".to_string()));
    }

    #[test]
    fn test_strings_format_no_specifiers() {
        let mut runtime = Runtime::new_default();
        let result = runtime
            .eval_string(r#"(str.format "hello world")"#)
            .unwrap();
        assert_eq!(result, Value::String("hello world".to_string()));
    }

    #[test]
    fn test_strings_format_percent_at_end() {
        let mut runtime = Runtime::new_default();
        let result = runtime
            .eval_string(r#"(str.format "x is %v%" 10)"#)
            .unwrap();
        assert_eq!(result, Value::String("x is 10%".to_string()));
    }

    #[test]
    fn test_strings_format_multiple_percent_escapes() {
        let mut runtime = Runtime::new_default();
        let result = runtime
            .eval_string(r#"(str.format "50%% and 100%%" 10)"#)
            .unwrap();
        assert_eq!(result, Value::String("50% and 100%".to_string()));
    }

    #[test]
    fn test_char_ord() {
        let mut runtime = Runtime::new_default();
        let result = runtime.eval_string(r#"(char.ord "a")"#).unwrap();
        assert_eq!(result, Value::Int(97));
        assert_eq!(runtime.eval_string(r#"(char.ord "A")"#).unwrap(), Value::Int(65));
        assert_eq!(runtime.eval_string(r#"(char.ord "z")"#).unwrap(), Value::Int(122));
        assert_eq!(runtime.eval_string(r#"(char.ord "Z")"#).unwrap(), Value::Int(90));
        assert_eq!(runtime.eval_string(r#"(char.ord "0")"#).unwrap(), Value::Int(48));
        assert_eq!(runtime.eval_string(r#"(char.ord "9")"#).unwrap(), Value::Int(57));
        assert_eq!(runtime.eval_string(r#"(char.ord " ")"#).unwrap(), Value::Int(32));
        assert_eq!(runtime.eval_string(r#"(char.ord "!")"#).unwrap(), Value::Int(33));
    }

    #[test]
    fn test_char_chr() {
        let mut runtime = Runtime::new_default();
        let result = runtime.eval_string(r#"(char.chr 97)"#).unwrap();
        assert_eq!(result, Value::String("a".to_string()));
        assert_eq!(runtime.eval_string(r#"(char.chr (char.ord "a"))"#).unwrap(), Value::String("a".to_string()));
        assert_eq!(runtime.eval_string(r#"(char.chr (char.ord "b"))"#).unwrap(), Value::String("b".to_string()));
        assert_eq!(runtime.eval_string(r#"(char.chr (char.ord "c"))"#).unwrap(), Value::String("c".to_string()));
        assert_eq!(runtime.eval_string(r#"(char.chr (char.ord "d"))"#).unwrap(), Value::String("d".to_string()));
        assert_eq!(runtime.eval_string(r#"(char.chr (char.ord "e"))"#).unwrap(), Value::String("e".to_string()));
        assert_eq!(runtime.eval_string(r#"(char.chr (char.ord "f"))"#).unwrap(), Value::String("f".to_string()));
        assert_eq!(runtime.eval_string(r#"(char.chr (char.ord "g"))"#).unwrap(), Value::String("g".to_string()));
        assert_eq!(runtime.eval_string(r#"(char.chr (char.ord "h"))"#).unwrap(), Value::String("h".to_string()));
    }
}
