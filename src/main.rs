//! CLI wrapper around the `ice_candidate_parser` crate.
//!
//! Reads candidate lines from the given file (or stdin) and prints a
//! structured breakdown of each one.

use std::io::{self, BufRead, Read};

use ice_candidate_parser::Candidate;

fn main() {
    let arg = std::env::args().nth(1);
    let input = match arg.as_deref() {
        None | Some("-") => {
            let mut s = String::new();
            io::stdin().read_to_string(&mut s).expect("read stdin");
            s
        }
        Some(path) => std::fs::read_to_string(path).unwrap_or_else(|e| {
            eprintln!("ice-candidate-parser: {path}: {e}");
            std::process::exit(1);
        }),
    };

    let mut any = false;
    for line in io::Cursor::new(input).lines().map_while(Result::ok) {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        any = true;
        match line.parse::<Candidate>() {
            Ok(c) => {
                let related = match (&c.related_address, c.related_port) {
                    (Some(a), Some(p)) => format!(" via {a}:{p}"),
                    _ => String::new(),
                };
                println!(
                    "{:<6} {:?} {}:{} (component {}, priority {}){}",
                    c.kind, c.transport, c.address, c.port, c.component, c.priority, related
                );
            }
            Err(e) => eprintln!("skip: {e}: {line}"),
        }
    }

    if !any {
        eprintln!("ice-candidate-parser: no candidate lines on input");
        std::process::exit(1);
    }
}
