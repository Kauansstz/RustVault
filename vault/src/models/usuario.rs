use serde::{Deserialize, Serialize};
struct Usuario{
    id: u32,
    nome: String,
    idade: u8,
    email: String,
    cpf: String,
    genero: String
}