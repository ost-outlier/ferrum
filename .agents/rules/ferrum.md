# Diretrizes de Desenvolvimento do Ferrum Editor Engine

Este documento define as regras obrigatórias, fluxo de trabalho e padrões arquiteturais para todo o desenvolvimento do **Ferrum** — o motor de edição de texto rico e embeddável em Rust, com paridade arquitetural direta com o **CodeMirror 6 (CM6)**.

---

## 1. Princípio Fundamental: TDD com Paridade Direta ao CodeMirror 6

Todo o desenvolvimento no Ferrum é estritamente orientado a testes (**Test-Driven Development - TDD**), utilizando os repositórios oficiais do CodeMirror 6 localizados em `/workspace/CodeMirror6/` como **gabarito canônico e especificação comportamental exata**.

### 1.1 O Ciclo Obrigatório de 4 Passos (Red-Green-Refactor)

Antes de alterar ou criar qualquer linha de código de implementação:

1. **Passo 1 — RED (Escrever o Teste Primeiro)**:
   - Localize o arquivo de teste correspondente no CodeMirror 6 (`/workspace/CodeMirror6/<pacote>/test/test-*.ts`).
   - Traduza os casos de teste (`describe` / `it`) de TypeScript para testes unitários em Rust (`#[test]`) no módulo de testes da crate apropriada (`tests.rs`).
   - Use helpers como `mk_state` e `state_str` para representar estados com cursores (`/` ou `<...>` para âncora/cabeça) e seleções múltiplas.
2. **Passo 2 — RED VERIFICATION (Confirmar Falha)**:
   - Execute `cargo test -p <crate> --lib tests::<nome_do_teste>`.
   - Verifique que o teste compila (com stubs mínimos de assinatura, se for nova API) e **falha exatamente pela ausência da lógica planejada**.
3. **Passo 3 — GREEN (Implementação Fiel)**:
   - Implemente a lógica necessária para satisfazer o teste, traduzindo o algoritmo original do CodeMirror 6 sem atalhos ou aproximações ingênuas.
   - Execute novamente o teste até ficar 100% verde.
4. **Passo 4 — REFACTOR (Polimento e Invariantes)**:
   - Elimine duplicações, preserve a legibilidade idiomática de Rust e garanta zero *compiler warnings*.
   - Execute a suíte completa: `cargo test --workspace`. Todos os testes devem passar.

### 1.2 Regras Estritas de Implementação (Tolerância Zero a Atalhos)

* **Proibição de Deslocamentos Crús (Byte/Char Offsets)**:
  Toda movimentação de cursor, deleção (backspace/delete) e quebra de palavras DEVE respeitar *Unicode Grapheme Clusters* (usando `unicode-segmentation`), acentuações combinadas, emojis e quebras de linha (CRLF e LF). Jamais use `head + 1` ou `head - 1`.
* **Transformação Operacional Real (OT)**:
  Mudanças em documentos utilizam o modelo de seções (`ChangeSet` / `ChangeDesc` com `SectionIter`, `compose`, `map`, `invert` e `apply`). Jamais substitua OT por listas ingênuas de substituições aplicadas sequencialmente em loop.
* **Normalização Exata de Seleção (`EditorSelection.normalized`)**:
  - Seleções adjacentes vazias (pontos) no mesmo offset se fundem.
  - Seleções não vazias que apenas se tocam (ex.: `6..7` e `7..8`) **coexistem** e não se fundem (invariante do CM6).
  - Em seleções sobrepostas, a direção (invertida ou não) do intervalo resultante é herdada do **último** intervalo mesclado.
* **Tratamento de Linhas Selecionadas**:
  Ao aplicar comandos por linha (`change_by_selected_line`), se uma seleção termina no início de uma linha (`range.to == line.from`), essa linha subsequente **não** deve ser afetada.

---

## 2. Mapa de Correspondência: CodeMirror 6 ↔ Ferrum

Cada componente do CodeMirror 6 possui sua contraparte direta no workspace do Ferrum:

| Pacote CodeMirror 6 (`/workspace/CodeMirror6/`) | Módulo CM6 | Crate Ferrum (`/workspace/ferrum/crates/`) | Arquivo Ferrum | Responsabilidade |
| :--- | :--- | :--- | :--- | :--- |
| `@codemirror/state` | `change.ts` | `ferrum-state` | `src/change.rs` | `ChangeDesc`, `ChangeSet`, OT, `compose`, `map`, `invert`, `apply` |
| `@codemirror/state` | `selection.ts` | `ferrum-state` | `src/selection.rs` | `SelectionRange`, `EditorSelection`, normalização de multi-cursor |
| `@codemirror/state` | `state.ts` | `ferrum-state` | `src/state.rs` | `EditorState`, `Transaction`, `TransactionSpec`, `change_by_range` |
| `@codemirror/state` | `facet.ts` | `ferrum-state` | `src/facet.rs` | Sistema de Facets, `FacetDefinition`, `Compartment`, `Configuration` |
| `@codemirror/state` | `char.ts` | `ferrum-state` | `src/char.rs` | Graphemes, `find_cluster_break`, `find_word_break`, `find_subword_break` |
| `@codemirror/commands` | `commands.ts` | `ferrum-commands` | `src/commands.rs` | Comandos de cursor, linhas, indentação, deleções, `KEYMAP` facet |
| `@codemirror/commands` | `history.ts` | `ferrum-commands` / `ferrum-state` | `src/history.rs` | Undo/Redo stacks, isolamento de eventos, inversão de `ChangeSet` |
| `@codemirror/view` | `view.ts`, `decoration.ts` | `ferrum-view` | `src/lib.rs` / `src/view.rs` | `EditorView`, `RangeSet`, `DecorationSet`, virtualização de viewport |

---

## 3. Quarentena do Zed e Diretrizes de Licenciamento

O repositório do Zed está clonado em `/workspace/zed/`. O Zed é licenciado sob licenças copyleft (**GPL-3.0 / AGPL**). O Ferrum é e deve permanecer uma biblioteca **100% permissiva (MIT ou MIT / Apache-2.0)**.

### 3.1 Regras de Quarentena "Clean-Room"

1. **JAMAIS COPIAR OU COLAR CÓDIGO DO ZED**:
   É terminantemente proibido copiar arquivos, funções, estruturas ou snippets do diretório `/workspace/zed/` para qualquer crate do Ferrum. Fazer isso contaminaria o Ferrum com a GPL.
2. **Uso Permitido do Zed**:
   O Zed serve **apenas** para estudo conceitual abstrato de como o GPUI gerencia janelas nativas, ciclos de frame de GPU e rasterização de fontes/glifos.
3. **Isolamento de Tipos e Dependências**:
   - Nenhuma struct, enum ou trait pública do Ferrum pode expor ou importar tipos do Zed.
   - As crates `ferrum-state` e `ferrum-commands` são **100% Rust puro**, sem dependência de GPUI, wgpu ou qualquer framework de UI.
   - Qualquer adaptador de renderização para GPUI futuro deve residir em uma crate bridge separada (ex.: `ferrum-view-gpui`), mantendo o core do Ferrum independente.

---

## 4. Roteiro e Diretrizes para Próximas Etapas

### 4.1 Histórico e Undo/Redo (`history.ts`)
- **Conceito**: O CodeMirror 6 implementa o histórico como um conjunto de Facets e StateFields (`historyField`). Ele agrupa transações em eventos de undo com base em limites de tempo ou anotações explícitas (`isolateHistory`).
- **Gabarito de Testes**: Portar `/workspace/CodeMirror6/commands/test/test-history.ts` para `ferrum-commands/src/tests.rs` antes de implementar `history.rs`.
- **Mapeamento de Inversão**: Usa `ChangeSet.invert(&doc)` já implementado no `ferrum-state`.

### 4.2 Decorações e Virtualização de Viewport (`ferrum-view`)
- **Estrutura**: Replicar o `RangeSet` / `DecorationSet` do CM6 (índice em árvore B/chunks de intervalos para marcações de sintaxe, widgets em linha, gutters e decorações de linha).
- **Virtualização**: Calcular apenas as linhas dentro do retângulo visível mais margem de overscan. O Ferrum State fornece o texto em fatias $O(\log N)$ via `ropey`; a View projeta apenas a viewport ativa.
- **Gabarito de Testes**: `/workspace/CodeMirror6/view/test/test-decoration.ts`.

### 4.3 Integração com Árvores Sintáticas (`ferrum-language`)
- Em Rust, utilizar o **Tree-sitter** com parsing incremental associado a um `StateField` do documento.
- Facets para linguagens fornecerão:
  - Regras de indentação automática.
  - Símbolos de comentários (`//`, `/* */`, `#`).
  - Suporte a syntax highlighting com spans mapeados via `DecorationSet`.

### 4.4 Colaboração em Tempo Real e CRDTs
- O Ferrum adota `ropey` para manuseio eficiente de strings e implementa OT completa (`ChangeSet.compose` e `ChangeSet.map`).
- **Facilidade com CRDTs**: A arquitetura do CM6 adotada pelo Ferrum é a mesma utilizada pelo ecossistema web para integrar Yjs e Automerge. As transações no Ferrum são determinísticas e invertíveis, permitindo rebasing local de operações concorrentes antes da sincronização remota, sem poluir o núcleo do editor com lógica de rede.

---

## 5. Padrões de Código e Diretrizes Rust

* **Sem Pânico em Produção**:
  Evite `unwrap()` e `expect()` em fluxos de execução de produção. Retorne `Result` ou lide graciosamente com limites.
* **Organização de Módulos**:
  Prefira `src/modulo.rs` a `src/modulo/mod.rs`.
* **Zero Warnings**:
  Todo código novo deve compilar sem avisos de warning (`#[warn(unused)]`, etc.).
* **Comentários de Código**:
  Não escreva comentários óbvios que apenas narram o que a linha faz. Documente o **porquê** de decisões não triviais e edge cases matemáticos/Unicode.
