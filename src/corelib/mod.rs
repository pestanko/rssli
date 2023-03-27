use crate::env::Environment;

mod core;
mod ops;
mod list;
mod io;
mod cast;
mod internal;

pub(crate) fn register(env: &mut Environment) {
    core::register(env);
    io::register(env);
    cast::register(env);
    ops::register(env);
    list::register(env);
    internal::register(env);
}