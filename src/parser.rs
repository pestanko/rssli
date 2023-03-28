use std::fmt::Display;

use crate::func::FuncKind;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FuncValue {
    pub args: Vec<String>,
    pub body: Box<Value>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Symbol(String),
    List(Vec<Value>),
    Bool(bool),
    Func(FuncKind),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(x) => write!(f, "{}", x),
            Value::Float(x) => write!(f, "{}", x),
            Value::String(x) => write!(f, "{}", x),
            Value::Symbol(x) => write!(f, "{}", x),
            Value::List(x) => write!(
                f,
                "({})",
                x.iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Value::Bool(x) => write!(f, "{}", x),
            Value::Func(x) => write!(f, "{:?}", x),
            Value::Nil => write!(f, "nil"),
        }
    }
}

impl Into<String> for &Value {
    fn into(self) -> String {
        self.to_string()
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        return Value::Bool(value);
    }
}

impl Into<bool> for &Value {
    fn into(self) -> bool {
        match self {
            Value::Int(n) => *n != 0,
            Value::Float(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Symbol(s) => s.is_empty(),
            Value::List(v) => v.is_empty(),
            Value::Bool(v) => *v,
            Value::Func(_) => true,
            Value::Nil => false,
        }
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        return Value::Int(value);
    }
}

impl Into<i64> for &Value {
    fn into(self) -> i64 {
        match self {
            Value::Int(x) => *x,
            Value::Float(f) => *f as i64,
            Value::String(s) => s.parse::<i64>().unwrap(),
            Value::Symbol(_) => 0,
            Value::List(_) => 0,
            Value::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            Value::Func(_) => 0,
            Value::Nil => 0,
        }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
}

impl Into<f64> for &Value {
    fn into(self) -> f64 {
        match self {
            Value::Int(x) => *x as f64,
            Value::Float(f) => *f,
            Value::String(s) => s.parse::<f64>().unwrap(),
            Value::Symbol(_) => 0.0,
            Value::List(l) => l.len() as f64,
            Value::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            Value::Func(_) => 0.0,
            Value::Nil => 0.0,
        }
    }
}

impl From<Vec<Value>> for Value {
    fn from(value: Vec<Value>) -> Self {
        Self::List(value)
    }
}

impl Into<Vec<Value>> for &Value {
    fn into(self) -> Vec<Value> {
        match self {
            Value::List(l) => l.clone(),
            _ => vec![self.clone()],
        }
    }
}

impl Value {
    pub fn as_bool(&self) -> bool {
        self.into()
    }

    pub fn as_string(&self) -> String {
        format!("{}", self)
    }

    pub fn as_int(&self) -> i64 {
        self.into()
    }

    pub fn as_float(&self) -> f64 {
        self.into()
    }

    pub fn as_list(&self) -> Vec<Value> {
        self.into()
    }

    pub fn as_func(&self) -> FuncKind {
        match self {
            Value::Func(f) => f.clone(),
            _ => panic!("Value is not a function"),
        }
    }

    pub fn is_symbol(&self) -> bool {
        match self {
            Value::Symbol(_) => true,
            _ => false,
        }
    }

    pub fn is_nil(&self) -> bool {
        match self {
            Value::Nil => true,
            _ => false,
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            Value::List(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Value::String(_) => true,
            _ => false,
        }
    }

    pub fn is_int(&self) -> bool {
        match self {
            Value::Int(_) => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            Value::Float(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Value::Bool(_) => true,
            _ => false,
        }
    }

    pub fn is_func(&self) -> bool {
        match self {
            Value::Func(_) => true,
            _ => false,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "integer",
            Value::Float(_) => "float",
            Value::String(_) => "string",
            Value::Symbol(_) => "symbol",
            Value::List(_) => "list",
            Value::Bool(_) => "bool",
            Value::Func(_) => "function",
            Value::Nil => "nil",
        }
    }
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
        } else if token == "true" {
            values.push(Value::Bool(true));
        } else if token == "false" {
            values.push(Value::Bool(false));
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
        } else if let Ok(number) = token.parse::<f64>() {
            values.push(Value::Float(number));
        } else if token.starts_with("0x") {
            let without_prefix = token.trim_start_matches("0x");
            values.push(Value::Int(i64::from_str_radix(without_prefix, 16).unwrap()));
        } else if token.starts_with("0b") {
            let without_prefix = token.trim_start_matches("0b");
            values.push(Value::Int(i64::from_str_radix(without_prefix, 2).unwrap()));
        } else {
            values.push(Value::Symbol(token.to_string()));
        }

        pos += 1;
    }

    (pos, values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_tokens() {
        assert_eq!(parse_tokens(&["true".to_string()]), vec![Value::Bool(true)]);
        assert_eq!(
            parse_tokens(&["false".to_string()]),
            vec![Value::Bool(false)]
        );
        assert_eq!(parse_tokens(&["nil".to_string()]), vec![Value::Nil]);
        assert_eq!(parse_tokens(&["158".to_string()]), vec![Value::Int(158)]);
        assert_eq!(
            parse_tokens(&["0x158".to_string()]),
            vec![Value::Int(0x158)]
        );
        assert_eq!(parse_tokens(&["0b100".to_string()]), vec![Value::Int(4)]);
        assert_eq!(parse_tokens(&["-158".to_string()]), vec![Value::Int(-158)]);
        assert_eq!(
            parse_tokens(&["158.0".to_string()]),
            vec![Value::Float(158.0)]
        );
        assert_eq!(
            parse_tokens(&["158.5".to_string()]),
            vec![Value::Float(158.5)]
        );
    }
}
