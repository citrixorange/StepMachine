use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Expression<T> {
    pub result: Option<T>,
    pub add: Option<T>,
    pub subtract: Option<T>,
    pub multiply: Option<T>,
    pub divide: Option<T>
}

impl<T> Expression<T> {
    pub fn new(add: Option<T>, subtract: Option<T>, multiply: Option<T>, divide: Option<T>) -> Self {
        Self {
            result: None,
            add: add,
            subtract: subtract,
            multiply: multiply,
            divide: divide
        }
    }
}