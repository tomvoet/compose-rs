use crate::{ComposeCommand, ComposeError};

use super::{CatchOutput, ComposeCommandArgs};

pub enum UpArgs {
    /// Scale a service to a number of containers
    Scale(String, u32),
    /// Waits for containers to be running|healthy before returning
    Wait,
}

impl ComposeCommandArgs for UpArgs {
    fn args(&self) -> Vec<String> {
        match self {
            UpArgs::Scale(service, count) => {
                vec!["--scale".to_string(), format!("{}={}", service, count)]
            }
            UpArgs::Wait => vec!["--wait".to_string()],
        }
    }
}

pub struct UpCommand {
    command: std::process::Command,
    args: Vec<UpArgs>,
}

impl UpCommand {
    pub fn new(command: std::process::Command) -> Self {
        Self {
            command,
            args: Vec::new(),
        }
    }

    pub fn scale(mut self, service: &str, count: u32) -> Self {
        self.args.push(UpArgs::Scale(service.to_string(), count));
        self
    }

    pub fn wait(mut self) -> Self {
        self.args.push(UpArgs::Wait);
        self
    }
}

impl ComposeCommand<(), UpArgs> for UpCommand {
    const COMMAND: &'static str = "up";

    fn exec(self) -> Result<(), ComposeError> {
        let mut command = self.command;
        command.arg(Self::COMMAND).arg("-d");

        for arg in self.args {
            command.args(&arg.args());
        }

        command.output().catch_output()?;

        Ok(())
    }
}
