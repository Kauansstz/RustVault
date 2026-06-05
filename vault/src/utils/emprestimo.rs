use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Status{
    DISPONIVEL,
    Emprestado{
        usuario: String,
    }
}