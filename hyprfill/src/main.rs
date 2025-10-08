use anyhow::Result;
use clap::{Parser, Subcommand};
use fork::daemon;
use hyprland::{
    data::*,
    dispatch::*,
    prelude::*,
    shared::{MonitorId, WorkspaceId},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, process::Command};

mod config;
mod error;

use config::*;
use error::*;

const DEFAULT_COMMAND: &str = "sinkgui";

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(long)]
    config: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Fill,
    Setup,
    ExampleConfig,
    DumpMonitors,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Fill => {
            let config = Config::load_config(cli.config)?;
            let monitors = Monitors::get()?;

            if let Some(commands) = config.defaultcommand.as_ref()
                && commands.is_empty()
            {
                return Err(FillError::EmptyCommand.into());
            }

            let resolved_workspaces = validate_fill(&config, &monitors)?;
            execute_fill(&config, &resolved_workspaces)?;
        }
        Commands::Setup => {
            Config::write_default_config(cli.config)?;
        }
        Commands::ExampleConfig => {
            let ex = Config::example_config()?;
            println!("{}", ex);
        }
        Commands::DumpMonitors => {
            let monitors = Monitors::get()?;

            println!("ID: NAME\tDESCRIPTION");
            println!("-------------------------------");
            for monitor in monitors {
                eprintln!(
                    "{:02}: {:010}\t{}",
                    monitor.id, monitor.name, monitor.description
                );
            }
        }
    }

    Ok(())
}

pub struct WorkspaceFill {
    id: WorkspaceId,
    monitor: MonitorId,
    command: Option<Vec<String>>,
}

fn validate_fill(config: &Config, monitors: &Monitors) -> Result<Vec<WorkspaceFill>> {
    for w in config.workspaces.iter() {
        if let Some(commands) = w.commands.as_ref()
            && commands.is_empty()
        {
            return Err(FillError::EmptyCommand.into());
        }
    }

    let mut notfound: Vec<String> = vec![];
    let resolved_workspaces = config
        .workspaces
        .iter()
        .map(|w| {
            let mut monitor_id = None;
            if let Some(desc) = w.monitorbydesc.clone() {
                if let Some(id) = monitors
                    .iter()
                    .find_map(|m| (m.description == desc).then_some(m.id))
                {
                    monitor_id = Some(id)
                } else {
                    notfound.push(desc);
                }
            } else if let Some(name) = w.monitorbyname.clone() {
                if let Some(id) = monitors
                    .iter()
                    .find_map(|m| (m.name == name).then_some(m.id))
                {
                    monitor_id = Some(id)
                } else {
                    notfound.push(name);
                }
            } else if let Some(id) = w.monitorbyid {
                if let Some(id) = monitors
                    .iter()
                    .find_map(|m| (m.id == id as MonitorId).then_some(m.id))
                {
                    monitor_id = Some(id)
                } else {
                    notfound.push(format!("id:{}", id));
                }
            } else {
                return Err(FillError::MissingMonitor);
            };

            let workspace = monitor_id.map(|monitor_id| WorkspaceFill {
                id: w.id as WorkspaceId,
                monitor: monitor_id,
                command: w.commands.clone(),
            });

            Ok(workspace)
        })
        .collect::<Result<Vec<Option<WorkspaceFill>>, FillError>>()?;

    if !notfound.is_empty() {
        let err = notfound
            .into_iter()
            .fold(String::new(), |acc, nf| acc + ", " + &nf);
        return Err(FillError::FollowingNotFound(err).into());
    }

    let resolved_workspaces = resolved_workspaces
        .into_iter()
        .map(|w| w.unwrap())
        .collect::<Vec<_>>();

    let workspace_set = resolved_workspaces
        .iter()
        .map(|w| w.id)
        .collect::<HashSet<_>>();

    if workspace_set.len() != resolved_workspaces.len() {
        return Err(FillError::DuplicateWorkspaces.into());
    }

    Ok(resolved_workspaces)
}

fn execute_fill(config: &Config, resolved_workspaces: &[WorkspaceFill]) -> Result<()> {
    let default_command = config
        .defaultcommand
        .clone()
        .unwrap_or(vec![DEFAULT_COMMAND.to_string()]);

    for workspace in resolved_workspaces {
        let command_list = workspace.command.as_ref().unwrap_or(&default_command);
        let _ = daemon(false, false);
        let proc = Command::new(&command_list[0])
            .args(&command_list[1..])
            .spawn()?;
        let pid = proc.id();

        Dispatch::call(DispatchType::FocusMonitor(MonitorIdentifier::Id(
            workspace.monitor,
        )))?;
        Dispatch::call(DispatchType::MoveToWorkspace(
            WorkspaceIdentifierWithSpecial::Id(workspace.id),
            Some(WindowIdentifier::ProcessId(pid)),
        ))?;
    }
    Ok(())
}
