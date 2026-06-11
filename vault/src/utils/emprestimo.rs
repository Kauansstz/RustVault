use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum Status{
    DISPONIVEL,
    Emprestado{
        usuario: String,
    }
}