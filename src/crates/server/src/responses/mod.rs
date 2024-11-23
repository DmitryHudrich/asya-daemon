use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Responses {
    Base {
        is_err: bool,
        message: String,
    },
}
