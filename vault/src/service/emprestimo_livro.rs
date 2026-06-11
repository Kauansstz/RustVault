use std::fs;
use std::io;
use std::thread;
use std::time::Duration;
use crate::models::livro::Livro;
use crate::utils::loading::loading;
use crate::utils::emprestimo;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmprestimoLivro {
    pub livros: Vec<Livro>
}

impl EmprestimoLivro {
       pub fn carregar_livros() -> Self{
        let livros = match fs::read_to_string("livro.json"){
            Ok(conteudo)=> {
                serde_json::from_str(&conteudo).unwrap_or_else(|_| Vec::new())
            }
            Err(_) => Vec::new()
        };
        Self { livros }
    }

    pub  fn salvar_json(&self)-> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.livros)?;
        fs::write("livro.json", json)?;
        Ok(())
    }

    pub fn exibir_livros(&self) {
        if self.livros.is_empty() {
            println!("Nenhum livro encontrado nas prateleiras.");
            return;
        }
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

    pub fn alterar_emprestimo(&mut self, numero_livro: String) -> Result<(), String> {
        let id_digitado = numero_livro
            .trim()
            .parse::<u32>()
            .map_err(|_| "Por favor, digite um número válido!".to_string())?;

        let livro = self.livros
            .iter_mut()
            .find(|l| l.id == id_digitado)
            .ok_or_else(|| "Nenhum livro encontrado com este ID.".to_string())?;

        println!("Realizando a transferência...");
        loading();
        
        livro.emprestimo = emprestimo::Status::Emprestado { usuario: "teste".to_string() };
        
        thread::sleep(Duration::from_secs(2));
        self.salvar_json().map_err(|e| format!("Erro ao salvar arquivo: {}", e))?;

        println!("Transferência concluída!");

        Ok(())
    }
}