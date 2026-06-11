use std::fs;
use std::io;
use std::thread;
use std::time::Duration;
use crate::models::livro::Livro;
use crate::utils::emprestimo;
use crate::utils::emprestimo::Status;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Devolucao {
    pub livros: Vec<Livro>
}

impl Devolucao {
       pub fn carregar_livros() -> Self{
        let livros = match fs::read_to_string("livro.json"){
            Ok(conteudo)=> {
                match serde_json::from_str::<Vec<Livro>>(&conteudo){
                    Ok(lista) => lista,
                    Err(erro)=> {
                        println!("Erro ao ler os dados do Json: {}", erro);
                        Vec::new()
                    }
                }
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

    let possui_indisponiveis = self.livros.iter().any(|l| matches!(l.emprestimo, Status::Emprestado { .. }));
    println!("TESTE");
    if !possui_indisponiveis {
        println!("Nenhum livro indisponível no momento.");
        return;
    }
    println!("{:-<60}", "-");
    println!("{:<4} | {:<20} | {:<15} | {:<12}", "ID", "Título", "Classificação", "Empréstimo");
    println!("{:-<60}", "-");
    for livro in &self.livros {
            if let Status::Emprestado { usuario } = &livro.emprestimo {
            
            let status_formatado = format!("Emprestado para: {}", usuario);
            
            println!(
                "{:<4} | {:<20} | {:<15} | {:<12}",
                livro.id, livro.titulo, livro.classificacao, status_formatado
            );
        }
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
        thread::sleep(Duration::from_secs(3));
        
        livro.emprestimo = emprestimo::Status::DISPONIVEL;
        
        thread::sleep(Duration::from_secs(2));
        self.salvar_json().map_err(|e| format!("Erro ao salvar arquivo: {}", e))?;

        println!("Transferência concluída!");

        Ok(())
    }
}