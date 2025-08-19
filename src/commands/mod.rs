mod exit;
mod echo;
mod help;

use std::collections::HashMap;
use crate::command::CommandHandler;

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

        // Create help handler with access to existing commands
        let help_handler = help::HelpHandler::new(&registry);
        registry.register(Box::new(help_handler));

        registry
    }

    pub fn register(&mut self, handler: Box<dyn CommandHandler>) {
        self.handlers.insert(handler.name().to_string(), handler);
    }

    pub fn get (&self, name: &str) -> Option<&Box<dyn CommandHandler>> {
        self.handlers.get(name)
    }

    pub fn list_commands(&self) -> Vec<&str>{
        self.handlers.keys().map(|x| x.as_str()).collect()
    }
}