use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub id: u64,
    pub timestamp: u64,
    pub command: String,
    pub args: Vec<String>,
}
