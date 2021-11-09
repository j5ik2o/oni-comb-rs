use std::collections::HashMap;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Value {
    Int(i64),
    // Float(f64),
    Bool(bool),
    String(String),
    Array(Vec<Value>),
    // Object(HashMap<String, Value>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // println!("display: {:?}", self);
        match self {
            Value::Int(i) => write!(f, "{}", i),
            // Value::Float(f) => write!(f, "{}", f),
            Value::Bool(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Array(a) => write!(f, "[{}]", a.iter().map(|e|e.to_string()).collect::<Vec<_>>().join(",")),
            // Value::Object(o) => write!(f, "{}", o),
        }
    }
}

impl Value {
    pub fn as_int(&self) -> i64 {
        match self {
            Value::Int(i) => *i,
            _ => panic!("Value is not an integer"),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            _ => panic!("Value is not a boolean"),
        }
    }

    pub fn as_string(&self) -> String {
       match self {
           Value::String(s) => s.clone(),
           _ => panic!("Value is not a string"),
       }
    }

}