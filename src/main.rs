use chrono::Local;
use clap::Parser;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use unicode_width::UnicodeWidthStr;

const BG_BLUE: &str = "\x1b[44m";
const CYAN: &str = "\x1b[36m";
const YELLOW: &str = "\x1b[33m";
const WHITE: &str = "\x1b[97m";
const MAGENTA: &str = "\x1b[35m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

#[derive(Parser, Debug)]
#[command(name = "rust_clock")]
#[command(about = "Relógio ASCII customizável no terminal", long_about = None)]
struct Args {
    #[arg(
        long,
        default_value = "blue",
        help = "Cor de fundo: blue, cyan, yellow, white, magenta, green, red"
    )]
    bg: String,

    #[arg(long, default_value = "cyan", help = "Cor da borda")]
    borda: String,

    #[arg(long, default_value = "cyan", help = "Cor do texto/ASCII")]
    texto: String,

    #[arg(long, default_value = "yellow", help = "Cor da hora")]
    hora: String,

    #[arg(long, action, help = "Lista todas as cores disponíveis")]
    cores: bool,
}

fn cor_ansi(nome: &str) -> &str {
    match nome.to_lowercase().as_str() {
        "blue" | "azul" => BG_BLUE,
        "cyan" | "ciano" => CYAN,
        "yellow" | "amarelo" => YELLOW,
        "white" | "branco" => WHITE,
        "magenta" | "rosa" => MAGENTA,
        "green" | "verde" => GREEN,
        "red" | "vermelho" => RED,
        _ => BG_BLUE,
    }
}

fn mostrar_cores() {
    println!(
        "{bold}🎨 Cores disponíveis:{reset}\n",
        bold = BOLD,
        reset = RESET
    );
    println!("{bg} blue/azul {reset}", bg = BG_BLUE, reset = RESET);
    println!("{c} cyan/ciano {reset}", c = CYAN, reset = RESET);
    println!("{y} yellow/amarelo {reset}", y = YELLOW, reset = RESET);
    println!("{w} white/branco {reset}", w = WHITE, reset = RESET);
    println!("{m} magenta/rosa {reset}", m = MAGENTA, reset = RESET);
    println!("{g} green/verde {reset}", g = GREEN, reset = RESET);
    println!("{r} red/vermelho {reset}\n", r = RED, reset = RESET);
    println!("Ex: cargo run --bg magenta --borda cyan --texto white --hora yellow");
}

fn limpar_tela() {
    print!("\x1b[2J\x1b[H");
}

fn largura_visual(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

fn linha_borda(largura: usize, tipo: &str, bg: &str, borda: &str) -> String {
    let linha = "═".repeat(largura);
    match tipo {
        "topo" => format!(
            "{bg}{borda}╔{linha}╗{reset}",
            bg = bg,
            borda = borda,
            linha = linha,
            reset = RESET
        ),
        "meio" => format!(
            "{bg}{borda}╠{linha}╣{reset}",
            bg = bg,
            borda = borda,
            linha = linha,
            reset = RESET
        ),
        "base" => format!(
            "{bg}{borda}╚{linha}╝{reset}",
            bg = bg,
            borda = borda,
            linha = linha,
            reset = RESET
        ),
        _ => String::new(),
    }
}

fn linha_conteudo(
    largura: usize,
    texto: &str,
    cor: &str,
    negrito: bool,
    bg: &str,
    borda: &str,
) -> String {
    let texto_len = largura_visual(texto);
    let espacos = (largura.saturating_sub(texto_len)) / 2;
    let espacos_fim = largura - espacos - texto_len;
    let estilo = if negrito { BOLD } else { "" };

    format!(
        "{bg}{borda}║{esp_esq}{estilo}{cor}{texto}{esp_dir}║{reset}",
        bg = bg,
        borda = borda,
        esp_esq = " ".repeat(espacos),
        estilo = estilo,
        cor = cor,
        texto = texto,
        esp_dir = " ".repeat(espacos_fim),
        reset = RESET
    )
}

// Exemplos para cor_ansi

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cor_ansi_aceita_nome_em_ingles() {
        assert_eq!(cor_ansi("blue"), BG_BLUE);
        assert_eq!(cor_ansi("cyan"), CYAN);
        assert_eq!(cor_ansi("red"), RED);
    }

    #[test]
    fn cor_ansi_aceita_nome_em_portugues() {
        assert_eq!(cor_ansi("azul"), BG_BLUE);
        assert_eq!(cor_ansi("amarelo"), YELLOW);
        assert_eq!(cor_ansi("verde"), GREEN);
    }

    #[test]
    fn cor_ansi_e_case_insensitive() {
        assert_eq!(cor_ansi("BLUE"), BG_BLUE);
        assert_eq!(cor_ansi("MaGeNtA"), MAGENTA);
    }

    #[test]
    fn cor_ansi_retorna_default_para_cor_invalida() {
        assert_eq!(cor_ansi("roxo"), BG_BLUE);
        assert_eq!(cor_ansi(""), BG_BLUE);
    }
}

// Exemplos para largura_visual

#[test]
fn largura_visual_conta_ascii_corretamente() {
    assert_eq!(largura_visual("abc"), 3);
    assert_eq!(largura_visual("Fabio"), 5);
}

#[test]
fn largura_visual_conta_emoji_como_dois() {
    assert_eq!(largura_visual("⚡"), 2);
    assert_eq!(largura_visual(" ⚡"), 3);
}

#[test]
fn largura_visual_lida_com_string_vazia() {
    assert_eq!(largura_visual(""), 0);
}

#[test]
fn largura_visual_conta_acentos_corretamente() {
    assert_eq!(largura_visual("São Paulo"), 9);
}

fn main() {
    let args = Args::parse();

    if args.cores {
        mostrar_cores();
        return;
    }

    let bg = cor_ansi(&args.bg);
    let cor_borda = cor_ansi(&args.borda);
    let cor_texto = cor_ansi(&args.texto);
    let cor_hora = cor_ansi(&args.hora);

    limpar_tela();
    println!(
        "{bold}💻 Terminal do Fabio{reset}",
        bold = BOLD,
        reset = RESET
    );
    println!("Use --help pra ver todas as opções\n");

    let largura = 46;
    let titulo = " Developer ";

    loop {
        let agora = Local::now();
        let data = agora.format("%d/%m/%Y %A").to_string();
        let hora = agora.format("%H:%M:%S").to_string();

        limpar_tela();

        let ascii_arte = [
            "⚡",
            "┌──────────┐",
            "│ 01010101 │",
            "│   CPU    │",
            "│  { } ;   │",
            "└──────────┘",
            "Compilando tempo...",
        ];

        println!("{}", linha_borda(largura, "topo", bg, cor_borda));
        println!(
            "{}",
            linha_conteudo(largura, titulo, WHITE, true, bg, cor_borda)
        );
        println!("{}", linha_borda(largura, "meio", bg, cor_borda));

        for linha in ascii_arte {
            println!(
                "{}",
                linha_conteudo(largura, linha, cor_texto, false, bg, cor_borda)
            );
        }

        println!(
            "{bg}{borda}║{esp}║{reset}",
            bg = bg,
            borda = cor_borda,
            esp = " ".repeat(largura),
            reset = RESET
        );
        println!(
            "{bg}{borda}║{linha}║{reset}",
            bg = bg,
            borda = cor_borda,
            linha = "─".repeat(largura),
            reset = RESET
        );

        println!(
            "{}",
            linha_conteudo(largura, &data, WHITE, false, bg, cor_borda)
        );
        println!(
            "{}",
            linha_conteudo(largura, &hora, cor_hora, true, bg, cor_borda)
        );

        println!("{}", linha_borda(largura, "base", bg, cor_borda));

        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}
