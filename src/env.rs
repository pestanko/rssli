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

    pub fn eval(&mut self, value: &Value) -> anyhow::Result<Value> {
        log::debug!("[EVAL] Expression: {:?}", value);
        let res = match value {
            Value::List(l) => self.eval_list(l)?,
            Value::Symbol(name) => self.get_var_or_func(name)?,
            Value::Func(fx) => self.eval_any_func(FuncDef::anonymous(fx.clone()), &[])?,
            _ => value.clone(),
        };

        log::debug!("[EVAL] Expression result: {:?} => {:?}", value, res);
        Ok(res)
    }

    pub fn get_var_or_func(&self, name: &str) -> anyhow::Result<Value> {
        let name_ref = &name.to_string();
        if let Some(val) = self.vars.get(name_ref) {
            return Ok(val);
        }
        if let Some(val) = self.funcs.get(name_ref) {
            return Ok(Value::Func(val.kind));
        }
        anyhow::bail!("Undeclared variable or function: {}", name)
    }

    pub fn get_func_def(&self, name: &str) -> anyhow::Result<FuncDef> {
        let name_ref = &name.to_string();
        if let Some(val) = self.funcs.get(name_ref) {
            return Ok(val);
        }

        if let Some(val) = self.vars.get(name_ref) {
            if val.is_func() {
                return Ok(FuncDef {
                    metadata: FuncMetadata {
                        name: name.to_owned(),
                        same_env: true,
                    },
                    kind: val.as_func(),
                });
            }
        }

        anyhow::bail!("Undeclared function: {}", name)
    }

    pub fn eval_list(&mut self, list: &[Value]) -> anyhow::Result<Value> {
        log::debug!("[EVAL] List: {:?}", list);
        if list.is_empty() {
            return Ok(Value::Nil);
        }

        if list[0].is_symbol() {
            // it is a function - so call it
            let fn_name = list[0].as_string();
            self.eval_func(&fn_name, &list[1..])
        } else if list[0].is_func() {
            let df = FuncDef::anonymous(list[0].as_func());
            self.eval_any_func(df, &list[1..])
        } else {
            let mut evaluated = Vec::new();
            for arg in list {
                evaluated.push(self.eval(arg)?);
            }
            Ok(Value::List(evaluated))
        }
    }

    pub fn eval_func(&mut self, name: &str, args: &[Value]) -> anyhow::Result<Value> {
        let fd = self.get_func_def(name)?;
        self.eval_any_func(fd, args)
    }

    fn eval_def_func(&mut self, func: FuncValue, args: &[Value]) -> anyhow::Result<Value> {
        for (i, func_arg) in func.args.iter().enumerate() {
            if i < args.len() {
                let value = self.eval(&args[i])?;
                self.vars.set(func_arg, &value);
            }
        }

        let res = self.eval(&func.body)?;
        log::debug!(">>> [EVAL] Function call result: {}", res);
        Ok(res)
    }

    fn eval_any_func(&mut self, func: FuncDef, args: &[Value]) -> anyhow::Result<Value> {
        let mut new_env = self.make_child();
        let metadata = &func.metadata;
        log::debug!(
            "[EVAL] Function[{}] \"{}\" with args {:?}",
            func.kind_name(),
            func.name(),
            args
        );

        let env = if metadata.same_env {
            self
        } else {
            &mut new_env
        };

        let result = match func.kind {
            FuncKind::Native(func) => func(args, env)?,
            FuncKind::Defined(func) => env.eval_def_func(func, args)?,
        };

        let final_result = if result.is_list() && result.as_list().len() == 1 {
            result.as_list()[0].clone()
        } else {
            result
        };

        log::debug!(
            ">>> [EVAL] Function call result[{}]: {:?}",
            metadata.name,
            final_result
        );

        Ok(final_result)
    }

    pub fn eval_args(&mut self, args: &[Value]) -> anyhow::Result<Vec<Value>> {
        log::debug!("[EVAL] args: {:?}", args);
        let mut evaluated = Vec::new();
        for arg in args {
            evaluated.push(self.eval(arg)?);
        }
        Ok(evaluated)
    }

    pub fn add_native(&mut self, name: &str, func: FuncType, same_env: bool) {
        let metadata = FuncMetadata {
            name: name.to_string(),
            same_env,
        };
        log::debug!("[ADD] native function: {:?}", metadata);
        let df = FuncDef {
            metadata,
            kind: FuncKind::Native(func),
        };

        self.funcs.set(&name.to_string(), &df)
    }
}
