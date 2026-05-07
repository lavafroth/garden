# Garden

A suite of command-line programs to replace the ugly parts of UNIX shell.


## `each`

Processes standard input line by line, executing the specified command for each entry.

`each [SUBCOMMAND] [COMMAND]`


### Subcommands
- `into` pipes each line from standard input into the command, one command invocation per line.
    Example: `cat base64-encoded-lines.txt | each into base64 -d`

- `over` appends each line from standard input as a trailing argument to the command.
    Example: `cat list.txt | each over echo "Item:"`
