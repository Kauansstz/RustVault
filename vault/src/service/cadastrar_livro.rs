use std::fs;
use std::io;
use std::thread;
use std::time::Duration;
use crate::models::livro::Livro;
use crate::utils::loading::loading;
use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CadastrarLivro{
    pub livros: Vec<Livro>,
}


impl CadastrarLivro{

    pub fn new() -> Self {
        let livros = match fs::read_to_string("livro.json"){
            Ok(conteudo)=> {
                serde_json::from_str(&conteudo).unwrap_or_else(|_| Vec::new())
            }
            Err(_) => Vec::new()
        };
        Self { livros }
    }

    pub fn cadastrar_livro(&mut self, mut novo_livro: Livro) -> Result<(), String> {
        self.validar_dados(&novo_livro)?;

        if self.validar_duplicidade(&novo_livro){
            return Err(format!("Já possui o título '{}' cadsatrado.", novo_livro.titulo))
        }
        let proximo_id = self.livros.iter().map(|l| l.id).max().unwrap_or(0) +1;
        novo_livro.id = proximo_id;

        self.livros.push(novo_livro);

        loading();
        thread::sleep(Duration::from_secs(2));
        self.salvar_json().map_err(|e| format!("Erro ao salvar arquivo: {}", e))?;

        println!("Livro cadastrado com sucesso!");
        Ok(())
    }

    pub fn validar_dados(&self, livro:&Livro) -> Result<(), String>{
        if livro.titulo.trim().is_empty() || livro.classificacao.trim().is_empty(){
            return Err("Preencher todos os campos necessários.".to_string());
        }
        Ok(())
    }


    fn validar_duplicidade(&self, livro: &Livro)-> bool{
       self.livros.iter().any(|l| l.titulo == livro.titulo)
    }

    fn salvar_json(&self)-> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.livros)?;
        fs::write("livro.json", json)?;
        Ok(())
    }
}