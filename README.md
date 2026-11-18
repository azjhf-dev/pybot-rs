# pybot-rs

Rust translation of the rule-based [Hugging Face PyBot](https://github.com/huggingface/pybot) source.

## Status: behavior verification in progress

The full `bot/` corpus has been translated: 527 non-root Python modules from 528 Python files, about 637,000 source lines. Each module has a generated Rust crate in `crates/gen_*`. This is no longer the earlier nine-module demo port.

All generated modules compile, and the executable builds. This does **not** establish behavioral equivalence. The Python-semantics support library is purpose-built for this corpus, not a complete Python implementation. Remaining differences are being investigated. Do not rely on this port for medical, crisis, or other safety-critical advice.

The complete safety module and its priority band are represented. The sample gas and self-harm inputs reach safety replies in Rust, while this upstream checkout's Python safety hook raises a regex error and falls through to other modules. These are recorded as differential failures, not parity passes.

## Build and run

```sh
cargo build -j1
cargo run -- -s 42 -p 'hello'
printf 'hello\nhow are you\n' | cargo run -- -s 42
cargo test -p pyrt --lib
```

A first build is large. Use `-j1` on memory-constrained machines. Release-build verification is still pending.

## Layout

- `transpile.py`: AST-to-Rust generator. Reads the sibling `../upstream/bot` checkout.
- `crates/gen_*`: translated modules and their complete rule/data pools.
- `crates/pyrt`: values, collections, regexes, arithmetic, dates, random numbers, exceptions and module linkage.
- `src/runtime.rs`: observer/response/arbiter/after orchestration.
- `differential.py`: seeded conversation comparisons with Python.
- `verification/`: observed test results and current limitations.

Source checkout: `huggingface/pybot` commit `7d63c7fbebb44c7e466e5c8b5c1f341ab767929e`. Integer-seeded random draws have targeted tests. String/float seeding, generators, regex edge cases and other language details are not universally equivalent to CPython.

## License

Apache-2.0. The upstream license is retained in `LICENSE`, with attribution in `NOTICE`. No GitHub Actions workflows are included.
