use std::io;

use crate::{env::Environment, parser::Value};

pub(crate) fn register(env: &mut Environment) -> () {
    // IO
    env.add_native("print", bi_print, false);
    env.add_native("io.print", bi_print, false);
    env.add_native("io.readline", bi_io_readline, false);
}


fn bi_print(args: &[Value], fenv: &mut Environment) -> Value {
    let parts: Vec<_> = fenv.eval_args(args).iter().map(|x| x.as_string()).collect();
    let join = parts.join(" ");
    log::trace!("Print: {}", join);
    println!("{}", join);
    Value::Nil
}

fn bi_io_readline(args: &[Value], fenv: &mut Environment) -> Value {
    if let Some(prompt) = args.get(0).map(|x| fenv.eval(x).as_string()) {
        print!("{} ", prompt);
    }
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let val = Value::String(buffer.trim().to_owned());
    val
}
