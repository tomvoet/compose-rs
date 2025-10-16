use crate::{ComposeCommand, ComposeError};

use super::{CatchOutput, ComposeCommandArgs};

pub enum StartArgs {
    /// Execute the command in dry-run mode
    DryRun,
}

impl ComposeCommandArgs for StartArgs {
    fn args(&self) -> Vec<String> {
        match self {
            StartArgs::DryRun => vec!["--dry-run".to_string()],
        }
    }
}

pub struct StartCommand {
    command: std::process::Command,
    args: Vec<StartArgs>,
}

impl StartCommand {
    pub fn new(command: std::process::Command) -> Self {
        Self { 
            command,
            args: Vec::new(),
        }
    }

    pub fn dry_run(mut self) -> Self {
        self.args.push(StartArgs::DryRun);
        self
    }
}

impl ComposeCommand<(), StartArgs> for StartCommand {
    const COMMAND: &'static str = "start";

    fn exec(self) -> Result<(), ComposeError> {
        let mut command = self.command;
        command.arg(Self::COMMAND);

        for arg in self.args {
            command.args(&arg.args());
        }

        command.output().catch_output()?;

        Ok(())
    }
}