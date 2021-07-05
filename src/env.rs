use crate::{
    func::{FuncKind, FuncType},
    parser::Value,
    utils::HierCellMapWrap,
};
use std::fmt::{Debug, Formatter};
use std::fmt;
use crate::func::FuncMetadata;

type FuncsType = HierCellMapWrap<String, FuncKind>;
type VarsType = HierCellMapWrap<String, Value>;

#[derive(Clone)]
pub struct Environment {
    pub funcs: FuncsType,
    pub vars: VarsType,
}

impl Debug for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if !self.funcs().curr_is_empty() {
            write!(f, "Funcs: {:?} ; ", self.funcs.keys())?;
        }
        if !self.vars().curr_is_empty() {
            write!(f, "Vars: {:?}", self.vars.data())?;
        }
        Ok(())
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            vars: VarsType::new_root(),
            funcs: FuncsType::new_root(),
        }
    }
}

impl Environment {
    pub fn make_child(&self) -> Self {
        Self {
            funcs: self.funcs.new_child(),
            vars: self.vars.new_child(),
        }
    }

    pub fn funcs(&self) -> &FuncsType {
        &self.funcs
    }

    pub fn vars(&self) -> &VarsType {
        &self.vars
    }

    pub fn eval(&mut self, value: &Value) -> Value {
        match value {
            Value::List(l) => self.eval_list(l),
            Value::Symbol(name) => self.vars.get(name).expect(&format!("Undeclared variable: {}", name)),
            _ => value.clone(),
        }
    }

    pub fn eval_list(&mut self, list: &[Value]) -> Value {
        if list.is_empty() {
            return Value::Nil;
        }

        if list[0].is_symbol() {
            // it is a function - so call it
            let fn_name = list[0].as_string();
            self.eval_func(&fn_name, &list[1..])
        } else {
            let evaluated: Vec<Value> = list.iter().map(|arg| self.eval(arg)).collect();
            Value::List(evaluated)
        }
    }

    pub fn eval_func(&mut self, name: &str, args: &[Value]) -> Value {
        log::debug!("[EVAL] Evaluating function: \"{}\" with args {:?}", name, args);

        let func = self
            .funcs
            .get(&name.to_string())
            .expect(&format!("No function found with name: {}", name));
        let mut new_env = self.make_child();
        let result = match func {
            FuncKind::Native {
                metadata,
                func,
            } => {
                let env = if metadata.same_env { self } else { &mut new_env };
                log::trace!("Calling nat [{}] with env: {:?}", name, env);
                func(args, env)
            }
            FuncKind::Defined {
                metadata, func
            } => {
                let env = if metadata.same_env { self } else { &mut new_env };

                for (i, func_arg) in func.func_args.iter().enumerate() {
                    if i < args.len() {
                        let value = env.eval(&args[i]);
                        env.vars.set(func_arg, &value);
                    }
                }

                log::debug!("Calling def [{}] with env: {:?}", name, env);

                env.eval(&func.func)
            }
        };

        let final_result = if result.is_list() && result.as_list().len() == 1 {
            result.as_list()[0].clone()
        } else {
            result
        };

        log::debug!("Function[{}] result: {:?}", name, final_result);

        final_result
    }

    pub fn eval_args(&mut self, args: &[Value]) -> Vec<Value> {
        args.iter().map(|arg| self.eval(arg)).collect()
    }

    pub fn add_native(&mut self, name: &str, func: FuncType, same_env: bool) {
        self.funcs.set(
            &name.to_string(),
            &FuncKind::Native {
                metadata: FuncMetadata {
                    name: name.to_string(),
                    same_env
                },
                func,
            },
        )
    }
}