#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_must_use)]
use std::{io::{self, Write}};
mod models;
mod service;
mod utils;

use models::livro::Livro;
use service::cadastrar_livro::CadastrarLivro;
use service::listar_livros::ListarLivros;
use service::emprestimo_livro::EmprestimoLivro;
use std::thread;
use std::time::Duration;
use crate::{models::livro, utils::emprestimo::{self, Status}};

fn main() {
    let mut cadastrar = CadastrarLivro::new();
    loop {    
        println!("");
        println!("");
        println!("");
        println!("");
        println!("Bem-vindo a nossa biblioteca!");
        println!("Escolha uma opção abaixo:");
        println!("{}", "=".repeat(40));
        println!(
            "
            1 - Cadastrar livro
            2 - Listar livros
            3 - Emprestimo de livro
            4 - Devolver livro
            5 - Buscar livro
            6 - Sair
        "
        );
        println!("{}", "=".repeat(40));

        print!("Escolha uma opção: ");
        io::stdout().flush().unwrap();

        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Erro ao ler a entrada");


        match response.trim() {     
            "1" => {
                println!("--- Cadastro de Novo Livro ---");
                
                println!("Titulo: ");
                let mut titulo = String::new();
                io::stdin().read_line(&mut titulo).unwrap();
                
                println!("Classificação: ");
                let mut classificacao = String::new() ;
                io::stdin().read_line(&mut classificacao).unwrap();
                let novo_livro = Livro{
                    id: 0,
                    titulo: titulo.trim().to_string(),
                    classificacao: classificacao.trim().to_string(),
                    emprestimo: Status::DISPONIVEL,
                    usuario: None,
                };

                match cadastrar.cadastrar_livro(novo_livro){
                    Ok(()) => println!("Livro salvo com sucesso no arquivo!"),
                    Err(erro) => println!("Falha ao cadastrar: {}", erro),
                }
            } 
            "2" => {
                println!();
                println!("--- Listagem de livros disponiveis ---");
                let listagem = ListarLivros::carregar_livros();
                listagem.exibir_livros();

            } 
            "3" => {
                println!();
                println!("--- Listagem de livros disponiveis ---");
                let mut biblioteca = EmprestimoLivro::carregar_livros();
                biblioteca.exibir_livros();

                println!();
                println!("--- Digite um ID para selecionar ---");
                let mut numero_livro = String::new();
                io::stdin().read_line(&mut numero_livro).expect("Falha ao ler a linha");
                match biblioteca.alterar_emprestimo(numero_livro) {
                    Ok(_) => {
                        thread::sleep(Duration::from_secs(2));
                        println!("");
                        EmprestimoLivro::salvar_json(&biblioteca); 
                    }
                    Err(mensagem_erro) => {
                        thread::sleep(Duration::from_secs(2));
                        println!("");
                        println!("Falha em fazer a transferência: {}", mensagem_erro);
                    }
                }
            }
            "4" => {
                println!("A opção escolhida foi a 4");
            } 
            "5" => {
                println!("A opção escolhida foi a 5");
            } "6" => {
                println!("Saindo do sistema... Até logo!");
                break; 
            }
            _ => { 
                println!("Opção inválida! Tente novamente.");
            }
        }
    }
}
