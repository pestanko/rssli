use crate::func::{FuncDef, FuncMetadata, FuncType};
use crate::{
    func::FuncKind,
    parser::{FuncValue, Value},
    utils::HierCellMapWrap,
};
use std::fmt;
use std::fmt::{Debug, Formatter};

type FuncsType = HierCellMapWrap<String, FuncDef>;
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
            Value::Symbol(name) => self.get_var_or_func(name),
            Value::Func(fx) => self.eval_any_func(
                FuncDef {
                    metadata: FuncMetadata {
                        name: "anonymous".to_owned(),
                        same_env: true,
                    },
                    kind: fx.clone(),
                },
                &[],
            ),
            _ => value.clone(),
        }
    }

    pub fn get_var_or_func(&self, name: &str) -> Value {
        let name_ref = &name.to_string();
        log::debug!("Vars: {:?}", self.vars.keys());
        if let Some(val) = self.vars.get(name_ref) {
            return val.clone();
        }
        if let Some(val) = self.funcs.get(name_ref) {
            return Value::Func(val.kind.clone());
        } else {
            panic!("Undeclared variable or function: {}", name);
        }
    }

    pub fn eval_symbol(&self, name: &String) -> Value {
        let val = self
            .vars
            .get(name)
            .expect(&format!("Undeclared variable: {}", name));
        log::debug!("Evaluating symbol: {} to: {:?}", name, val);
        val
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
        log::debug!(
            "[EVAL] Evaluating function: \"{}\" with args {:?}",
            name,
            args
        );

        let func = self.get_var_or_func(name);
        if func.is_func() {
            let fd = FuncDef {
                metadata: FuncMetadata {
                    name: name.to_owned(),
                    same_env: true,
                },
                kind: func.as_func(),
            };
            self.eval_any_func(fd, args)
        } else {
            panic!("Symbol {} is not a function", name);
        }
    }

    fn eval_def_func(&mut self, func: FuncValue, args: &[Value]) -> Value {
        for (i, func_arg) in func.args.iter().enumerate() {
            if i < args.len() {
                let value = self.eval(&args[i]);
                self.vars.set(func_arg, &value);
            }
        }

        log::debug!("Calling function with env: {:?}", self);

        self.eval(&func.body)
    }

    fn eval_any_func(&mut self, func: FuncDef, args: &[Value]) -> Value {
        let mut new_env = self.make_child();
        let metadata = &func.metadata;

        let env = if metadata.same_env {
            self
        } else {
            &mut new_env
        };

        let result = match func.kind {
            FuncKind::Native(func) => {
                log::trace!("Calling nat [{}] with env: {:?}", metadata.name, env);
                func(args, env)
            }
            FuncKind::Defined(func) => {
                log::debug!("Calling def [{}] with env: {:?}", metadata.name, env);
                env.eval_def_func(func.clone(), args)
            }
        };

        let final_result = if result.is_list() && result.as_list().len() == 1 {
            result.as_list()[0].clone()
        } else {
            result
        };

        log::debug!("Function call result: {:?}", final_result);

        final_result
    }
    pub fn eval_args(&mut self, args: &[Value]) -> Vec<Value> {
        args.iter().map(|arg| self.eval(arg)).collect()
    }

    pub fn add_native(&mut self, name: &str, func: FuncType, same_env: bool) {
        let metadata = FuncMetadata {
            name: name.to_string(),
            same_env,
        };
        log::debug!("Adding native function: {:?}", metadata);
        let df = FuncDef {
            metadata: metadata,
            kind: FuncKind::Native(func),
        };

        self.funcs.set(&name.to_string(), &df)
    }
}
