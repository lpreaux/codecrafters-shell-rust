mod echo;
mod exit;
mod help;
mod type_cmd;
mod pwd;

use crate::command::CommandHandler;
use std::collections::HashMap;

pub struct CommandRegistry {
    handlers: HashMap<String, Box<dyn CommandHandler>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            handlers: HashMap::new(),
        };

        registry.register(Box::new(exit::ExitHandler));
        registry.register(Box::new(echo::EchoHandler));
        registry.register(Box::new(pwd::PwdHandler));
        registry.register(Box::new(type_cmd::TypeHandler));
        registry.register(Box::new(help::HelpHandler));

        registry
    }

    pub fn register(&mut self, handler: Box<dyn CommandHandler>) {
        self.handlers.insert(handler.name().to_string(), handler);
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn CommandHandler>> {
        self.handlers.get(name)
    }

    pub fn list_commands(&self) -> Vec<&str> {
        self.handlers.keys().map(|x| x.as_str()).collect()
    }
}
