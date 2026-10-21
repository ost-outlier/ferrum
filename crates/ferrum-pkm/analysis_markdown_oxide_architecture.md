# Análise Arquitetural do markdown-oxide/ para o Projeto Ferrum
## **O veredicto em resumo:**

1. **O reaproveitamento é viável com cirurgia moderada** . O módulo `vault/` contém 95%+ da lógica que você precisa.

2. **O acoplamento com LSP se resume a um único ponto estrutural**: `MyRange(tower_lsp::lsp_types::Range)` — um newtype que wrapa `Range { start: Position { line: u32, character: u32 }, end: Position { ... } }`. Substituir por uma struct própria (`TextRange`/`TextPosition`) é mecânico — os campos são idênticos, só muda o namespace.

3. **O que extrair diretamente**: `vault/mod.rs` (Vault, MDFile, Reference, Referenceable, todo o parsing regex, toda a lógica de matching de referências), `vault/metadata.rs` (zero acoplamento LSP), `vault/parsing.rs`.

4. **O que descartar**: `main.rs`, `completion/`, `tokens.rs`, `codelens.rs` — são 100% LSP ou entrelaçados demais.

5. **O que adaptar**: `config.rs` (remover 1 param `ClientCapabilities`), `rename.rs` e `diagnostics.rs` (extrair a lógica core de "quais referências atualizar" / "quais estão unresolved", descartar o wiring LSP).

6. **Dependências herdadas**: ~10 crates leves (regex, ropey, rayon, walkdir, itertools, once_cell, pathdiff, serde, serde_yaml, urlencoding). Zero `tokio`, zero `tower-lsp`.

## TL;DR

O markdown-oxide/ tem uma separação **parcial** entre lógica de vault e LSP. O módulo `vault/` contém ~95% da lógica pura de indexação, parsing, e resolução de referências, mas usa `tower_lsp::lsp_types::{Position, Range}` como tipos de posição/range em todas as suas structs core. A camada LSP (`main.rs` + módulos de features) é fina e delega para o vault. **O reaproveitamento é viável com cirurgia moderada** — o ponto central é substituir `MyRange(tower_lsp::lsp_types::Range)` por um tipo próprio e remover a função `to_symbol_informations`.

---

## 1. Mapeamento de Camadas

### Camada Core (Lógica de Vault)

| Arquivo | Tamanho | Responsabilidade |
|---------|---------|-----------------|
| [vault/mod.rs](markdown-oxide/src/vault/mod.rs) | ~114KB, 3351 linhas | **O coração**: struct `Vault`, `MDFile`, `Reference`, `Referenceable`, parsing de links/tags/headings, resolução de referências, grafo de backlinks |
| [vault/parsing.rs](markdown-oxide/src/vault/parsing.rs) | 6KB | Parsing de code blocks (regex-based) |
| [vault/metadata.rs](markdown-oxide/src/vault/metadata.rs) | 5KB | Parsing de frontmatter YAML (aliases, tags) — **zero acoplamento LSP** |
| [config.rs](markdown-oxide/src/config.rs) | 7KB | Settings/configuração do vault — **um import LSP** (`ClientCapabilities`) |
| [daily.rs](markdown-oxide/src/daily.rs) | ~200B | Helper para formato de daily notes |

### Camada LSP (Handlers/Protocolo)

| Arquivo | Tamanho | Responsabilidade |
|---------|---------|-----------------|
| [main.rs](markdown-oxide/src/main.rs) | 35KB | Struct `Backend`, impl `LanguageServer`, todo o wiring LSP |
| [completion/](markdown-oxide/src/completion) | ~73KB total | Completions — fortemente acoplado a `CompletionItem` |
| [gotodef.rs](markdown-oxide/src/gotodef.rs) | 1.5KB | Goto definition → retorna `Vec<Location>` |
| [references.rs](markdown-oxide/src/references.rs) | 1.5KB | Find references → retorna `Vec<Location>` |
| [rename.rs](markdown-oxide/src/rename.rs) | 11KB | Rename → retorna `WorkspaceEdit` |
| [hover.rs](markdown-oxide/src/hover.rs) | 800B | Hover → retorna `Hover` |
| [codeactions.rs](markdown-oxide/src/codeactions.rs) | 7KB | Code actions → retorna `Vec<CodeActionOrCommand>` |
| [codelens.rs](markdown-oxide/src/codelens.rs) | 3KB | Code lens → retorna `Vec<CodeLens>` |
| [diagnostics.rs](markdown-oxide/src/diagnostics.rs) | 2.5KB | Diagnostics → retorna `Vec<Diagnostic>` |
| [symbol.rs](markdown-oxide/src/symbol.rs) | 10KB | Symbols → retorna `Vec<SymbolInformation>`, `DocumentSymbolResponse` |
| [tokens.rs](markdown-oxide/src/tokens.rs) | 3KB | Semantic tokens → retorna `SemanticTokensResult` |
| [commands.rs](markdown-oxide/src/commands.rs) | 6KB | Daily note jump — usa `tower_lsp::Client` diretamente |
| [ui.rs](markdown-oxide/src/ui.rs) | 3KB | Preview/hover formatting → retorna `MarkupContent` |

### Separação: Existe mas é Imperfeita

A separação segue este padrão:

```
main.rs (LanguageServer handler)
  → chama módulo de feature (gotodef, references, etc.)
    → que chama vault.select_*() para obter dados
    → e converte para tipos LSP
```

> [!IMPORTANT]
> O problema é que **a camada vault já usa tipos LSP internamente** — especificamente `tower_lsp::lsp_types::Position` e `tower_lsp::lsp_types::Range` — como seus tipos de posicionamento. Isso permeia *todas* as structs core do vault.

---

## 2. Estrutura de Dados Core

### O Vault (Coleção de Notas)

```rust
// vault/mod.rs L163-170
pub struct Vault {
    pub md_files: MyHashMap<MDFile>,  // PathBuf → MDFile
    pub ropes: MyHashMap<Rope>,       // PathBuf → Rope (text buffer)
    root_dir: PathBuf,
}
```

### Uma Nota Individual (MDFile)

```rust
// vault/mod.rs L649-660
pub struct MDFile {
    pub references: Vec<Reference>,
    pub headings: Vec<MDHeading>,
    pub indexed_blocks: Vec<MDIndexedBlock>,
    pub tags: Vec<MDTag>,
    pub footnotes: Vec<MDFootnote>,
    pub path: PathBuf,
    pub link_reference_definitions: Vec<MDLinkReferenceDefinition>,
    pub metadata: Option<MDMetadata>,
    pub codeblocks: Vec<MDCodeBlock>,
}
```

### Links/Referências

```rust
// vault/mod.rs L796-807
pub enum Reference {
    Tag(ReferenceData),
    WikiFileLink(ReferenceData),
    WikiHeadingLink(ReferenceData, File, Specialref),
    WikiIndexedBlockLink(ReferenceData, File, Specialref),
    MDFileLink(ReferenceData),
    MDHeadingLink(ReferenceData, File, Specialref),
    MDIndexedBlockLink(ReferenceData, File, Specialref),
    Footnote(ReferenceData),
    LinkRef(ReferenceData),
}

// vault/mod.rs L779-784
pub struct ReferenceData {
    pub reference_text: String,
    pub display_text: Option<String>,
    pub range: MyRange,              // ⚠️ Wraps tower_lsp::lsp_types::Range
}
```

### Referenceables (O Que Pode Ser Referenciado)

```rust
// vault/mod.rs L1537-1549
pub enum Referenceable<'a> {
    File(&'a PathBuf, &'a MDFile),
    Heading(&'a PathBuf, &'a MDHeading),
    IndexedBlock(&'a PathBuf, &'a MDIndexedBlock),
    Tag(&'a PathBuf, &'a MDTag),
    Footnote(&'a PathBuf, &'a MDFootnote),
    UnresovledFile(PathBuf, &'a String),
    UnresolvedHeading(PathBuf, &'a String, &'a String),
    UnresovledIndexedBlock(PathBuf, &'a String, &'a String),
    LinkRefDef(&'a PathBuf, &'a MDLinkReferenceDefinition),
}
```

### O Range Wrapper (O Ponto Central do Acoplamento)

```rust
// vault/mod.rs L1233-1282
pub struct MyRange(pub tower_lsp::lsp_types::Range);  // ⚠️ AQUI

impl MyRange {
    pub fn from_range(rope: &Rope, range: Range<usize>) -> MyRange {
        // converte byte offset → line/column usando ropey
        tower_lsp::lsp_types::Range {
            start: Position { line: start_line as u32, character: start_offset as u32 },
            end: Position { line: end_line as u32, character: end_offset as u32 },
        }.into()
    }
}

impl Deref for MyRange {
    type Target = tower_lsp::lsp_types::Range;  // ⚠️
}
```

### Headings, Tags, Indexed Blocks

```rust
pub struct MDHeading {
    pub heading_text: String,
    pub range: MyRange,           // ⚠️
    pub level: HeadingLevel,
}

pub struct MDTag {
    pub tag_ref: String,
    pub range: MyRange,           // ⚠️
}

pub struct MDIndexedBlock {
    pub index: String,
    pub range: MyRange,           // ⚠️
}

pub struct MDFootnote {
    pub index: String,
    pub footnote_text: String,
    pub range: MyRange,           // ⚠️
}
```

### Metadata (Frontmatter) — **Limpa!**

```rust
// vault/metadata.rs L6-12
pub struct MDMetadata {
    aliases: Vec<String>,
    tags: Vec<String>,
}
```

> [!TIP]
> `MDMetadata` e o parsing de frontmatter são **completamente livres de LSP**. Extraíveis diretamente.

### O Grafo de Backlinks

Não existe como struct dedicada. O grafo é computado on-demand pelos métodos do `Vault`:

```rust
// Vault methods (query interface)
vault.select_references(path)                    // → Vec<(&Path, &Reference)>
vault.select_referenceable_nodes(path)           // → Vec<Referenceable>
vault.select_references_for_referenceable(&ref)  // → Vec<(&Path, &Reference)>  ← BACKLINKS
vault.select_referenceables_for_reference(&ref)  // → Vec<Referenceable>       ← FORWARD LINKS
vault.select_reference_at_position(path, pos)    // → Option<&Reference>
vault.select_referenceable_at_position(path, pos)// → Option<Referenceable>
```

---

## 3. Acoplamento LSP — Grau de Cirurgia

### Tabela de Acoplamento

| Módulo | Importa `tower-lsp`? | Retorna tipos LSP? | Acoplamento Core | Veredicto |
|--------|----------------------|--------------------|--------------------|-----------|
| **vault/mod.rs** | ✅ `Position, Range, Location, SymbolInformation, SymbolKind, Url` | ✅ `MyRange` wrapa `lsp_types::Range`; `to_symbol_informations()` retorna `Vec<SymbolInformation>` | **ESTRUTURAL** — `MyRange` permeia todas as structs | 🔴 **Cirurgia obrigatória** |
| **vault/parsing.rs** | ❌ diretamente, mas usa `MyRange` (que wrapa LSP) | Indireto via `MyRange` | Indireto | 🟡 Resolve automaticamente com vault/mod.rs |
| **vault/metadata.rs** | ❌ | ❌ | **ZERO** | 🟢 Extraível diretamente |
| **config.rs** | ✅ `ClientCapabilities` (1 uso em `Settings::new()`) | Param de input | Pontual (1 param) | 🟢 Remoção trivial |
| **gotodef.rs** | ✅ `Location, Position, Url` | ✅ `Vec<Location>` | Tradução pura | 🟡 Camada fina, descartável |
| **references.rs** | ✅ `Location, Position, Url` | ✅ `Vec<Location>` | Tradução pura | 🟡 Camada fina, descartável |
| **rename.rs** | ✅ (12 tipos LSP) | ✅ `WorkspaceEdit` | **ENTRELAÇADO** | 🔴 Lógica de rename misturada com construção de LSP edits |
| **hover.rs** | ✅ `Hover, HoverContents, HoverParams` | ✅ `Hover` | Tradução + param | 🟡 Fino |
| **diagnostics.rs** | ✅ `Diagnostic, Url` | Parte core (`path_unresolved_references`) retorna tipos próprios; `diagnostics()` retorna `Vec<Diagnostic>` | **Misto** | 🟡 `path_unresolved_references` é core puro |
| **symbol.rs** | ✅ `DocumentSymbol, SymbolInformation, SymbolKind` | ✅ tipos LSP | Lógica pura de tree (`construct_tree`) + mapeamento LSP (`map_to_lsp_tree`) | 🟡 Separável |
| **tokens.rs** | ✅ `SemanticToken, SemanticTokensResult` | ✅ | LSP puro | 🔵 Descartável |
| **commands.rs** | ✅ `tower_lsp::Client`, `ShowDocumentParams, Url` | ✅ | **ENTRELAÇADO** com `Client` (async IPC) | 🔴 Reescrever |
| **codeactions.rs** | ✅ (12 tipos LSP) | ✅ `Vec<CodeActionOrCommand>` | **ENTRELAÇADO** | 🔴 Reescrever |
| **codelens.rs** | ✅ `CodeLens, Command, Location, Url` | ✅ `Vec<CodeLens>` | **ENTRELAÇADO** | 🔴 Descartável (feature visual) |
| **ui.rs** | ✅ `MarkupContent, MarkupKind` | ✅ `MarkupContent` | Tradução | 🟡 Logica core (string formatting) extraível |
| **completion/** | ✅ massivamente | ✅ `CompletionResponse` | **PROFUNDAMENTE ENTRELAÇADO** | 🔴 Reescrever para ferrum |
| **main.rs** | ✅ massivamente | `LanguageServer` impl | **100% LSP** | 🔵 Completamente descartável |

### Resumo do Acoplamento

O acoplamento com LSP no vault se resume a **dois pontos**:

1. **`MyRange` = wrapper de `tower_lsp::lsp_types::Range`** → Usado em TODAS as structs core (`MDHeading`, `MDTag`, `MDIndexedBlock`, `MDFootnote`, `MDCodeBlock`, `ReferenceData`, `Block`). Os métodos de query do Vault também usam `Position` como parâmetro.

2. **`to_symbol_informations()`** → Método no `Vault` que retorna `Vec<SymbolInformation>` diretamente. É o único método do vault que retorna tipos LSP complexos.

---

## 4. Ponto de Entrada da Lógica

### Construção do Vault

```rust
// vault/mod.rs L25-79
impl Vault {
    pub fn construct_vault(context: &Settings, root_dir: &Path) -> Result<Vault, std::io::Error>
}
```

**Input**: `Settings` (configuração) + `root_dir` (caminho da vault)  
**Output**: `Vault` completo com todos os arquivos indexados

**O que faz**:
1. Walka o diretório com `WalkDir`, filtrando `.md`
2. Exclui pastas ocultas, `.obsidian`, `logseq`, e pastas configuradas
3. Em paralelo (`rayon`), lê cada arquivo e cria `MDFile::new()`
4. Em paralelo, cria `Rope` para cada arquivo (text buffer)
5. Retorna `Vault { md_files, ropes, root_dir }`

### Construção de um MDFile

```rust
// vault/mod.rs L663
impl MDFile {
    fn new(context: &Settings, text: &str, path: PathBuf) -> MDFile
}
```

**O que faz** (tudo regex-based, zero dependência de tree-sitter):
1. Parseia code blocks → `MDCodeBlock::new(text)`
2. Parseia referências (wiki links, MD links, tags, footnotes) → `Reference::new(text, file_name)`
3. Parseia headings → `MDHeading::new(text)`
4. Parseia indexed blocks → `MDIndexedBlock::new(text)`
5. Parseia footnotes → `MDFootnote::new(text)`
6. Parseia frontmatter → `MDMetadata::new(text)`
7. Filtra referências/tags dentro de code blocks

### Atualização Incremental

```rust
// vault/mod.rs L81
pub fn update_vault(context: &Settings, old: &mut Vault, new_file: (&PathBuf, &str))
```

Atualiza um único arquivo no vault existente (re-parseia e substitui).

---

## 5. Veredicto de Reaproveitamento

### A lógica core pode ser extraída como crate independente?

**Sim, com cirurgia moderada.** A lógica de indexação, parsing, e resolução de referências vive essencialmente em `vault/mod.rs` + `vault/metadata.rs` + `vault/parsing.rs`. Toda a lógica de matching de referências, construção do grafo, e query é independente de LSP em termos de *lógica*, mas dependente em termos de *tipos* (`Position`, `Range`).

### Esforço Estimado

**Cirurgia moderada** — não é nem extração limpa nem reescrita. Estimo 3-5 dias de trabalho focado:

#### O que extrair diretamente (zero refatoração):
- `vault/metadata.rs` — intacto
- `MDMetadata`, `MDFile::new()` parsing logic, `Reference::new()` regex logic
- Toda a lógica de matching em `Reference::references()` e `Referenceable::matches_reference()`
- `get_obsidian_ref_path()`, `heading_to_slug()`, `matches_path_or_file()`

#### O que exige cirurgia pontual (1-2 dias):
- **`MyRange`**: Substituir `tower_lsp::lsp_types::Range` por uma struct própria:
  ```rust
  // ferrum-knowledge
  pub struct TextRange {
      pub start: TextPosition,
      pub end: TextPosition,
  }
  pub struct TextPosition {
      pub line: u32,
      pub character: u32,
  }
  ```
  → `MyRange::from_range()` continua usando `ropey` internamente, só muda o tipo de saída.
  → Todos os uses de `position.line` / `range.start.character` continuam funcionando.
  → São os mesmos campos (`line: u32`, `character: u32`), só muda o namespace.

- **`to_symbol_informations()`**: Remover do vault, mover para camada de adaptação LSP.

- **`select_referenceable_at_position()` e `select_reference_at_position()`**: Mudar o param `Position` de `tower_lsp::lsp_types::Position` para `TextPosition`.

- **`config.rs`**: Remover o param `ClientCapabilities` de `Settings::new()` — é usado apenas para detectar se o editor suporta semantic tokens.

#### O que descartar/reescrever:
- `main.rs` — 100% LSP boilerplate
- `completion/` — lógica de completions é feature de editor, não de knowledge graph
- `rename.rs` — a lógica de "quais referências atualizar" é core, mas a construção de `TextDocumentEdit`/`WorkspaceEdit` é LSP. Extrair a lógica, descartar o wiring.
- `commands.rs` — daily notes jump via `tower_lsp::Client`
- `tokens.rs`, `codelens.rs` — features puramente visuais de editor

### Dependências Bloqueantes?

> [!IMPORTANT]
> **Não há dependência bloqueante.** O acoplamento com `tower_lsp::lsp_types` é exclusivamente nos tipos `Position` e `Range`, que são structs simples com campos `u32`. Substituí-los é mecânico — search-and-replace de tipo, sem mudança de lógica.

### O que vale reescrever vs. extrair

| Parte | Ação | Razão |
|-------|------|-------|
| `Vault`, `MDFile`, `Reference`, `Referenceable` | **Extrair** com type substitution | A lógica é sólida e testada |
| `MyRange` / posições | **Substituir tipo** | Mecânico, ~50 ocorrências |
| `vault/metadata.rs` | **Extrair intacto** | Zero acoplamento |
| `vault/parsing.rs` | **Extrair** com type substitution | Só usa `MyRange` |
| Parsing de links (regexes) | **Extrair** | ~400 linhas de regex battle-tested |
| `Reference::references()` (matching) | **Extrair** | ~100 linhas de lógica de matching |
| `construct_vault()` | **Extrair** | Entry point limpo |
| `Settings` | **Adaptar** | Remover `ClientCapabilities` param |
| `diagnostics::path_unresolved_references()` | **Extrair** | Retorna tipos próprios |
| Completion, rename, codeactions | **Reescrever** para API de ferrum | Entrelaçados com LSP edits |
| main.rs, tokens.rs, codelens.rs | **Descartar** | 100% LSP |

---

## 6. Dependências Externas

### Dependências da Lógica Core (herda o ferrum-knowledge)

| Crate | Uso | Essencial? |
|-------|-----|-----------|
| `regex` | Parsing de wikilinks, tags, headings, code blocks | ✅ Sim |
| `ropey` | Text rope (byte↔char↔line conversion para ranges) | ✅ Sim |
| `rayon` | Paralelismo (parsing e queries) | ✅ Sim (performance) |
| `itertools` | `.collect_vec()`, `.unique_by()`, etc. | ✅ Sim (ergonomia) |
| `walkdir` | Scan de diretório do vault | ✅ Sim |
| `once_cell` | `Lazy` para compilação de regexes | ✅ Sim |
| `pathdiff` | `diff_paths()` para ref paths relativos | ✅ Sim |
| `serde` + `serde_yaml` | Parsing de frontmatter YAML | ✅ Sim |
| `serde_json` | Parsing de config files Obsidian | 🟡 Opcional (se não ler .obsidian) |
| `urlencoding` | Decode de URLs em links | ✅ Sim |
| `nucleo-matcher` | Fuzzy matching (workspace symbol) | 🟡 Opcional (feature de search) |

### Dependências Exclusivas do LSP (NÃO herda)

| Crate | Uso |
|-------|-----|
| `tower-lsp` | LSP server framework + `lsp_types` |
| `tokio` | Async runtime para o server LSP |
| `clap` | CLI argument parsing |
| `chrono` + `fuzzydate` | Daily notes (parsing de datas) |
| `config` | Config file reading framework |
| `shellexpand` | Expansão de `~` em paths de config |
| `nanoid` | Geração de IDs (usado em completions) |
| `do-notation` | Macro de monadic notation |

> [!TIP]
> O `ferrum-knowledge` herdaria ~10 dependências de produção, todas lightweight. A mais pesada é `rayon` (~perf-critical). Sem `tokio`, sem `tower-lsp`.

---

## 7. Recomendação Final

### Estratégia: **Extrair + Substituir tipos + Adaptar**

```
┌─────────────────────────────────────────────────┐
│              ferrum-knowledge (nova crate)       │
│                                                 │
│  vault/mod.rs    → Vault, MDFile, Reference,    │
│                    Referenceable, query methods  │
│  vault/parsing.rs → MDCodeBlock                 │
│  vault/metadata.rs → MDMetadata (intacto)       │
│  types.rs        → TextRange, TextPosition      │
│                    (substitui MyRange/Position)  │
│  config.rs       → Settings (sem ClientCaps)    │
│                                                 │
│  API pública:                                   │
│    Vault::construct(settings, root_dir)          │
│    Vault::update(path, text)                     │
│    vault.select_references(path)                 │
│    vault.select_referenceables(path)             │
│    vault.select_references_for_referenceable()   │
│    vault.select_referenceables_for_reference()   │
│    vault.select_reference_at_position(path, pos) │
│    vault.select_referenceable_at_position()      │
│    vault.select_headings(path)                   │
│    vault.select_blocks()                         │
│    vault.select_referenceable_preview()           │
│    vault.root_dir()                              │
│                                                  │
│  Deps: regex, ropey, rayon, itertools, walkdir,  │
│        once_cell, pathdiff, serde, serde_yaml,   │
│        urlencoding                               │
└─────────────────────────────────────────────────┘
```

### Passos concretos:

1. **Criar `ferrum-knowledge` como crate com `vault/` extraído**
2. **Definir `TextRange`/`TextPosition`** como tipos próprios (4 campos `u32`)
3. **Search-and-replace** de `tower_lsp::lsp_types::{Range, Position}` por tipos próprios em vault/
4. **Remover** `to_symbol_informations()` do vault (mover para adapter se necessário)
5. **Remover** `find_range()` helper (usa `tower_lsp::lsp_types::Range` diretamente)
6. **Adaptar** `config.rs` para não depender de `ClientCapabilities`
7. **Opcionalmente**, fornecer `From<TextRange> for lsp_types::Range` numa feature flag para quem quiser usar com LSP

### Estimativa: **3-5 dias** para um dev confortável com Rust.

> [!NOTE]
> O parsing regex-based do markdown-oxide/ é funcional mas frágil. Para o Ferrum que já tem `ferrum-markdown` com tree-sitter, pode fazer sentido eventualmente **substituir** os regexes de parsing por queries tree-sitter — mas isso é uma otimização futura, não um bloqueio para a extração.
