use anyhow::{Context, Result};
use std::env::Args;
use std::io::Write;
use std::iter::{Peekable, Skip};
use std::process::exit;
use std::{io::stdin, process::Command};

fn usage() {
    let usage_str = r#"
`each <PREDICATE> [SUBCOMMAND] [COMMAND]`

### Subcommands
#### `into`

Pipes each line from standard input into the command, one command invocation per line.

Example: Decode lines of base64 encoded data by piping each line into the base64 command.

```sh
cat base64-encoded-lines.txt | each into base64 --decode
```

#### `over`

Appends each line from standard input as a trailing argument to the command.

Example: Wrap each line in the input in HTML list item tags.

```sh
cat list.txt | each printf "<li>%s</li>\n"
```

### Predicates

#### `with newline`

Preserve the trailing newline of the input lines before passing them to the spawned command.

Example: Encode lines to base64 data by piping each line *with trailing newlines* into the base64 command.

```sh
cat lines.txt | each with newline into base64
```
"#;
    eprintln!("{usage_str}");
}

pub enum Approach {
    Into,
    Over,
}

fn has_with_newline_predicate(args: &mut Peekable<Skip<Args>>) -> bool {
    for s in ["with", "newline"] {
        if !args.peek().is_some_and(|arg| arg == s) {
            return false;
        }
        args.next();
    }
    true
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1).peekable();
    let with_newline = has_with_newline_predicate(&mut args);
    let Some(approach) = args.next() else {
        usage();
        exit(1);
    };

    let Some(program) = args.next() else {
        usage();
        exit(1);
    };

    let args: Vec<String> = args.collect();

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
        let Ok(mut line) = line else {
            usage();
            exit(1);
        };

        if with_newline {
            line.push('\n');
        }
        let mut command = Command::new(&program);
        command.args(&args);

        let mut child = match approach {
            Approach::Into => {
                let mut child = command
                    .stdin(std::process::Stdio::piped())
                    .spawn()
                    .with_context(|| format!("failed to spawn process: {command:?}"))?;

                if let Some(mut stdin) = child.stdin.take() {
                    stdin.write_all(line.as_bytes()).with_context(|| {
                        let repr = &line[..line.len().min(16)];
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
