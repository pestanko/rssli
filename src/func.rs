use crate::{env::Environment, parser::Value};

pub type FuncType = fn(args: &[Value], fenv: &mut Environment) -> Value;

#[derive(Clone)]
pub enum FuncKind {
    Native {
        metadata: FuncMetadata,
        func: FuncType,
    },
    Defined {
        metadata: FuncMetadata,
        func: FuncDef,
    },
}

#[derive(Clone, Debug)]
pub struct FuncMetadata {
    pub name: String,
    pub same_env: bool,
}

#[derive(Clone, Debug)]
pub struct FuncDef {
    pub func_args: Vec<String>,
    pub func: Value,
}
