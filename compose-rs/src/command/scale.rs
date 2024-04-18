use super::{CatchOutput, ComposeCommandArgs};
use crate::{ComposeCommand, ComposeError};

pub enum ScaleArgs {
    NoDeps,
    Service(u32, String),
}

impl ComposeCommandArgs for ScaleArgs {
    fn args(&self) -> Vec<String> {
        match self {
            ScaleArgs::NoDeps => vec!["--no-deps".to_string()],
            ScaleArgs::Service(count, service) => {
                vec![format!("{}={}", service, count)]
            }
        }
    }
}
pub struct ScaleCommand {
    command: std::process::Command,
    args: Vec<ScaleArgs>,
}

impl ScaleCommand {
    pub fn new(command: std::process::Command) -> Self {
        Self {
            command,
            args: Vec::new(),
        }
    }

    pub fn no_deps(mut self) -> Self {
        self.args.push(ScaleArgs::NoDeps);
        self
    }

    pub fn service(mut self, count: u32, service: &str) -> Self {
        self.args
            .push(ScaleArgs::Service(count, service.to_string()));
        self
    }
}

impl ComposeCommand<(), ScaleArgs> for ScaleCommand {
    const COMMAND: &'static str = "scale";

    fn exec(self) -> Result<(), ComposeError> {
        let mut command = self.command;
        command.arg(Self::COMMAND);

        // first use no_deps if it is present
        if let Some(no_deps) = self.args.iter().find(|a| matches!(a, ScaleArgs::NoDeps)) {
            command.args(&no_deps.args());
        }

        // then apply all service args
        let scale_args = self
            .args
            .iter()
            .filter(|a| !matches!(a, ScaleArgs::NoDeps))
            .collect::<Vec<&ScaleArgs>>();

        if scale_args.is_empty() {
            return Err(ComposeError::InvalidArguments(
                "No service specified".to_string(),
            ));
        }

        for arg in scale_args {
            command.args(&arg.args());
        }

        command.output().catch_output()?;

        Ok(())
    }
}
