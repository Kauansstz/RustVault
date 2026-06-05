use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
struct Estoque{
    id: u32,
    genero: String,
    classificacao: u8
}