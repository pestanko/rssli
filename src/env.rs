use crate::func::{FuncDef, FuncMetadata, FuncType};
use crate::{
    func::FuncKind,
    parser::{parse_tokens, Value},
    tokenizer::tokenize,
    utils::HierCellMapWrap,
};
use std::collections::HashSet;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::path::PathBuf;

type FuncsType = HierCellMapWrap<String, FuncDef>;
type VarsType = HierCellMapWrap<String, Value>;

#[derive(Clone)]
pub struct Environment {
    pub funcs: FuncsType,
    pub vars: VarsType,
    importing_files: HashSet<PathBuf>,
    current_file: Option<PathBuf>,
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
            importing_files: HashSet::new(),
            current_file: None,
        }
    }
}

impl Environment {
    pub fn make_child(&self) -> Self {
        Self {
            funcs: self.funcs.new_child(),
            vars: self.vars.new_child(),
            importing_files: self.importing_files.clone(),
            current_file: self.current_file.clone(),
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
                        same_env: false,
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
        } else if list[0].is_list() {
            let all_lists = list.iter().all(|v| v.is_list());
            let first = self.eval(&list[0])?;
            if first.is_func() && !all_lists {
                let df = FuncDef::anonymous(first.as_func());
                self.eval_any_func(df, &list[1..])
            } else {
                let mut result = first;
                for expr in &list[1..] {
                    result = self.eval(expr)?;
                }
                Ok(result)
            }
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

    fn eval_any_func(&mut self, func: FuncDef, args: &[Value]) -> anyhow::Result<Value> {
        let metadata = &func.metadata;
        log::debug!(
            "[EVAL] Function[{}] \"{}\" with args {:?}",
            func.kind_name(),
            func.name(),
            args
        );

        let result = match func.kind {
            FuncKind::Native(native_fn) => {
                let mut new_env = self.make_child();
                let env = if metadata.same_env {
                    self
                } else {
                    &mut new_env
                };
                native_fn(args, env)?
            }
            FuncKind::Closure(func_val, captured_env) => {
                let mut closure_env = captured_env.make_child();
                for (i, param) in func_val.args.iter().enumerate() {
                    if i < args.len() {
                        let value = self.eval(&args[i])?;
                        closure_env.vars.set(param, &value);
                    }
                }
                closure_env.eval(&func_val.body)?
            }
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

    pub fn eval_string(&mut self, prog: &str) -> anyhow::Result<Value> {
        self.eval_string_with_file(prog, None, false)
    }

    pub fn eval_string_with_file(&mut self, prog: &str, file_path: Option<&PathBuf>, preserve_lists: bool) -> anyhow::Result<Value> {
        // Set current file context if provided
        let old_current_file = self.current_file.clone();
        if let Some(path) = file_path {
            self.current_file = Some(path.clone());
        }

        let tokens = tokenize(prog)?;
        let parsed = parse_tokens(&tokens)?;

        let mut final_res = Value::Nil;
        for expr in &parsed {
            final_res = self.eval(expr)?;
        }

        let final_res = if preserve_lists {
            final_res
        } else if let Value::List(lst) = final_res {
            lst.last().cloned().unwrap_or(Value::Nil)
        } else {
            final_res
        };

        // Restore previous current file context
        self.current_file = old_current_file;

        Ok(final_res)
    }

    pub fn import_file(&mut self, file_path: &str) -> anyhow::Result<Value> {
        use std::fs;
        use std::env;

        // Resolve path - start with the path as given
        let mut path = PathBuf::from(file_path);
        
        // If path is relative, resolve relative to current file's directory (if available)
        // Otherwise fall back to current working directory
        if !path.is_absolute() {
            if let Some(ref current_file) = self.current_file {
                // Resolve relative to the directory of the current file
                if let Some(parent_dir) = current_file.parent() {
                    path = parent_dir.join(&path);
                } else {
                    // Current file has no parent (shouldn't happen, but handle gracefully)
                    let current_dir = env::current_dir()
                        .map_err(|e| anyhow::anyhow!("Failed to get current directory: {}", e))?;
                    path = current_dir.join(&path);
                }
            } else {
                // No current file context, use working directory
                let current_dir = env::current_dir()
                    .map_err(|e| anyhow::anyhow!("Failed to get current directory: {}", e))?;
                path = current_dir.join(&path);
            }
        }

        // Try the path as-is first
        if !path.exists() {
            // Try with .lsp extension
            let path_with_ext = path.with_extension("lsp");
            if path_with_ext.exists() {
                path = path_with_ext;
            } else {
                anyhow::bail!("File not found: {} (also tried {})", file_path, path_with_ext.display());
            }
        }

        // Normalize the path to detect circular imports
        let canonical_path = path.canonicalize()
            .map_err(|e| anyhow::anyhow!("Failed to canonicalize path {}: {}", path.display(), e))?;

        // Check for circular import
        if self.importing_files.contains(&canonical_path) {
            anyhow::bail!("Circular import detected: {}", canonical_path.display());
        }

        // Add to importing set
        self.importing_files.insert(canonical_path.clone());

        // Read and evaluate file with the file path context
        let result = match fs::read_to_string(&canonical_path) {
            Ok(content) => {
                let eval_result = self.eval_string_with_file(&content, Some(&canonical_path), false);
                // Remove from importing set (even on error)
                self.importing_files.remove(&canonical_path);
                eval_result?
            }
            Err(e) => {
                // Remove from importing set on error
                self.importing_files.remove(&canonical_path);
                anyhow::bail!("Failed to read file {}: {}", canonical_path.display(), e);
            }
        };

        Ok(result)
    }
}
