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

    pub fn eval_string(&mut self, prog: &str) -> Value {
        let tokens = tokenize(prog);
        let parsed = parse_tokens(&tokens);

        let res = if parsed.len() == 1 {
            self.env.eval(parsed.get(0).unwrap())
        } else {
            self.env.eval(&Value::List(parsed))
        };

        if let Value::List(lst) = res {
            lst.last().unwrap().clone()
        } else {
            res
        }
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

        let result = runtime.eval_string(
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
        );

        assert_eq!(
            result,
            Value::String("Ahoj svet".to_string()),
        )
    }

    #[test]
    fn test_simple_variables() {
        let mut runtime = Runtime::new_default();

        let result = runtime.eval_string(
            r#"
        (
           (def x 5)
           (def y 8)
           (def z (+ x y))

           (def x (+ z 9))
           (+ x 20)
        )
        "#,
        );

        assert_eq!(
            result,
            Value::Int(42),
        );
    }

    #[test]
    fn test_factorial_function() {
        let mut runtime = Runtime::new_default();

        let result = runtime.eval_string(
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
        );

        assert_eq!(result, Value::Int(120));
    }

    #[test]
    fn test_fib_function() {
        let mut runtime = Runtime::new_default();

        let result = runtime.eval_string(
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
        );

        assert_eq!(result, Value::Int(144),);
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

        assert_eq!(
            result,
            Value::Int(15),
        );
    }
}
