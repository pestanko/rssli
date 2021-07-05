use crate::env::Environment;

mod core;
mod ops;
mod list;

pub(crate) fn register(env: &mut Environment) {
    core::register(env);
    ops::register(env);
    list::register(env);
}