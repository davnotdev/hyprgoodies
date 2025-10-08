use hyprland::{error::HyprError, shared::MonitorId};
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StashError {
    #[error("Bad name, only alphanumeric characters accepted")]
    BadName,
    #[error("Got mismatched pop type")]
    MismatchedPopType,
    #[error("Unexpected missing active monitor and workspace")]
    NoActiveMonitorWorkspace,
    #[error("Monitor {0} not found")]
    MonitorNotFound(MonitorId),
    #[error("Multiple dispatch errors")]
    Dispatch(DispatchError),
    #[error("IO {0}")]
    IOError(#[from] io::Error),
}

#[derive(Debug, Default)]
pub struct DispatchError(pub Vec<HyprError>);

impl DispatchError {
    pub fn append(&mut self, mut errors: DispatchError) {
        self.0.append(&mut errors.0);
    }

    pub fn into_optional(self) -> Option<Self> {
        self.0.is_empty().then_some(self)
    }

    pub fn print_errors(self) {
        todo!()
    }
}
