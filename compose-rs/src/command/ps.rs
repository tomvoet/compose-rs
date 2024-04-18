use crate::{container::Container, parser, ComposeCommand, ComposeError};

use super::CatchOutput;

pub struct PsCommand {
    command: std::process::Command,
}

impl PsCommand {
    pub fn new(command: std::process::Command) -> Self {
        Self { command }
    }
}

impl ComposeCommand<Vec<Container>> for PsCommand {
    const COMMAND: &'static str = "ps";

    fn exec(self) -> Result<Vec<Container>, ComposeError> {
        let mut command = self.command;
        command.arg(Self::COMMAND).arg("-a").arg("--no-trunc");

        let output = command.output().catch_output()?;
        let output = String::from_utf8_lossy(&output.stdout);

        parser::parse_ps(&output)
    }
}
