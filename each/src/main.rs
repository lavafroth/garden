use anyhow::{Context, Result};
use std::io::Write;
use std::process::exit;
use std::{io::stdin, process::Command};

fn usage() {
    let usage_str = r#"
USAGE:
    each [SUBCOMMAND] [COMMAND]

DESCRIPTION:
    Processes standard input line by line, executing the specified command for each entry.

SUBCOMMANDS:
    into    Pipes each line from stdin into the standard input of the command.
            Example: cat list.txt | each into grep 'pattern'

    over    Appends each line from stdin as a trailing argument to the command.
            Example: cat list.txt | each over echo "Item:"

EXAMPLES:
    cat names.txt | each into wc -c
    cat files.txt | each over rm
"#;
    eprintln!("{usage_str}");
}

pub enum Approach {
    Into,
    Over,
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let Some(approach) = args.next() else {
        usage();
        exit(1);
    };
    let Some(program) = args.next() else {
        usage();
        exit(1);
    };

    let args: Vec<_> = args.collect();
    let approach = match approach.as_str() {
        "into" => Approach::Into,
        "over" => Approach::Over,
        "help" => {
            usage();
            exit(0)
        }
        _ => {
            usage();
            exit(1);
        }
    };

    for line in stdin().lines() {
        let Ok(line) = line else {
            usage();
            exit(1);
        };
        let mut command = Command::new(&program);
        command.args(&args);

        let mut child = match approach {
            Approach::Into => {
                let mut child = command
                    .stdin(std::process::Stdio::piped())
                    .spawn()
                    .with_context(|| format!("failed to spawn process: {command:?}"))?;

                if let Some(mut stdin) = child.stdin.take() {
                    let repr = &line[0..line.len().min(16)];
                    stdin.write_all(line.as_bytes()).with_context(|| {
                        format!("failed to write {repr:?} to stdin of child process: {command:?}")
                    })?;
                }
                child
            }
            Approach::Over => command
                .arg(line)
                .spawn()
                .with_context(|| format!("failed to spawn process: {command:?}"))?,
        };

        child
            .wait()
            .with_context(|| format!("failed to wait for child process: {command:?}"))?;
    }
    Ok(())
}
