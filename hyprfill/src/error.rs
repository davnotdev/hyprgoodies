use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FillError {
    #[error("Unexpected missing home dir")]
    NoHomeDir,
    #[error("One config entry is missing a monitor identifier")]
    MissingMonitor,
    #[error("Got duplicate workspace ids in config")]
    DuplicateWorkspaces,
    #[error("The following could not be found: {0}")]
    FollowingNotFound(String),
    #[error("Config entry contains empty command array")]
    EmptyCommand,
    #[error("IO {0}")]
    IOError(#[from] io::Error),
}
