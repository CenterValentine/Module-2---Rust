## Starting a rust program:

- cargo new project_name
- code . (to execute)
- cargo build (.exe)
- cargo run 



## important to learn:
 - F2 to rename all variables
 - rustfmt to format

Go to Definition F12 - Go to the source code of the type definition.
Peek Definition ⌥F12 - Bring up a Peek window with the type definition.
Go to References ⇧F12 - Show all references for the type.
Show Call Hierarchy ⇧⌥H - Show all calls from or to a function.

You can navigate via symbol search using the Go to Symbol commands from the Command Palette (⇧⌘P).

Go to Symbol in File - ⇧⌘O
Go to Symbol in Workspace - ⌘T


The rustc linter, enabled by default, detects basic Rust errors, but you can use clippy to get more lints

Rename Symbol from the context menu, Command Palette, or via F2.
Here are just a few of the refactorings available:


The rust-analyzer extension also supports other code refactorings and code generation, which the extension calls Assists.
Convert if statement to guarded return
Inline variable
Extract function
Add return type
Add import


To pretty-format rust code:
cargo fmt
OR
cargo +nightly fmt

Example: - cargo fmt --all -- --check

Using --check instructs rustfmt to exit with an error code if the input is not formatted correctly. (It will also print any found differences.)

For things you do not want rustfmt to mangle, use #[rustfmt::skip]


