//! Chat with PyBot in a terminal. Ported from upstream `pybot.py`.
//!
//!     pybot-rs                            interactive chat
//!     pybot-rs -p "your message"          one reply, then exit
//!     echo "your message" | pybot-rs      one reply per input line

use pybot_rs::memory;
use pybot_rs::Runtime;
use std::io::{BufRead, IsTerminal, Write};
use std::time::Instant;

const LOGO: &str = r"
    ____        ____        __
   / __ \__  __/ __ )____  / /_
  / /_/ / / / / __  / __ \/ __/
 / ____/ /_/ / /_/ / /_/ / /_
/_/    \__, /_____/\____/\__/
      /____/";

const TAGLINE: &str =
    "A chatbot written in plain Rust. No weights, no training, just rules. (A Rust port of PyBot.)";
const USER_PREFIX: &str = "user: ";
const BOT_PREFIX: &str = "pybot: ";
const HINT: &str = "Type a message and press Enter. /reset starts a new conversation, /exit quits.";
const HELP: &str = "Commands:
  /reset   clear the screen and start again (PyBot forgets what was said)
  /help    show this help
  /exit    quit (Ctrl+D works too)";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let runtime = Runtime::new();

    // -p "message": one reply, then exit.
    if let Some(pos) = args.iter().position(|a| a == "-p" || a == "--prompt") {
        let prompt = args.get(pos + 1).cloned().unwrap_or_default();
        let mut memory = memory::new_memory();
        let start = Instant::now();
        let turn = runtime.turn(&prompt, &mut memory);
        println!("{}", turn.reply);
        if args.iter().any(|a| a == "-v" || a == "--verbose") {
            eprintln!(
                "[module: {} | {:.0} ms]",
                turn.module.unwrap_or("none"),
                start.elapsed().as_secs_f64() * 1000.0
            );
        }
        return;
    }

    let stdin = std::io::stdin();
    if !stdin.is_terminal() {
        // Piped input: one reply per line, nothing else.
        let mut memory = memory::new_memory();
        for line in stdin.lock().lines() {
            match line {
                Ok(text) => {
                    let text = text.trim().to_string();
                    if text.is_empty() {
                        continue;
                    }
                    println!("{}", runtime.reply(&text, &mut memory));
                }
                Err(_) => break,
            }
        }
        return;
    }

    // Interactive chat.
    println!("{}", LOGO);
    println!();
    println!("{}", TAGLINE);
    println!("{}", HINT);
    println!();

    let mut memory = memory::new_memory();
    loop {
        print!("{}", USER_PREFIX);
        let _ = std::io::stdout().flush();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) => {
                // Ctrl+D
                println!();
                break;
            }
            Ok(_) => {}
            Err(_) => break,
        }
        let text = line.trim();
        match text {
            "/exit" | "/quit" => break,
            "/help" => {
                println!("{}", HELP);
                continue;
            }
            "/reset" => {
                print!("\x1b[2J\x1b[H");
                memory = memory::new_memory();
                println!("{}", LOGO);
                println!();
                println!("{}", TAGLINE);
                println!("{}", HINT);
                println!();
                continue;
            }
            "" => continue,
            _ => {}
        }
        let start = Instant::now();
        let turn = runtime.turn(text, &mut memory);
        println!("{}{}", BOT_PREFIX, turn.reply);
        if std::env::var("PYBOT_DEBUG").is_ok() {
            println!(
                "[module: {} | {:.0} ms]",
                turn.module.unwrap_or("none"),
                start.elapsed().as_secs_f64() * 1000.0
            );
        }
    }
}
