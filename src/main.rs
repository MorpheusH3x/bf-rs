//!Brainfuck interpreter in Rust

// Make clippy quite nasty
#![deny(clippy::cargo)] // Checks for garbage in the Cargo TOML files
#![deny(clippy::complexity)] // Checks for needlessly complex structures
#![deny(clippy::correctness)] // Checks for common invalid usage and workarounds
#![deny(clippy::nursery)] // Checks for things that are typically forgotten by learners
#![deny(clippy::pedantic)] // Checks for mildly annoying comments it could make about your code
#![deny(clippy::perf)] // Checks for inefficient ways to perform common tasks
#![deny(clippy::style)] // Checks for inefficient styling of code
#![deny(clippy::suspicious)] // Checks for potentially malicious behaviour
// Add some new clippy lints
#![deny(clippy::use_self)] // Checks for the use of a struct's name in its `impl`
// Add some default lints
#![warn(unused_variables)] // Checks for unused variables
// Deny missing documentation
#![deny(missing_docs)]
#![deny(rustdoc::missing_crate_level_docs)]

use clap::Parser;
use log::{debug, info};
use simple_logger::SimpleLogger;
use std::path::{Path, PathBuf};

/// Brainfuck interpreter in Rust
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "BF FILE")]
    file: Option<PathBuf>,

    #[arg(short, long)]
    debug: bool,
}

#[derive(Debug)]
enum Op {
    Inc(usize),
    Dec(usize),
    Left(usize),
    Right(usize),
    Write(usize),
    Read(usize),
    JumpIfZero(usize),
    JumpIfNonZero(usize),
}


fn lex(source: &str) -> Vec<Op> {
    let mut ops = Vec::new();
    let mut count = 1;

    for (index, symbol) in source.chars().enumerate() {
        match symbol {
            '>' => {
                if let Some(next) = source.chars().nth(index + 1) {
                    if next == symbol {
                        count += 1;
                    } else {
                        ops.push(Op::Right(count));
                        count = 1;
                    }
                } else {
                    ops.push(Op::Right(count));
                }
            }
            '<' => {
                if let Some(next) = source.chars().nth(index + 1) {
                    if next == symbol {
                        count += 1;
                    } else {
                        ops.push(Op::Left(count));
                        count = 1;
                    }
                } else {
                    ops.push(Op::Left(count));
                }
            }
            '+' => {
                if let Some(next) = source.chars().nth(index + 1) {
                    if next == symbol {
                        count += 1;
                    } else {
                        ops.push(Op::Inc(count));
                        count = 1;
                    }
                } else {
                    ops.push(Op::Inc(count));
                }
            }
            '-' => {
                if let Some(next) = source.chars().nth(index + 1) {
                    if next == symbol {
                        count += 1;
                    } else {
                        ops.push(Op::Dec(count));
                        count = 1;
                    }
                } else {
                    ops.push(Op::Dec(count));
                }
            }
            '.' => {
                if let Some(next) = source.chars().nth(index + 1) {
                    if next == symbol {
                        count += 1;
                    } else {
                        ops.push(Op::Write(count));
                        count = 1;
                    }
                } else {
                    ops.push(Op::Write(count));
                }
            }
            ',' => {
                if let Some(next) = source.chars().nth(index + 1) {
                    if next == symbol {
                        count += 1;
                    } else {
                        ops.push(Op::Read(count));
                        count = 1;
                    }
                } else {
                    ops.push(Op::Read(count));
                }
            }
            '[' => ops.push(Op::JumpIfZero(0)),
            ']' => ops.push(Op::JumpIfNonZero(0)),
            _ => (),
        }
    }

    ops
}

fn main() {
    SimpleLogger::new().init().unwrap();
    let cli = Cli::parse();

    info!("Debug mode: {0}", cli.debug);
    debug!(
        "Executing file: {0}",
        cli.file
            .as_deref()
            .unwrap_or_else(|| Path::new(""))
            .display()
    );

    if let Some(file_path) = cli.file.as_deref() {
        let code = std::fs::read_to_string(file_path).unwrap();
        debug!("Executing code:\n{code}");
        let opcodes = lex(&code);
        debug!("Executing opcodes:\n{:?}", opcodes);
    }
}
