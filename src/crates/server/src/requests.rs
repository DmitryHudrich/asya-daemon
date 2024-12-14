use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Requests {
    General { action: String },
}
