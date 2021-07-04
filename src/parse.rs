pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Symbol(String),
    List(Vec<Value>),
    Nil,
}

pub fn parse_tokens(tokens: &[String]) -> Vec<Value> {
    let (_, val) = parse_tokens_from(tokens, 0);
    val
}

fn parse_tokens_from(tokens: &[String], from: usize) -> (usize, Vec<Value>) {
    let mut values: Vec<Value> = Vec::new();

    let mut pos = from;
    while pos < tokens.len() {
        let token = tokens.get(pos).unwrap();
        if token == "nil" {
            values.push(Value::Nil);
        } else if token.starts_with("\"") {
            values.push(Value::String(token[1..].to_string()));
        } else if token == "(" {
            // recursive call
            let (np, vals) = parse_tokens_from(tokens, pos + 1);
            values.push(Value::List(vals));
            pos = np - 1;
        } else if token == ")" {
            return (pos + 1, values);
        } else if let Ok(number) = token.parse::<i64>() {
            values.push(Value::Int(number));
        } else {
            values.push(Value::Symbol(token.to_string()));
        }

        pos += 1;
    }

    (pos, values)
}


#[cfg(test)]
mod tests {
    
}