use std::{io, process::Output};

use crate::ComposeError;
mod up;
pub use up::UpCommand;
mod down;
pub use down::DownCommand;
mod ps;
pub use ps::PsCommand;
mod scale;
pub use scale::ScaleCommand;
mod stats;
pub use stats::StatsCommand;

pub trait ComposeCommand<ReturnT, ArgType = ()>
where
    ArgType: ComposeCommandArgs,
{
    const COMMAND: &'static str;

    fn exec(self) -> Result<ReturnT, ComposeError>;
}

pub trait ComposeCommandArgs {
    fn args(&self) -> Vec<String>;
}

impl ComposeCommandArgs for () {
    fn args(&self) -> Vec<String> {
        Vec::new()
    }
}

pub(super) trait CatchOutput {
    fn catch_output(self) -> Result<Output, ComposeError>;
}

impl CatchOutput for io::Result<Output> {
    fn catch_output(self) -> Result<Output, ComposeError> {
        match self {
            Ok(output) => {
                if output.status.success() {
                    Ok(output)
                } else {
                    Err(ComposeError::CommandFailed(output))
                }
            }
            Err(err) => Err(ComposeError::IoError(err)),
        }
    }
}
