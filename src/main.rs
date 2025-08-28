mod command;
mod commands;
mod shell;
mod utils;
mod parser;

use shell::Shell;

fn main() {
    let shell = Shell::new();
    shell.run();
}
