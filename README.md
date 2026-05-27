# 🧠 MEGA-BASIC — Interpreter with Custom Syntax

A toy programming language interpreter written in **Rust**, built from scratch to understand how lexers, parsers, and interpreters actually work.

The twist: **the syntax is fully swappable**. Write the same program in English, Russian, Emoji, or Crab 🦀 — it all runs the same way under the hood.

---

## ✨ Features

- **Multi-syntax support** — swap keyword dictionaries at runtime
- **Pratt parser** — correct operator precedence with prefix, infix, and postfix operators
- **Variables** — assign, read, and reuse
- **Math** — `+` `-` `*` `/` `^` `!` with full precedence, unary minus, factorial, and parentheses
- **Conditionals** — `IF ... THEN ...` with `==`, `!=`, `<`, `>`
- **Labels & GOTO** — `Turing-complete` control flow
- **String printing** — `PRINT "hello"`
- **Random numbers** — `RANDOM x 1 100`
- **Run from file** — pass a `.bas` file as CLI argument
- **Zero-copy lexer** — operates on `&str` slices with Rust lifetimes, no unnecessary allocations

---

## Syntax Dialects

| Feature   | English   | Russian  | Emoji | Crab  |
|-----------|-----------|----------|-------|-------|
| Assign    | `LET`     | `ПУСТЬ`  | `✍`  | `🦀`  |
| Print     | `PRINT`   | `ПЕЧАТЬ` | `🖨`  | `📢`  |
| Input     | `INPUT`   | `ВВОД`   | `⌨`  | `⚓`  |
| If        | `IF`      | `ЕСЛИ`   | `❓`  | `🌊`  |
| Then      | `THEN`    | `ТО`     | `➡`  | `🚢`  |
| Goto      | `GOTO`    | `ИДИ`    | `🚀`  | `🚀`  |
| Random    | `RANDOM`  | `РАНДОМ` | `🎲`  | `🎲`  |
| End       | `END`     | `СТОП`   | `⛔`  | `⛔`  |

---

## 📝 Example Programs

### English
```
#mode "ENGLISH"
RANDOM SECRET 1 100
LET TRIES = 0
PRINT "--- GUESS THE NUMBER GAME ---"
:game_loop
PRINT "Enter your guess:"
INPUT GUESS
LET TRIES = TRIES + 1
IF GUESS == SECRET THEN GOTO win
IF GUESS < SECRET THEN GOTO too_low
IF GUESS > SECRET THEN GOTO too_high
:too_low
PRINT "Too low! Try again."
GOTO game_loop
:too_high
PRINT "Too high! Try again."
GOTO game_loop
:win
PRINT "YOU WIN!!!"
PRINT "Total tries:"
PRINT TRIES
```

### Russian
```
#mode "RUSSIAN"
ПУСТЬ Х = 15
ПУСТЬ У = 10
ЕСЛИ Х != У ТО ПЕЧАТЬ Х
ПЕЧАТЬ У
```

### Emoji
```
#mode "EMOJI"
✍ X = 10
✍ Y = 5
❓ X > Y ➡ 🖨 X
🖨 Y
```

### Crab 🦀
```
#mode "CRAB"
🦀 X = 42
📢 "крабы захватили мир"
📢 X
```

---

## ⚙️ Math Operations

| Operator | Description       | Example        |
|----------|-------------------|----------------|
| `+`      | Addition          | `LET x = 2 + 3`|
| `-`      | Subtraction       | `LET x = 5 - 1`|
| `*`      | Multiplication    | `LET x = 4 * 3`|
| `/`      | Division          | `LET x = 8 / 2`|
| `^`      | Power (right-assoc) | `LET x = 2^8`|
| `!`      | Factorial (postfix) | `LET x = 5!` |
| `-x`     | Unary minus       | `LET x = -5`   |
| `()`     | Grouping          | `LET x = (2+3)*4`|

---

## 🚀 Running

### From source

```bash
git clone https://github.com/XsamuraiX228/MEGA-BASIC-WITH-CUSTOM-SYNTAX
cd MEGA-BASIC-WITH-CUSTOM-SYNTAX
cargo run
```

### Example file

Save a program as `game.bas` to folder `FILES` and run:
```bash
cargo run
```

---

## 🏗️ Architecture

```
src/
├── main.rs               # CLI entrypoint, reads file and calls run()
├── lib.rs                # Public API: set_dict, create_lexer, run
└── main_logic/
    ├── lexer.rs          # Tokenizer — splits source into Tokens<'a>
    ├── parser.rs         # Pratt parser — builds AST (OperationTree, Command)
    ├── interpreter.rs    # Executes commands, manages env, handles GOTO
    └── syntaxd.rs        # Keyword dictionaries (English, Russian, Emoji, Crab)
└── io/
    └── scanner.rs
```

The pipeline is:

```
source &str
   └─► Lexer  →  Vec<Tokens<'a>>
         └─► Parser  →  Vec<Command<'a>>
               └─► Interpreter  →  output
```

All stages share the same lifetime `'a` tied to the original source string — no unnecessary cloning, no garbage collector needed.

---

## 🧩 How Custom Syntax Works

Each dialect is just a `HashMap<String, KeyWordType>`:

```rust
fn crab_style() -> SyntaxDict {
    let mut keywords = HashMap::new();
    keywords.insert("🦀".to_string(), KeyWordType::Let);
    keywords.insert("📢".to_string(), KeyWordType::Print);
    // ...
    SyntaxDict { keywords }
}
```

The lexer looks up every word/emoji in this map. The parser and interpreter are completely unaware of which dialect is running — they only see `KeyWordType` variants.

## 📄 License

MIT
