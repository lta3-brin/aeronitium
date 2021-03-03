use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleMessage<T> {
    code: u8,
    code_message: String,
    kind: u8,
    kind_message: String,
    data: T
}

impl<U> SimpleMessage<U> {
    pub fn new(
        code: u8,
        code_message: String,
        types: u8,
        type_message: String,
        data: U
    ) -> Self {
        Self {
            code,
            code_message,
            kind: types,
            kind_message: type_message,
            data
        }
    }
}
