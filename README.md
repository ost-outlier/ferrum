# 🦀 Ferrum Editor Engine

**Ferrum** é um motor de edição de texto rico, modular e embeddável em Rust, com paridade arquitetural direta com o **CodeMirror 6 (CM6)**.

Projetado para ser incorporado por qualquer aplicação desktop (GPUI, wgpu, egui, iced) ou ferramentas de linha de comando/terminal sem exigir WebView, sem runtime JavaScript e sem pontes IPC serializadas.

---

## ⚡ Princípios e Destaques Arquiteturais

- **100% Permissivo (MIT / Apache-2.0)**: Desenvolvido sob estrita quarentena *Clean-Room*. Zero código proprietário ou sob licenças copyleft (GPL).
- **Paridade Comportamental com o CodeMirror 6**: Implementado com TDD estrito usando os testes oficiais do CodeMirror 6 (`@codemirror/state`, `@codemirror/commands`) como gabarito canônico exato.
- **Transformação Operacional Real (OT)**: Alterações compactadas em seções (`ChangeSet` / `ChangeDesc`) com suporte matemático completo a `compose`, `map_desc`, `map`, `invert` e `apply` linear. Confluência $A \circ B' = B \circ A'$ garantida por testes de propriedade com geração aleatória.
- **Manipulação Rigorosa de Unicode**: Deslocamentos, deleções (backspace/delete) e navegação por palavras respeitam *Unicode Grapheme Clusters* (via `unicode-segmentation`), acentuações compostas, emojis complexos e quebras de linha (CRLF e LF). Zero saltos ingênuos de bytes/chars.
- **Sistema de Facets sem Conflito**: Extensões não herdam classes nem mutam o editor diretamente; contribuem valores tipados para Facets. Múltiplas extensões contribuindo para o mesmo Facet coexistem harmonicamente.
- **Histórico e Undo/Redo Determinístico**: Motor de histórico baseado em `StateField`, com anotações de transação (`TransactionTime`, `IsolateHistory`, `AddToHistory`), agrupamento temporal (`new_group_delay`) e projeção de mudanças remotas.

---

## 📦 Estrutura do Workspace Cargo

```text
crates/
├── ferrum-state/       # Core funcional: EditorState, ChangeSet, Facet, Extension, Text, Selection
├── ferrum-commands/    # Comandos de edição, movimento por graphemes, keymaps e history (undo/redo)
├── ferrum-view/        # Viewport virtualizado, DecorationSet (Mark, Replace, Widget), RenderedLine
└── ferrum/             # Crate fachada principal que reexporta as crates via feature flags
```

---

## 🚀 Guia Rápido (Quickstart)

Adicione o Ferrum ao seu `Cargo.toml`:

```toml
[dependencies]
ferrum = { path = "crates/ferrum" } # ou via crates.io quando publicado
```

### 1. Criando um Estado com Extensões

```rust
use ferrum::state::{EditorState, EditorStateConfig, Text, TransactionSpec};
use ferrum::commands::{history, default_keymap, insert_text, undo, redo, HistoryConfig, KEYMAP};
use ferrum::view::EditorView;

fn main() {
    // 1. Configurar o estado imutável com extensões
    let state = EditorState::create(EditorStateConfig {
        doc: Some(Text::from("fn main() {\n    println!(\"Hello, Ferrum!\");\n}\n")),
        selection: None,
        extensions: Some(vec![
            history(HistoryConfig::default()),
            KEYMAP.of(default_keymap()),
        ]),
    });

    // 2. Instanciar a View (controladora do editor)
    let mut view = EditorView::new(state);

    // 3. Executar edições através de comandos ou especificações de transação
    let insert_spec = insert_text(view.state(), "// Licença: MIT\n").unwrap();
    view.dispatch(insert_spec);

    println!("Documento após inserção:\n{}", view.state().doc());

    // 4. Desfazer e refazer via histórico canônico
    let undo_spec = undo(view.state()).unwrap();
    view.dispatch(undo_spec);
    println!("Documento após desfazer:\n{}", view.state().doc());

    let redo_spec = redo(view.state()).unwrap();
    view.dispatch(redo_spec);
    println!("Documento após refazer:\n{}", view.state().doc());
}
```

### 2. Multi-Cursor com Transformação Operacional

```rust
use ferrum::state::{EditorSelection, SelectionRange, TransactionSpec};
use ferrum::commands::insert_text;
use ferrum::view::EditorView;

// Criar múltiplos cursores simultâneos
let multi_sel = EditorSelection::create(
    vec![
        SelectionRange::cursor(0),
        SelectionRange::cursor(24),
    ],
    0,
);

// Aplicar a nova seleção
view.dispatch(TransactionSpec::new().with_selection(multi_sel));

// Digitar concorrentemente em todos os cursores
let multi_insert = insert_text(view.state(), "    // [TAG] ").unwrap();
view.dispatch(multi_insert);
```

### 3. Filtros de Transação

Você pode interceptar e bloquear transações arbitrariamente antes que elas sejam aplicadas:

```rust
use ferrum::state::{TRANSACTION_FILTER, Transaction};
use std::sync::Arc;

let block_forbidden = Arc::new(|tr: &Transaction| {
    let mut allowed = true;
    tr.changes().iter_changes(|_, _, _, _, inserted| {
        if inserted.to_string().contains("@bloqueado") {
            allowed = false; // Cancela a transação
        }
    }, false);
    allowed
});

// Adicione ao extensions do EditorState:
// TRANSACTION_FILTER.of(block_forbidden)
```

---

## 🧪 Suíte de Testes e Paridade

O Ferrum possui uma rigorosa suíte de testes unitários (100% verde):

```bash
# Executa todos os testes do workspace
cargo test --workspace

# Executa os testes de histórico e comandos com paridade ao CodeMirror 6
cargo test -p ferrum-commands

# Executa os testes de OT, clipping e confluência matemática
cargo test -p ferrum-state
```

---

## 🖥️ Executando a Prova de Conceito (POC)

```bash
cargo run -p ferrum --example poc_editor
```

---

## 📄 Licença

Este projeto é licenciado sob a licença **MIT** — consulte o arquivo LICENSE para detalhes.
