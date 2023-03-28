pub mod corelib;
pub mod env;
mod func;
pub mod parser;
pub mod runtime;
pub mod tokenizer;
mod utils;


pub use crate::runtime::Runtime;