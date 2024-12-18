use serde::{Deserialize, Serialize};
use usecases::usecases::Usecases;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Requests {
    General { action: Usecases },
}
