use std::sync::Once;

use crate::{
    corelib,
    env::Environment,
    parser::{parse_tokens, Value},
    tokenizer::tokenize,
};

#[derive(Default)]
pub struct Runtime {
    env: Environment,
}

impl Runtime {
    pub fn new() -> Self {
        setup_logger();
        Self {
            env: Environment::default(),
        }
    }

    pub fn new_default() -> Self {
        let mut run = Self::new();

        corelib::register(&mut run.env);
        run
    }

    pub fn eval_string(&mut self, prog: &str) -> anyhow::Result<Value> {
        let tokens = tokenize(prog)?;
        let parsed = parse_tokens(&tokens)?;

        let res = if parsed.len() == 1 {
            self.env.eval(parsed.first().unwrap())
        } else {
            self.env.eval(&Value::List(parsed))
        };

        let final_res = if let Value::List(lst) = res {
            lst.last().cloned().unwrap()
        } else {
            res
        };
        Ok(final_res)
    }
}

static INIT: Once = Once::new();

fn setup_logger() {
    INIT.call_once(env_logger::init);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_prog_simple_ops() {
        let mut runtime = Runtime::new_default();

        let result = runtime
            .eval_string(
                r#"
        (
            (+ 5 15)
            (- 15 2)
            (* 7 3)
            (/ 8 4)
            (+ 5.5 2)
            (+ "Ahoj" " svet")
        )
        "#,
            )
            .unwrap();

        assert_eq!(result, Value::String("Ahoj svet".to_string()),)
    }

    #[test]
    fn test_simple_variables() {
        let mut runtime = Runtime::new_default();

        let result = runtime
            .eval_string(
                r#"
        (
           (def x 5)
           (def y 8)
           (def z (+ x y))

           (def x (+ z 9))
           (+ x 20)
        )
        "#,
            )
            .unwrap();

        assert_eq!(result, Value::Int(42),);
    }

    #[test]
    fn test_factorial_function() {
        let mut runtime = Runtime::new_default();

        let result = runtime
            .eval_string(
                r#"
           (
           (fn inc (y)
                (+ y 1)
           )

           (fn dec (z)
                (- z 1)
           )
           (fn factorial (x)
                (if (< x 1)
                    1
                    (* x (factorial (dec x)))
                )
           )
           (factorial 5)
        )
        "#,
            )
            .unwrap();

        assert_eq!(result, Value::Int(120));
    }

    #[test]
    fn test_fib_function() {
        let mut runtime = Runtime::new_default();

        let result = runtime
            .eval_string(
                r#"
        (
           (fn fib (x)
                (if (< x 3)
                    1
                    (+
                        (fib (- x 1))
                        (fib (- x 2))
                    )
                )
            )
           (fib 12)
        )
        "#,
            )
            .unwrap();

        assert_eq!(result, Value::Int(144),);
    }

    #[test]
    fn test_neq_returns_true_for_unequal_values() {
        let mut runtime = Runtime::new_default();
        assert_eq!(
            runtime.eval_string("(!= 1 2)").unwrap(),
            Value::Bool(true)
        );
    }

    #[test]
    fn test_neq_returns_false_for_equal_values() {
        let mut runtime = Runtime::new_default();
        assert_eq!(
            runtime.eval_string("(!= 1 1)").unwrap(),
            Value::Bool(false)
        );
    }

    #[test]
    fn test_eq_returns_true_for_equal_values() {
        let mut runtime = Runtime::new_default();
        assert_eq!(
            runtime.eval_string("(== 5 5)").unwrap(),
            Value::Bool(true)
        );
    }

    #[test]
    fn test_eq_returns_false_for_unequal_values() {
        let mut runtime = Runtime::new_default();
        assert_eq!(
            runtime.eval_string("(== 5 3)").unwrap(),
            Value::Bool(false)
        );
    }

    #[test]
    fn test_eq_evaluates_first_arg_once() {
        let mut runtime = Runtime::new_default();
        // If == double-evaluates args[0], counter would be incremented
        // twice (once for fst, once in loop), yielding 2 != 1.
        let result = runtime
            .eval_string(
                r#"
            (
                (def counter 0)
                (== (def counter (+ counter 1)) 1)
            )
            "#,
            )
            .unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_while_skips_body_when_condition_initially_false() {
        let mut runtime = Runtime::new_default();
        let result = runtime
            .eval_string(
                r#"
            (
                (def x 0)
                (while false (def x (+ x 1)))
                x
            )
            "#,
            )
            .unwrap();
        assert_eq!(result, Value::Int(0));
    }

    #[test]
    fn test_while_returns_nil() {
        let mut runtime = Runtime::new_default();
        let result = runtime.eval_string("(while false 1)").unwrap();
        assert_eq!(result, Value::Nil);
    }

    #[test]
    fn test_nonempty_list_is_truthy() {
        let mut runtime = Runtime::new_default();
        let result = runtime
            .eval_string(r#"(if (list.seq 1 3) 1 0)"#)
            .unwrap();
        assert_eq!(result, Value::Int(1));
    }

    #[test]
    fn test_truthiness_values() {
        // Verify Symbol/List truthiness via boolean coercion
        assert!(Value::Symbol("x".to_string()).as_bool());
        assert!(!Value::Symbol("".to_string()).as_bool());
        assert!(Value::List(vec![Value::Int(1)]).as_bool());
        assert!(!Value::List(vec![]).as_bool());
    }

    #[test]
    fn test_not_nice_function() {
        let mut runtime = Runtime::new_default();

        let result = runtime.eval_string(
            r#"
        (
           (fn func (x) (+ c x))
           (fn func2(c) (func 5))
           (func2 10)
        )
        "#,
        );

        assert_eq!(result.unwrap(), Value::Int(15),);
    }
}
