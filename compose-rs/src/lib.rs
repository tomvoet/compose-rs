mod error;
use command::{DownCommand, PsCommand, ScaleCommand, StatsCommand, UpCommand};
pub use error::{ComposeBuilderError, ComposeError};
mod builder;
pub use builder::ComposeBuilder;
pub mod command;
mod container;
mod parser;
pub use command::ComposeCommand;

pub struct Compose {
    path: String,
}

impl Compose {
    pub fn builder() -> ComposeBuilder {
        ComposeBuilder::new()
    }

    pub fn from_file(path: &str) -> Result<Self, ComposeBuilderError> {
        let builder = Self::builder().path(path);
        builder.build()
    }

    fn init_command(&self) -> std::process::Command {
        let mut cmd = std::process::Command::new("docker");
        cmd.arg("compose").arg("-f").arg(&self.path);
        cmd
    }

    pub fn up(&self) -> UpCommand {
        UpCommand::new(self.init_command())
    }

    pub fn down(&self) -> DownCommand {
        DownCommand::new(self.init_command())
    }

    pub fn ps(&self) -> PsCommand {
        PsCommand::new(self.init_command())
    }

    pub fn scale(&self) -> ScaleCommand {
        ScaleCommand::new(self.init_command())
    }

    pub fn stats(&self) -> StatsCommand {
        StatsCommand::new(self.init_command())
    }
}
