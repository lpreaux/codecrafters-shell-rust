mod command;
mod commands;
mod shell;
mod utils;
mod parser;
mod execution;

use shell::Shell;

fn main() {
    let mut shell = Shell::new();
    shell.run();
}
