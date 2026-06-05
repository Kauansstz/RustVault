use serde::{Serialize, Deserialize};
use crate::utils::emprestimo::Status;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Livro{
    pub id:u32,
    pub titulo:String,
    pub emprestimo:Status,
    pub classificacao:String,
    pub usuario:Option<String>,
}

impl Livro{
    pub fn novo(titulo: String, classificacao: String) -> Self{
        Self {
            id: 0,
            titulo,
            classificacao,
            emprestimo: Status::DISPONIVEL,
            usuario: None,
        }
    }
}