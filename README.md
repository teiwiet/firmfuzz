# Firmfuzz

A coverage-uninformed binary fuzzer written in Rust, targeting ELF binaries — including cross-architecture targets (ARM/MIPS) via QEMU user-mode emulation.

Built as a learning project to understand fuzzing internals from scratch.

---

## What it does

```
ELF binary
    │
    ▼
┌─────────────┐     Scans for dangerous functions (strcpy, gets, system…)
│  Analyzer   │     and input functions (recv, read, scanf…)
└─────────────┘     Scores binaries by attack surface
        │
        ▼
┌─────────────┐     Generates mutated inputs from seed corpus
│   Mutator   │     Strategies: bit flip, byte flip, append,
└─────────────┘     truncate, repeat, known-bad string injection
        │
        ▼
┌─────────────┐     Executes binary with each mutated input
│   Harness   │     Supports native execution and QEMU user-mode
└─────────────┘     Detects crashes via exit code (SIGSEGV/SIGABRT)
        │
        ▼
┌─────────────┐     Orchestrates the loop, grows corpus on crash,
│    Fuzzer   │     saves crash inputs to disk
└─────────────┘
```

---

## Features

- **ELF static analysis** — detects dangerous and input-handling functions via symbol table parsing (goblin)
- **Multi-arch support** — x86, x86_64, ARM32, ARM64, MIPS via QEMU user-mode
- **Mutation strategies** — bit flip, byte flip, append, truncate, repeat, known-bad injection
- **Crash triage** — distinguishes real crashes (SIGSEGV 139, SIGABRT 134) from normal error exits
- **Corpus growth** — crash-triggering inputs are added back to corpus for further mutation

---

## Usage

### Analyze a binary

```bash
cargo run -- analyze ./path/to/binary
```

### Fuzz a native binary

```rust
// main.rs
let harness = Harness::new("./target_binary", "qemu-mips", ".", 2000, false);
let seeds = vec![b"hello\n".to_vec()];
let mut fuzzer = Fuzzer::new(harness, "./crashes", seeds);
fuzzer.run(100_000);
```

### Fuzz a MIPS binary via QEMU

```rust
let harness = Harness::new("./mips_binary", "qemu-mips", "/path/to/mips/libs", 2000, true);
```

Crash inputs are saved to `./crashes/crash_<iter>_code<signal>`.

---

## Project structure

```
src/
├── main.rs        — Entry point
├── analyzer.rs    — ELF binary analysis
├── mutator.rs     — Input mutation strategies
├── harness.rs     — Process execution and crash detection
└── fuzzer.rs      — Main fuzzing loop
```

---

## Dependencies

| Crate | Purpose |
|---|---|
| `goblin` | ELF parsing |
| `rand` | Random mutation |
| `wait-timeout` | Process timeout handling |
| `rayon` | Parallel binary scanning |
| `serde` | Serialization of reports |
| `walkdir` | Recursive directory traversal |

---

## Limitations

- No coverage feedback (dumb fuzzer) — mutations are random, not guided by code coverage
- No crash deduplication — multiple crash files may represent the same underlying bug
- No persistent mode / fork server — spawns a new process per iteration

---

## Roadmap

- [ ] Add basic block coverage via `__sanitizer_cov_trace_pc_guard`
- [ ] Crash deduplication by signal + stack hash
- [ ] Parallel fuzzing with `rayon`
- [ ] Structured input generation for HTTP/JSON targets

---

## What I learned

- ELF binary format and symbol table structure
- How fuzzers mutate inputs to explore program state
- Process execution and signal handling in Rust
- QEMU user-mode emulation for cross-architecture testing