use crate::{
    corelib,
    env::Environment,
    parser::Value,
};
use std::path::PathBuf;

#[derive(Default)]
pub struct Runtime {
    env: Environment,
}

impl Runtime {
    pub fn new() -> Self {
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
        self.env.eval_string(prog)
    }

    pub fn eval_file(&mut self, file_path: &str, content: &str) -> anyhow::Result<Value> {
        let path = PathBuf::from(file_path);
        let canonical_path = path.canonicalize()
            .map_err(|e| anyhow::anyhow!("Failed to canonicalize path {}: {}", file_path, e))?;
        self.env.eval_string_with_file(content, Some(&canonical_path), false)
    }

    /// Evaluates a program preserving list results (no unwrapping).
    /// This is useful for REPL where you want to see full list results from functions
    /// like `list.map` and `list.filter`.
    pub fn eval_string_preserve_lists(&mut self, prog: &str) -> anyhow::Result<Value> {
        self.env.eval_string_with_file(prog, None, true)
    }

    /// Evaluates a program without unwrapping list results.
    /// This is useful for testing functions that return lists, as `eval_string`
    /// unwraps list results to their last element (REPL behavior).
    pub fn eval_parsed(&mut self, prog: &str) -> anyhow::Result<Value> {
        use crate::tokenizer::tokenize;
        use crate::parser::parse_tokens;
        
        let tokens = tokenize(prog)?;
        let parsed = parse_tokens(&tokens)?;
        
        // Evaluate all expressions, return last without unwrapping
        let mut result = Value::Nil;
        for expr in &parsed {
            result = self.env.eval(expr)?;
        }
        Ok(result)
    }

    pub fn env(&self) -> &Environment {
        &self.env
    }
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
        assert_eq!(runtime.eval_string("(!= 1 2)").unwrap(), Value::Bool(true));
    }

    #[test]
    fn test_neq_returns_false_for_equal_values() {
        let mut runtime = Runtime::new_default();
        assert_eq!(runtime.eval_string("(!= 1 1)").unwrap(), Value::Bool(false));
    }

    #[test]
    fn test_eq_returns_true_for_equal_values() {
        let mut runtime = Runtime::new_default();
        assert_eq!(runtime.eval_string("(== 5 5)").unwrap(), Value::Bool(true));
    }

    #[test]
    fn test_eq_returns_false_for_unequal_values() {
        let mut runtime = Runtime::new_default();
        assert_eq!(runtime.eval_string("(== 5 3)").unwrap(), Value::Bool(false));
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
        let result = runtime.eval_string(r#"(if (list.seq 1 3) 1 0)"#).unwrap();
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
    fn test_basic_closure_captures_variable() {
        let mut runtime = Runtime::new_default();

        let result = runtime
            .eval_string(
                r#"
        (
           (def n 10)
           (fn add-n (x) (+ x n))
           (add-n 5)
        )
        "#,
            )
            .unwrap();

        assert_eq!(result, Value::Int(15));
    }

    #[test]
    fn test_closure_returning_closure() {
        let mut runtime = Runtime::new_default();

        let result = runtime
            .eval_string(
                r#"
        (
           (fn make-adder (n) (fn (x) (+ n x)))
           (def add5 (make-adder 5))
           (add5 10)
        )
        "#,
            )
            .unwrap();

        assert_eq!(result, Value::Int(15));
    }

    #[test]
    fn test_immediately_invoked_returned_closure() {
        let mut runtime = Runtime::new_default();

        let result = runtime
            .eval_string(
                r#"
        (
           (fn make-adder (n) (fn (x) (+ n x)))
           ((make-adder 5) 10)
        )
        "#,
            )
            .unwrap();

        assert_eq!(result, Value::Int(15));
    }

    #[test]
    fn test_closure_shared_mutable_state() {
        let mut runtime = Runtime::new_default();

        let result = runtime
            .eval_string(
                r#"
        (
           (def counter 0)
           (fn inc () (def counter (+ counter 1)))
           (inc)
           (inc)
           (inc)
           counter
        )
        "#,
            )
            .unwrap();

        assert_eq!(result, Value::Int(3));
    }

    #[test]
    fn test_closure_does_not_see_caller_variables() {
        let mut runtime = Runtime::new_default();

        let result = runtime.eval_string(
            r#"
        (
           (fn func (x) (+ c x))
           (fn func2 (c) (func 5))
           (func2 10)
        )
        "#,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_comments_are_ignored() {
        let mut runtime = Runtime::new_default();

        let result = runtime
            .eval_string(
                r#"
        (
           ; define variables
           (def x 5)   ; x = 5
           (def y 10)  ; y = 10
           ; compute the sum
           (+ x y)
        )
        "#,
            )
            .unwrap();

        assert_eq!(result, Value::Int(15));
    }

    #[test]
    fn test_not_operator() {
        let mut runtime = Runtime::new_default();
        assert_eq!(
            runtime.eval_string("(not true)").unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            runtime.eval_string("(not false)").unwrap(),
            Value::Bool(true)
        );
        assert_eq!(runtime.eval_string("(not 0)").unwrap(), Value::Bool(true));
        assert_eq!(runtime.eval_string("(not 1)").unwrap(), Value::Bool(false));
        assert_eq!(runtime.eval_string("(not nil)").unwrap(), Value::Bool(true));
    }

    #[test]
    fn test_leq_operator() {
        let mut runtime = Runtime::new_default();
        assert_eq!(runtime.eval_string("(<= 1 2)").unwrap(), Value::Bool(true));
        assert_eq!(runtime.eval_string("(<= 2 2)").unwrap(), Value::Bool(true));
        assert_eq!(runtime.eval_string("(<= 3 2)").unwrap(), Value::Bool(false));
        assert_eq!(
            runtime.eval_string("(<= 1.5 2.0)").unwrap(),
            Value::Bool(true)
        );
    }

    #[test]
    fn test_geq_operator() {
        let mut runtime = Runtime::new_default();
        assert_eq!(runtime.eval_string("(>= 3 2)").unwrap(), Value::Bool(true));
        assert_eq!(runtime.eval_string("(>= 2 2)").unwrap(), Value::Bool(true));
        assert_eq!(runtime.eval_string("(>= 1 2)").unwrap(), Value::Bool(false));
        assert_eq!(
            runtime.eval_string("(>= 2.0 1.5)").unwrap(),
            Value::Bool(true)
        );
    }

    #[test]
    fn test_modulo_operator() {
        let mut runtime = Runtime::new_default();
        assert_eq!(runtime.eval_string("(% 10 3)").unwrap(), Value::Int(1));
        assert_eq!(runtime.eval_string("(% 10 5)").unwrap(), Value::Int(0));
        assert_eq!(
            runtime.eval_string("(% 10.5 3.0)").unwrap(),
            Value::Float(1.5)
        );
    }
}
