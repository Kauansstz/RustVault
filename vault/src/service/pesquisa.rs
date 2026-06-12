use std::fs;

use crate::{models::livro::Livro, utils::loading::loading};
pub struct Pesquisa{
    livros: Vec<Livro>,
}

impl Pesquisa {
    pub fn carregar_json() -> Self{
        let livros =  match fs::read_to_string("livro.json"){
            Ok(conteudo) => {
                serde_json::from_str(&conteudo).unwrap_or_else(|_| Vec::new())
            }
            Err(_) => Vec::new()
        };
        Self{livros}
    }

    pub fn search(&self, pesquisa:String){
        if self.livros.is_empty() {
            println!("Nenhum livro encontrado nas prateleiras.");
            return;
        }

        let termo_limpo = pesquisa.trim();
        let termo_minusculo = termo_limpo.to_lowercase();

        let id_pesquisado: Option<u32> = termo_limpo.parse::<u32>().ok();
        let encontrou = self.livros.iter().any(|l|{
            match id_pesquisado {
                Some(id) => l.id == id,
                None => l.titulo.to_lowercase().contains(&termo_minusculo),
            }
        });

        if  encontrou {
            
            loading();
            println!();
            println!();
            println!("{:-<60}", "-");
            println!("{:<4} | {:<20} | {:<15} | {:<12}", "ID", "Título", "Classificação", "Empréstimo");
            println!("{:-<60}", "-");
            
            for livro in &self.livros {
                let dar_match = match id_pesquisado{
                    Some(id) => livro.id == id,
                    None => livro.titulo.to_lowercase().contains(&termo_minusculo),
                };

                if dar_match{
                let status_formatado = format!("{:?}", livro.emprestimo);
                
                println!(
                    "{:<4} | {:<20} | {:<15} | {:<12}",
                    livro.id, livro.titulo, livro.classificacao, status_formatado
                );
            }
        }
        println!("{:-<60}", "-");
        }else {
        println!("Nenhum livro encontrado com o termo '{}'.", termo_limpo);
    }

    }
}