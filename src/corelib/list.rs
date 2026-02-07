use crate::env::Environment;
use crate::parser::Value;

pub(crate) fn register(env: &mut Environment) {
    // env.add_native("map", bi_list_map, false);
    env.add_native("list", list_create_list, false);
    env.add_native("head", bi_list_head, false);
    env.add_native("last", bi_list_last, false);
    env.add_native("list.seq", list_seq, false);
    env.add_native("list.len", list_len, false);
    env.add_native("list.append", list_append, false);
    env.add_native("list.reverse", list_reverse, false);
    env.add_native("list.filter", list_filter, false);
    env.add_native("list.reduce", list_reduce, false);
    env.add_native("list.map", list_map, false);
}

fn list_create_list(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let mut values: Vec<Value> = Vec::new();
    for x in args {
        let value = fenv.eval(x)?;
        values.push(value);
    }
    Ok(Value::List(values))
}

fn list_filter(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let list = fenv.eval(&args[0])?.as_list();
    let func = fenv.eval(&args[1])?.as_func();

    let mut result = Vec::new();
    let func_value = Value::Func(func);

    for element in &list {
        let call_list = Value::List(vec![func_value.clone(), element.clone()]);
        let predicate_result = fenv.eval(&call_list)?;
        if predicate_result.as_bool() {
            result.push(element.clone());
        }
    }

    Ok(Value::List(result))
}

fn list_reduce(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let list = fenv.eval(&args[0])?.as_list();
    let func = fenv.eval(&args[1])?.as_func();

    if list.is_empty() {
        return Ok(Value::Nil);
    }

    let func_value = Value::Func(func);
    let mut accumulator = list[0].clone();

    for element in list.iter().skip(1) {
        let call_list = Value::List(vec![func_value.clone(), accumulator.clone(), element.clone()]);
        accumulator = fenv.eval(&call_list)?;
    }

    Ok(accumulator)
}

fn list_map(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let list = fenv.eval(&args[0])?.as_list();
    let func = fenv.eval(&args[1])?.as_func();

    let mut result = Vec::new();
    let func_value = Value::Func(func);

    for element in list {
        let call_list = Value::List(vec![func_value.clone(), element]);
        let mapped_value = fenv.eval(&call_list)?;
        result.push(mapped_value);
    }

    Ok(Value::List(result))
}

fn list_len(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let list = fenv.eval(&args[0])?.as_list();
    Ok(Value::Int(list.len() as i64))
}

fn list_append(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let mut list = fenv.eval(&args[0])?.as_list();
    for value in fenv.eval_args(&args[1..])? {
        list.push(value);
    }
    Ok(Value::List(list))
}

fn list_reverse(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let list = fenv.eval(&args[0])?.as_list();
    Ok(Value::List(list.into_iter().rev().collect()))
}

/*fn bi_list_map(args: &[Value], fenv: &mut Environment) -> Value {
    Value::Nil
}*/

fn bi_list_head(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let list = args[0].as_list();
    let first = list.first().ok_or_else(|| anyhow::anyhow!("Cannot get head of empty list"))?;
    fenv.eval(first)
}

fn bi_list_last(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let list = args[0].as_list();
    let last = list.last().ok_or_else(|| anyhow::anyhow!("Cannot get last element of empty list"))?;
    fenv.eval(last)
}

fn list_seq(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value> {
    let start = fenv.eval(&args[0])?.as_int();
    let end = fenv.eval(&args[1])?.as_int();
    let step = if let Some(step_val) = args.get(2) {
        fenv.eval(step_val)?.as_int()
    } else {
        1
    };

    let mut list = Vec::new();
    for i in (start..end).step_by(step as usize) {
        list.push(Value::Int(i));
    }

    Ok(Value::List(list))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::Runtime;

    #[test]
    fn test_list_map() {
        let mut runtime = Runtime::new_default();

        // Define function first, then call map
        runtime.eval_string("(fn inc (x) (+ x 1))").unwrap();
        let result = runtime
            .eval_parsed("(list.map (list.seq 1 5) inc)")
            .unwrap();

        assert_eq!(
            result,
            Value::List(vec![
                Value::Int(2),
                Value::Int(3),
                Value::Int(4),
                Value::Int(5)
            ])
        );

        // Map multiply by 2
        runtime.eval_string("(fn double (x) (* x 2))").unwrap();
        let result = runtime
            .eval_parsed("(list.map (1 2 3) double)")
            .unwrap();

        assert_eq!(
            result,
            Value::List(vec![Value::Int(2), Value::Int(4), Value::Int(6)])
        );

        // Map over empty list
        runtime.eval_string("(fn id (x) x)").unwrap();
        let result = runtime
            .eval_parsed("(list.map () id)")
            .unwrap();

        assert_eq!(result, Value::List(vec![]));
    }

    #[test]
    fn test_list_filter() {
        let mut runtime = Runtime::new_default();

        // Filter even numbers
        runtime.eval_string("(fn is-even (x) (== (% x 2) 0))").unwrap();
        let result = runtime
            .eval_parsed("(list.filter (list.seq 1 10) is-even)")
            .unwrap();

        assert_eq!(
            result,
            Value::List(vec![
                Value::Int(2),
                Value::Int(4),
                Value::Int(6),
                Value::Int(8)
            ])
        );

        // Filter numbers greater than 5
        runtime.eval_string("(fn gt5 (x) (> x 5))").unwrap();
        let result = runtime
            .eval_parsed("(list.filter (1 3 5 7 9) gt5)")
            .unwrap();

        assert_eq!(
            result,
            Value::List(vec![Value::Int(7), Value::Int(9)])
        );

        // Filter over empty list
        runtime.eval_string("(fn always-true (x) true)").unwrap();
        let result = runtime
            .eval_parsed("(list.filter () always-true)")
            .unwrap();

        assert_eq!(result, Value::List(vec![]));

        // Filter with all false (empty result)
        runtime.eval_string("(fn always-false (x) false)").unwrap();
        let result = runtime
            .eval_parsed("(list.filter (1 2 3) always-false)")
            .unwrap();

        assert_eq!(result, Value::List(vec![]));
    }

    #[test]
    fn test_list_reduce() {
        let mut runtime = Runtime::new_default();

        // Sum numbers
        runtime.eval_string("(fn add (acc x) (+ acc x))").unwrap();
        let result = runtime
            .eval_string("(list.reduce (1 2 3 4 5) add)")
            .unwrap();

        assert_eq!(result, Value::Int(15));

        // Multiply numbers
        runtime.eval_string("(fn multiply (acc x) (* acc x))").unwrap();
        let result = runtime
            .eval_string("(list.reduce (2 3 4) multiply)")
            .unwrap();

        assert_eq!(result, Value::Int(24));

        // Reduce single element list
        runtime.eval_string("(fn add (acc x) (+ acc x))").unwrap();
        let result = runtime
            .eval_string("(list.reduce (42) add)")
            .unwrap();

        assert_eq!(result, Value::Int(42));

        // Reduce empty list returns nil
        runtime.eval_string("(fn add (acc x) (+ acc x))").unwrap();
        let result = runtime
            .eval_string("(list.reduce () add)")
            .unwrap();

        assert_eq!(result, Value::Nil);
    }
}
