use serde::{Deserialize, Serialize};

/// Represents the possible responses from the server.
#[derive(Serialize, Deserialize)]
pub enum Responses {
    /// A basic response indicating success or failure with an opinion from Asya.
    Base { is_err: bool, message: String },
}
