mod command;
mod commands;
mod shell;

use shell::Shell;

fn main() {
    let shell = Shell::new();
    shell.run();
}
