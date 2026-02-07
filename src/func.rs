use crate::{
    env::Environment,
    parser::{FuncValue, Value},
};

pub type FuncType = fn(args: &[Value], fenv: &mut Environment) -> anyhow::Result<Value>;

#[derive(Clone, Debug)]
pub struct FuncDef {
    pub metadata: FuncMetadata,
    pub kind: FuncKind,
}

impl FuncDef {
    pub fn anonymous(kind: FuncKind) -> Self {
        Self {
            metadata: FuncMetadata {
                name: "anonymous".to_owned(),
                same_env: false,
            },
            kind,
        }
    }

    pub fn kind_name(&self) -> &'static str {
        match &self.kind {
            FuncKind::Native(_) => "native",
            FuncKind::Defined(_) => "defined",
        }
    }

    pub fn name(&self) -> &str {
        &self.metadata.name
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FuncMetadata {
    pub name: String,
    pub same_env: bool,
}

#[derive(Clone)]
pub enum FuncKind {
    Native(FuncType),
    Defined(FuncValue),
}

impl PartialEq for FuncKind {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl PartialOrd for FuncKind {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}

impl std::fmt::Debug for FuncKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FuncKind::Native(_) => write!(f, "(nat fn)"),
            FuncKind::Defined(df) => write!(f, "(def fn {:?}, {:?})", df.args, df.body),
        }
    }
}
