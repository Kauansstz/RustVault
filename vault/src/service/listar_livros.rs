
use std::fs;
use crate::{models::livro::Livro, utils::loading::loading};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListarLivros{
    pub livros: Vec<Livro>,
}

impl ListarLivros {
   pub fn carregar_livros() -> Self{
        let livros = match fs::read_to_string("livro.json"){
            Ok(conteudo)=> {
                serde_json::from_str(&conteudo).unwrap_or_else(|_| Vec::new())
            }
            Err(_) => Vec::new()
        };
        Self { livros }
    }

    pub fn exibir_livros(&self) {
        if self.livros.is_empty() {
            println!("Nenhum livro encontrado nas prateleiras.");
            return;
        }
        loading();
        println!();
        println!();
        println!("{:-<60}", "-");
        println!("{:<4} | {:<20} | {:<15} | {:<12}", "ID", "Título", "Classificação", "Empréstimo");
        println!("{:-<60}", "-");

        for livro in &self.livros {
            let status_formatado = format!("{:?}", livro.emprestimo);
            
            println!(
                "{:<4} | {:<20} | {:<15} | {:<12}",
                livro.id, livro.titulo, livro.classificacao, status_formatado
            );
        }
        println!("{:-<60}", "-");
    }
}