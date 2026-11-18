//! Chat with PyBot in a terminal. Ported from upstream `pybot.py`.
//!
//!     pybot-rs                            interactive chat
//!     pybot-rs -p "your message"          one reply, then exit
//!     echo "your message" | pybot-rs      one reply per input line

use pybot_rs::memory;
use pybot_rs::pyrt;
use pybot_rs::Runtime;
use std::io::{BufRead, Write};
use std::time::Instant;

const VERSION: &str = "0.2.0";
const SOURCE_COMMIT: &str = "24cbc0439e07fd015c0747ab3be6e782a42b5c5b";

const LOGO: &str = r#"    ____        ____        __
   / __ \__  __/ __ )____  / /_
  / /_/ / / / / __  / __ \/ __/
 / ____/ /_/ / /_/ / /_/ / /_
/_/    \__, /_____/\____/\__/
      /____/"#;
const TAGLINE: &str = "A chatbot written in plain Rust. No weights, no training, just rules.";
const HINT: &str = "Type a message and press Enter. /reset starts a new conversation, /exit quits.";
const HELP: &str = "Commands:
  /reset   clear the screen and start again (PyBot forgets what was said)
  /help    show this help
  /exit    quit (Ctrl+D works too)";

struct Args {
    prompt: Option<String>,
    interactive: bool,
    seed: Option<i64>,
    verbose: bool,
}

fn parse_args() -> Args {
    let mut args = Args { prompt: None, interactive: false, seed: None, verbose: false };
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() {
            "-p" | "--prompt" => args.prompt = it.next(),
            "-i" | "--interactive" => args.interactive = true,
            "-v" | "--verbose" => args.verbose = true,
            "-s" | "--seed" => args.seed = it.next().and_then(|v| v.parse().ok()),
            "--version" => {
                println!("pybot-rs {} (bot at commit {})", VERSION, &SOURCE_COMMIT[..12]);
                std::process::exit(0);
            }
            "--help" | "-h" => {
                println!("usage: pybot-rs [-h] [-p TEXT] [-i] [-s N] [-v] [--version]\n\nChat with PyBot, a chatbot written in plain rules: no model, no weights.\n\nWith no options, PyBot starts an interactive chat. When input is piped in, it\nprints one reply per input line.");
                std::process::exit(0);
            }
            other if other.starts_with("-p") && other.len() > 2 => args.prompt = Some(other[2..].to_string()),
            _ => {}
        }
    }
    args
}

struct TurnResult {
    reply: String,
    module: Option<String>,
    seconds: f64,
}

fn answer(rt: &Runtime, text: &str, memory: &mut pybot_rs::memory::Memory) -> TurnResult {
    // Mirrors pybot.py answer(): the reply is one non-empty line, and a turn never raises.
    let start = Instant::now();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let t = rt.turn(text, memory);
        (t.reply, t.module)
    }));
    let seconds = start.elapsed().as_secs_f64();
    match result {
        Ok((reply, module)) => {
            let reply = reply.split_whitespace().collect::<Vec<_>>().join(" ");
            let reply = if reply.is_empty() { "Hello!".to_string() } else { reply };
            TurnResult { reply, module, seconds }
        }
        Err(_) => TurnResult { reply: "Hello!".to_string(), module: None, seconds },
    }
}

fn details(module: &Option<String>, seconds: f64) -> String {
    format!("[module: {} | {:.0} ms]", module.as_deref().unwrap_or("none"), seconds * 1000.0)
}

fn opening() {
    println!("{}\n", LOGO);
    println!("{}", TAGLINE);
    println!("{}\n", HINT);
}

fn main() {
    pyrt::install_hook();
    let args = parse_args();
    if let Some(seed) = args.seed {
        pyrt::pyrandom::seed(&pyrt::value::Py::Int(seed));
    }
    let stdin = std::io::stdin();
    let interactive = (args.prompt.is_none() && std::io::IsTerminal::is_terminal(&stdin)) || (args.prompt.is_some() && args.interactive);
    if interactive {
        eprint!("Loading PyBot...");
        std::io::stderr().flush().ok();
    }
    let rt = Runtime::new();
    if interactive {
        eprint!("\r                \r");
        opening();
        let mut memory = memory::new_memory();
        let mut first = args.prompt.clone();
        loop {
            let text = match first.take() {
                Some(t) => { println!("user: {}", t); t }
                None => {
                    print!("user: ");
                    std::io::stdout().flush().ok();
                    let mut line = String::new();
                    match stdin.lock().read_line(&mut line) {
                        Ok(0) | Err(_) => { println!(); break; }
                        Ok(_) => line.trim_end_matches(['\r', '\n']).to_string(),
                    }
                }
            };
            let command = text.trim().to_lowercase();
            if command == "/exit" || command == "/quit" || command == "/bye" { break; }
            if command == "/reset" {
                memory = memory::new_memory();
                if let Some(seed) = args.seed { pyrt::pyrandom::seed(&pyrt::value::Py::Int(seed)); }
                println!("New conversation.\n");
                continue;
            }
            if command == "/help" { println!("{}\n", HELP); continue; }
            if command.is_empty() { continue; }
            let r = answer(&rt, &text, &mut memory);
            println!("pybot: {}", r.reply);
            if args.verbose { println!("{}", details(&r.module, r.seconds)); }
            println!();
        }
    } else if let Some(prompt) = args.prompt {
        let mut memory = memory::new_memory();
        let r = answer(&rt, &prompt, &mut memory);
        println!("{}", r.reply);
        if args.verbose { eprintln!("{}", details(&r.module, r.seconds)); }
    } else {
        let mut memory = memory::new_memory();
        for line in stdin.lock().lines() {
            let Ok(line) = line else { break };
            let r = answer(&rt, line.trim_end_matches(['\r', '\n']), &mut memory);
            println!("{}", r.reply);
            std::io::stdout().flush().ok();
            if args.verbose { eprintln!("{}", details(&r.module, r.seconds)); }
        }
    }
}
