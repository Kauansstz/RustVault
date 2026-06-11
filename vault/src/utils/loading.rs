use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

pub fn loading() {
    let pb = ProgressBar::new(100);

    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40.white/black}] {percent}%"
        )
        .unwrap()
        // O primeiro caractere é o preenchimento (bloco sólido)
        // O segundo é a ponta da barra (usamos o mesmo bloco para ficar uniforme)
        // O terceiro é o fundo vazio (um espaço em branco ou um bloco cinza claro)
        .progress_chars("██ "), 
    );

    println!("Carregando sistema...");

    for _ in 0..100 {
        pb.inc(1);
        thread::sleep(Duration::from_millis(25));
    }

    pb.finish_with_message("Pronto!");
}