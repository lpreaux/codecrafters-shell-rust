mod command;
mod commands;
mod shell;
mod utils;
mod parser;

use shell::Shell;

fn main() {
    let mut shell = Shell::new();
    shell.run();
}
