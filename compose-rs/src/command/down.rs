use std::time::Duration;

use super::{CatchOutput, ComposeCommandArgs};
use crate::{ComposeCommand, ComposeError};

pub enum RemoveOptions {
    Local,
    All,
}

pub enum DownArgs {
    RemoveVolumes,
    RemoveOrphans,
    RemoveImages(RemoveOptions),
    Timeout(Duration),
}

impl ComposeCommandArgs for DownArgs {
    fn args(&self) -> Vec<String> {
        match self {
            DownArgs::RemoveVolumes => vec!["-v".to_string()],
            DownArgs::RemoveOrphans => vec!["--remove-orphans".to_string()],
            DownArgs::RemoveImages(opt) => match opt {
                RemoveOptions::Local => vec!["--rmi".to_string(), "local".to_string()],
                RemoveOptions::All => vec!["--rmi".to_string(), "all".to_string()],
            },
            DownArgs::Timeout(duration) => {
                vec!["--timeout".to_string(), duration.as_secs().to_string()]
            }
        }
    }
}

pub struct DownCommand {
    command: std::process::Command,
    args: Vec<DownArgs>,
}

impl DownCommand {
    pub fn new(command: std::process::Command) -> Self {
        Self {
            command,
            args: Vec::new(),
        }
    }

    pub fn remove_volumes(mut self) -> Self {
        self.args.push(DownArgs::RemoveVolumes);
        self
    }

    pub fn remove_orphans(mut self) -> Self {
        self.args.push(DownArgs::RemoveOrphans);
        self
    }

    pub fn remove_images(mut self, opt: RemoveOptions) -> Self {
        self.args.push(DownArgs::RemoveImages(opt));
        self
    }

    pub fn timeout(mut self, duration: Duration) -> Self {
        self.args.push(DownArgs::Timeout(duration));
        self
    }
}

impl ComposeCommand<(), DownArgs> for DownCommand {
    const COMMAND: &'static str = "down";

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
