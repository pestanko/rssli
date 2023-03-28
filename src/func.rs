use crate::{env::Environment, parser::{Value, FuncValue}};

pub type FuncType = fn(args: &[Value], fenv: &mut Environment) -> Value;

#[derive(Clone)]
pub enum FuncKind {
    Native {
        metadata: FuncMetadata,
        func: FuncType,
    },
    Defined {
        metadata: FuncMetadata,
        func: FuncValue,
    },
}

#[derive(Clone, Debug)]
pub struct FuncMetadata {
    pub name: String,
    pub same_env: bool,
}