use crate::env::Environment;
use crate::parser::Value;

pub(crate) fn register(env: &mut Environment) {
    // env.add_native("map", bi_list_map, false);
    env.add_native("head", bi_list_head, false);
    env.add_native("last", bi_list_last, false);
}

/*fn bi_list_map(args: &[Value], fenv: &mut Environment) -> Value {
    Value::Nil
}*/

fn bi_list_head(args: &[Value], fenv: &mut Environment) -> Value {
    let list = args[0].as_list();
    fenv.eval(&list[0])
}

fn bi_list_last(args: &[Value], fenv: &mut Environment) -> Value {
    let list = args[0].as_list();
    fenv.eval(&list.last().cloned().unwrap())
}
