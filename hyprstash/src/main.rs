use anyhow::Result;
use clap::{Parser, Subcommand};
use hyprland::{
    dispatch::*,
    prelude::*,
    shared::{Address, MonitorId, WorkspaceId},
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

//  - stash workspace
//      - pops into target workspace
//  - stash Monitor
//      - pops onto target monitor
//      - pops onto previous monitor / workspace configuration
//          - moved workspaces get existing windows moved into replacement
//  - stash everything
//      - pops all stashed monitor / workspace configuration
//          - moved workspaces get existing windows moved into replacement
//  - pop stash
//  - clear workspace
//
//  - when workspaces need to be rearranged,
//

mod data;
mod error;
mod stashes;
mod state;
mod util;

use data::*;
use error::*;
use stashes::*;
use state::*;
use util::*;

const DEFAULT_STASH_LOCATION: WorkspaceId = 8;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(long)]
    stash_location: Option<WorkspaceId>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    StashWorkspace {
        name: String,
        #[arg(long)]
        workspace: Option<WorkspaceId>,
    },
    StashMonitor {
        name: String,
        #[arg(long)]
        monitor: Option<MonitorId>,
    },
    StashEverything {
        name: String,
    },
    // TODO: Implement Generic Pop
    PopWorkspace {
        name: String,
        #[arg(long)]
        target: Option<WorkspaceId>,
    },
    PopMonitor {
        name: String,
        #[arg(long)]
        target: Option<MonitorId>,

        #[arg(long)]
        relative: Option<bool>,
    },
    PopSession {
        name: String,

        #[arg(long)]
        relative: Option<bool>,

        #[arg(long)]
        no_missing_monitors: Option<bool>,
    },
    Clear {
        name: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let data = Data::new()?;

    match cli.command {
        Commands::StashWorkspace { name, workspace } => {
            let (instance, dispatch_error) = workspace_stash(
                &data,
                workspace.unwrap_or(data.active_workspace),
                cli.stash_location.unwrap_or(DEFAULT_STASH_LOCATION),
            )?;
            StashedInstance::Workspace(instance).write(&name)?;

            if let Some(errors) = dispatch_error {
                errors.print_errors();
                todo!()
            }
        }
        Commands::StashMonitor { name, monitor } => {
            let (instance, dispatch_error) = monitor_stash(
                &data,
                monitor.unwrap_or(data.active_monitor),
                cli.stash_location.unwrap_or(DEFAULT_STASH_LOCATION),
            )?;
            StashedInstance::Monitor(instance).write(&name)?;

            if let Some(errors) = dispatch_error {
                errors.print_errors();
                todo!()
            }
        }
        Commands::StashEverything { name } => {
            let (instance, dispatch_error) =
                everything_stash(&data, cli.stash_location.unwrap_or(DEFAULT_STASH_LOCATION))?;
            StashedInstance::Everything(instance).write(&name)?;

            if let Some(errors) = dispatch_error {
                errors.print_errors();
                todo!()
            }
        }
        Commands::PopWorkspace { name, target } => {
            let instance = StashedInstance::new_from_name(&name)?;
            let StashedInstance::Workspace(stashed_workspace) = instance else {
                return Err(StashError::MismatchedPopType.into());
            };
            workspace_pop(&data, &stashed_workspace, target)?;
            StashedInstance::remove_instance(&name);
        }
        Commands::PopMonitor {
            name,
            target,
            relative,
        } => {
            let relative = relative.unwrap_or_default();
            let instance = StashedInstance::new_from_name(&name)?;
            let StashedInstance::Monitor(stashed_monitor) = instance else {
                return Err(StashError::MismatchedPopType.into());
            };
            if relative {
                monitor_pop_relative(&data, &stashed_monitor, false)?;
            } else {
                monitor_pop_absolute(&data, &stashed_monitor, target)?;
            }
            StashedInstance::remove_instance(&name);
        }
        Commands::PopSession {
            name,
            relative,
            no_missing_monitors,
        } => {
            let relative = relative.unwrap_or_default();
            let no_missing_monitors = no_missing_monitors.unwrap_or_default();
            let instance = StashedInstance::new_from_name(&name)?;
            let StashedInstance::Everything(stashed_session) = instance else {
                return Err(StashError::MismatchedPopType.into());
            };
            everything_pop(&data, &stashed_session, no_missing_monitors, relative)?;
            StashedInstance::remove_instance(&name);
        }
        Commands::Clear { name } => {
            if let Some(name) = name {
                StashedInstance::remove_instance(&name);
            } else {
                StashedInstance::remove_all_instances();
            }
        }
    }

    Ok(())
}
