# Garden

A suite of command-line programs to replace the ugly parts of UNIX shell.


## `each`

Processes standard input line by line, executing the specified command for each entry.

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
