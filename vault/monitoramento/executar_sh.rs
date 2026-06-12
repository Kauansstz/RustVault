use std::process::Command;
use std::thread;
use std::time::Duration;

pub fn executar_scripts_monitoramento() {
    println!("\n=========================================");
    println!("📊 INICIANDO SISTEMA DE MONITORAMENTO");
    println!("=========================================");
    
    // Pequena pausa dramática para ficar bonito no terminal
    thread::sleep(Duration::from_millis(500)); 

    // Usamos o .status() para que o script use o MESMO terminal do Rust em tempo real
    let status = Command::new("python") // ou "bash", ou "powershell"
        .arg("run.sh")       // O nome do seu script principal
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("\n✅ Monitoramento finalizado com sucesso!");
        }
        Ok(_) => {
            println!("\n⚠️ O script de monitoramento fechou com algum erro.");
        }
        Err(e) => {
            println!("\n❌ Erro crítico: Não foi possível iniciar o script. Detalhes: {}", e);
        }
    }

    // Dá tempo para o usuário ler o resultado antes de voltar para o menu da biblioteca
    println!("\nVoltando ao menu em 3 segundos...");
    thread::sleep(Duration::from_secs(3));
}