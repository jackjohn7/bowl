use serde::{Serialize, Deserialize};

/// Deserializable wrapper for Inquire prompting tools
#[derive(Serialize, Deserialize)]
pub enum Prompt {
    Text {},
    Select {},
}
