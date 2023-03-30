use crate::env::Environment;

mod assert;
mod cast;
mod core;
mod internal;
mod io;
mod list;
mod ops;

pub(crate) fn register(env: &mut Environment) {
    core::register(env);
    io::register(env);
    cast::register(env);
    ops::register(env);
    list::register(env);
    internal::register(env);
    assert::register(env);
}
