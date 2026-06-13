# Rust Clock

Relógio ASCII para terminal com suporte a cores ANSI e flags de linha de comando. Projeto didático para estudo de conceitos fundamentais de Rust.

## Objetivos de Aprendizado

Este projeto demonstra na prática:

1. **Ownership e Borrowing**: Uso de `&str` vs `String` nas funções de formatação. Funções recebem referências para evitar cópias desnecessárias.
2. **Structs e Derives**: `#[derive(Parser, Debug)]` com `clap` mostra como macros procedurais geram código para parsing de argumentos.
3. **Tratamento de Terminal**: Sequências ANSI para cores e limpeza de tela. Controle direto de stdout com `io::Write`.
4. **Concorrência Básica**: `thread::sleep` para loop de atualização com período de 1 segundo.
5. **Crates Externas**: Integração de `chrono` para tempo local, `clap` para CLI, `unicode-width` para cálculo correto de largura de strings com caracteres multi-byte.
6. **Formatação de Strings**: `format!` macro e interpolação nomeada. Construção de layouts ASCII.
7. **Enums e Match**: Função `cor_ansi` usa pattern matching para mapear nomes de cores para códigos ANSI.

## Estrutura do Código

```plaintext
src/main.rs
├── Args: struct com derive Parser para clap
├── cor_ansi: match para conversão de nome -> código ANSI
├── largura_visual: wrapper para unicode-width
├── linha_borda/linha_conteudo: funções puras de formatação
└── main: loop principal com atualização periódica
```

### Detalhes Técnicos

**ANSI Escape Codes**: Códigos como `\x1b[44m` definem cor de fundo. `\x1b[0m` reseta formatação. Necessário resetar após cada linha para evitar vazamento de estilo.

**unicode-width**: Função `UnicodeWidthStr::width` calcula largura de exibição real. Crítico para centralização correta de strings com emoji ou caracteres CJK, pois `str.len()` retorna bytes e `chars().count()` retorna pontos de código Unicode, não colunas do terminal.

**clap derive**: Atributos `#[command]` e `#[arg]` geram implementação de `Parser` para `Args`. Método `Args::parse()` lê `std::env::args()` e faz parsing automático com validação e help gerado.

**Loop de renderização**: `print!` + `io::stdout().flush()` é usado em vez de `println!` para controle granular de output. `\x1b[2J\x1b[H` limpa tela e reposiciona cursor para evitar flicker.

## Compilação e Execução

### Requisitos

- Rust 1.75 ou superior
- Terminal com suporte a ANSI

### Dependências

Definidas em Cargo.toml:

```toml
chrono = "0.4"
clap = { version = "4.5", features = ["derive"] }
unicode-width = "0.1"
```

### Comandos

Build de desenvolvimento:

```bash
cargo build
cargo run --help
```

Build otimizado:

```bash
cargo build --release
./target/release/rust_clock --bg magenta --borda cyan
```

Observação: Com cargo run é necessário usar -- para separar flags do cargo das flags do binário.

### Flags Disponíveis

```javascript
--bg <COR>      Cor de fundo. Padrão: blue
--borda <COR>   Cor da borda. Padrão: cyan  
--texto <COR>   Cor do texto ASCII. Padrão: cyan
--hora <COR>    Cor do horário. Padrão: yellow
--cores         Lista cores disponíveis e encerra
--help          Exibe help gerado pelo clap
```

Cores aceitas: blue/azul, cyan/ciano, yellow/amarelo, white/branco, magenta/rosa, green/verde, red/vermelho.

## Testes Unitários

Rust possui suporte nativo a testes via atributo #[test]. Testes ficam no mesmo arquivo e são executados com cargo test.

### Executando testes

```bash
cargo test
cargo test cor_ansi
cargo test largura_visual --nocapture
```

Flag `--nocapture` exibe saídas de `println!` dentro dos testes.

## Extensões Sugeridas para Estudo

1. **Error Handling:** Substituir unwrap() por Result e ? na função flush().
2. **Configuração:** Adicionar suporte a arquivo de configuração TOML usando crate serde.
3. **Testes:** Escrever testes unitários para largura_visual e cor_ansi usando #[test].
4. **Paralelismo:** Usar tokio para separar lógica de relógio e renderização.
5. **Customização:** Permitir usuário definir ASCII art via arquivo externo.

## Limitações Conhecidas

1. Largura fixa de 46 colunas. Não adapta ao tamanho do terminal.
2. Assume terminal com suporte completo a ANSI. Pode falhar no Windows CMD sem WSL.
3. Formato de data usa locale do sistema via chrono::Local. Não há opção de formatação customizada via flag.
