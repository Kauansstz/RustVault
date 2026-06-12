#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_must_use)]

use std::io::{self, Write};
use std::process::Command;
use std::thread;
use std::time::Duration;
use sysinfo::{System, Disks};

mod models;
mod service;
mod utils;

use models::livro::Livro;
use service::cadastrar_livro::CadastrarLivro;
use service::listar_livros::ListarLivros;
use service::emprestimo_livro::EmprestimoLivro;
use service::devolucao::Devolucao;
use service::pesquisa::Pesquisa;
use utils::emprestimo::Status;

fn main() {
    println!("Analisando métricas reais de hardware...");
    
    let mut sys = System::new_all();
    sys.refresh_all();
    thread::sleep(Duration::from_millis(200));
    sys.refresh_cpu_usage();

    // 1. Cálculo Real da RAM
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let ram_uso_porcentagem = if total_memory > 0 {
        ((used_memory as f64 / total_memory as f64) * 100.0) as u64
    } else {
        0
    };

    // 2. Cálculo Real do Disco Principal (Usando a nova struct Disks da v0.30)
    let mut disco_uso_porcentagem = 0;
    let mut disco_livre_gb = 0.0;
    
    let disks = Disks::new_with_refreshed_list();
    if let Some(disk) = disks.first() {
        let total_space = disk.total_space();
        let available_space = disk.available_space();
        disco_livre_gb = (available_space as f64) / 1024.0 / 1024.0 / 1024.0;
        
        if total_space > 0 {
            disco_uso_porcentagem = (((total_space - available_space) as f64 / total_space as f64) * 100.0) as u64;
        }
    }

    // 3. Carga de CPU simplificada na v0.30
    let cpu_uso = sys.global_cpu_usage() as u64;

    // Passa os dados reais coletados pelo Rust diretamente para o run.sh via argumentos
    let status = if cfg!(target_os = "windows") {
        Command::new("sh")
            .arg("run.sh")
            .arg("--check")
            .arg(ram_uso_porcentagem.to_string())
            .arg(disco_uso_porcentagem.to_string())
            .arg(cpu_uso.to_string())
            .arg(format!("{:.1}", disco_livre_gb))
            .status()
    } else {
        Command::new("./run.sh")
            .arg("--check")
            .arg(ram_uso_porcentagem.to_string())
            .arg(disco_uso_porcentagem.to_string())
            .arg(cpu_uso.to_string())
            .arg(format!("{:.1}", disco_livre_gb))
            .status()
    };

    match status {
        Ok(s) if s.success() => {
            println!("\n[SISTEMA PRONTO] Entrando no ambiente da biblioteca...");
            thread::sleep(Duration::from_secs(1));
        }
        _ => {
            eprintln!("\n[ALERTA] O run.sh detectou anomalias ou reiniciou serviços pendentes.");
            thread::sleep(Duration::from_secs(2));
        }
    }

    let mut cadastrar = CadastrarLivro::new();
    
    loop {    
        print!("{}[2J{}[1;1H", 27 as char, 27 as char);
        io::stdout().flush().unwrap();

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
                println!("\n--- Cadastro de Novo Livro ---");
                print!("Titulo: ");
                io::stdout().flush().unwrap();
                let mut titulo = String::new();
                io::stdin().read_line(&mut titulo).unwrap();
                
                print!("Classificação: ");
                io::stdout().flush().unwrap();
                let mut classificacao = String::new();
                io::stdin().read_line(&mut classificacao).unwrap();
                
                let novo_livro = Livro {
                    id: 0,
                    titulo: titulo.trim().to_string(),
                    classificacao: classificacao.trim().to_string(),
                    emprestimo: Status::DISPONIVEL,
                    usuario: None,
                };

                match cadastrar.cadastrar_livro(novo_livro) {
                    Ok(()) => println!("Livro salvo com sucesso no arquivo!"),
                    Err(erro) => println!("Falha ao cadastrar: {}", erro),
                }
                thread::sleep(Duration::from_secs(2));
            } 
            "2" => {
                println!("\n--- Listagem de livros disponiveis ---");
                let listagem = ListarLivros::carregar_livros();
                listagem.exibir_livros();
                
                println!("\nPressione Enter para voltar ao menu...");
                let mut _pausa = String::new();
                io::stdin().read_line(&mut _pausa).unwrap();
            } 
            "3" => {
                println!("\n--- Listagem de livros disponiveis ---");
                let mut biblioteca = EmprestimoLivro::carregar_livros();
                biblioteca.exibir_livros();

                println!("\n--- Digite um ID para selecionar ---");
                let mut numero_livro = String::new();
                io::stdin().read_line(&mut numero_livro).expect("Falha ao ler a linha");
                
                match biblioteca.alterar_emprestimo(numero_livro) {
                    Ok(_) => {
                        println!("Transferência efetuada!");
                        EmprestimoLivro::salvar_json(&biblioteca); 
                    }
                    Err(mensagem_erro) => {
                        println!("Falha em fazer a transferência: {}", mensagem_erro);
                    }
                }
                thread::sleep(Duration::from_secs(2));
            }
            "4" => {
                println!("\n--- Listagem de livros indisponiveis ---");
                let mut biblioteca = Devolucao::carregar_livros();
                biblioteca.exibir_livros();

                println!("\n--- Digite um ID para selecionar ---");
                let mut numero_livro = String::new();
                io::stdin().read_line(&mut numero_livro).expect("Falha ao ler a linha");
                
                match biblioteca.alterar_emprestimo(numero_livro) {
                    Ok(_) => {
                        println!("Devolução efetuada com sucesso!");
                        Devolucao::salvar_json(&biblioteca); 
                    }
                    Err(mensagem_erro) => {
                        println!("Falha ao devolver: {}", mensagem_erro);
                    }
                }
                thread::sleep(Duration::from_secs(2));
            } 
            "5" => {
                println!("\n");
                let biblioteca = Pesquisa::carregar_json();
                println!("--- Digite um ID ou um Nome para pesquisar ---");
                let mut numero_livro = String::new();
                io::stdin().read_line(&mut numero_livro).expect("Falha ao ler a linha");
                biblioteca.search(numero_livro);

                println!("\nPressione Enter para continuar... ");
                let mut _pausa = String::new();
                io::stdin().read_line(&mut _pausa).unwrap();
            } 
            "6" => {
                println!("Saindo do sistema... Até logo!");
                break; 
            }
            _ => { 
                println!("Opção inválida! Tente novamente.");
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}