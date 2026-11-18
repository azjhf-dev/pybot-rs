# pybot-rs

A Rust port of [PyBot](https://github.com/huggingface/pybot): a chatbot with **no weights, no training, just rules** - pattern matching, lookup tables and if-else branches, the way ELIZA worked sixty years ago.

The original PyBot is ~640k lines of Python written by a swarm of 1,000+ agents at Hugging Face, and it holds its own against SmolLM-360M-Instruct on SmolTalkBench (26% win rate). This repo asks the obvious follow-up: what does PyBot look like in Rust?

## What this is

A **partial port**: the full runtime architecture, ported faithfully, plus a curated subset of the rule modules. The runtime contract is the thing that makes PyBot PyBot, and it is all here:

- **Modules** with `observe` / `respond` / `after` hooks and a `PRIORITY`, sorted highest-priority-first (upstream: `bot/runtime.py`).
- **Per-module memory copies**: every `respond` works on its own deep copy of the conversation memory, so a reply that isn't chosen leaves no trace. The chosen module's copy becomes the conversation's memory.
- **An arbiter** that picks one candidate (upstream's is 5.5k lines of tie-breaking; the runtime default - highest priority wins - plus the fallback-never-outvotes rule is what's reproduced here).
- **A turn never fails**: a module that errors is skipped, and with no candidates the bot says `Hello!`.
- **The same CLI**: interactive chat with the PyBot logo, `/reset`, `/help`, `/exit`, `-p` one-shot, and pipe mode (one reply per line).

## What's ported vs. what's not

| Area | Upstream | This port |
|---|---|---|
| Runtime architecture | `bot/runtime.py` (120 lines, the fixed interface) | Ported 1:1 (`src/runtime.rs`) |
| Module count | 528 files, ~640k lines | 9 modules, ~1k lines |
| Greetings / introductions / name memory | `chat_replies.py`, `memory/facts.py` | Ported, reply pools verbatim; name-reading blocklist is a subset |
| "how are you" / thanks / sign-offs | `chat_replies.py` | Ported, pools verbatim |
| Household knowledge + follow-ups ("how long does that take") | `bot/knowledge/` | 6 entries incl. the README's kettle-limescale demo; upstream has tens of thousands |
| Safety net (wish-to-die, medical red flags) | `safety.py` (12k lines, PRIORITY 9000, never outvoted) | Shape and priority kept; **small subset** of the patterns, fewer exclusions - over-triggers where upstream wouldn't |
| Arbiter | `arbiter.py` (5.5k lines) | Runtime default + fallback rule only |
| Everything else (grief, money, care, travel, draft writing, ...) | hundreds of modules | Not ported |

**Honest bottom line:** this reproduces the architecture and the demo conversations. It is not a 640k-line port and will not match PyBot's SmolTalkBench win rate - that score *is* the 640k lines.

## Try it

```sh
cargo run --release
```

```text
    ____        ____        __
   / __ \__  __/ __ )____  / /_
  / /_/ / / / / __  / __ \/ __/
 / ____/ /_/ / /_/ / /_/ / /_
/_/    \__, /_____/\____/\__/
      /____/

A chatbot written in plain Rust. No weights, no training, just rules. (A Rust port of PyBot.)
Type a message and press Enter. /reset starts a new conversation, /exit quits.

user: hi, i'm Sam
pybot: Nice to meet you, Sam. What can I help with?

user: my kettle is full of limescale, how do i get rid of it?
pybot: Half fill the kettle with equal parts water and white vinegar, ...

user: how long does that take
pybot: Half an hour.

user: what's my name?
pybot: You're Sam.
```

One question and exit, or pipe messages in:

```sh
pybot-rs -p "what's my name?"
echo "how are you?" | pybot-rs
```

`PYBOT_DEBUG=1` (interactive) or `-v` (one-shot) shows which module answered and how long it took - the same `[module: ... | ms]` line as upstream's demo.

## Why Rust is a good fit

The upstream demo says PyBot "takes a few seconds to start, because Python has about 640,000 lines of code to load, and about a second to reply." A compiled rule engine has neither problem: no interpreter warmup, regexes compiled once, modules dispatched through a sorted static registry. This subset starts instantly and answers in microseconds.

## Porting more modules

Each module implements one trait:

```rust
impl Module for MyModule {
    fn name(&self) -> &'static str { "my_module" }
    fn priority(&self) -> i64 { 0 }
    fn observe(&self, text: &str, memory: &mut Memory) { /* remember, never reply */ }
    fn respond(&self, text: &str, memory: &mut Memory) -> Option<String> { /* propose or pass */ }
    fn after(&self, text: &str, reply: &str, memory: &mut Memory) { /* tidy up */ }
}
```

Register it in `src/modules/mod.rs`. The runtime does the rest.

## License and attribution

Apache-2.0, same as upstream. Reply pools, patterns and architecture are adapted from [huggingface/pybot](https://github.com/huggingface/pybot), copyright its authors (Hugging Face) - see [NOTICE](NOTICE). Not affiliated with or endorsed by Hugging Face.
