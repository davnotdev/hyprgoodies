use super::*;

use hyprland::data::*;

#[derive(Debug)]
pub struct Data {
    pub monitors: Monitors,
    pub clients: Clients,
    pub workspaces: Workspaces,

    pub active_workspace: WorkspaceId,
    pub active_monitor: MonitorId,
}

impl Data {
    pub fn new() -> Result<Self> {
        let (active_monitor, active_workspace) = Monitors::get()?
            .into_iter()
            .find_map(|monitor| {
                monitor
                    .focused
                    .then_some((monitor.id, monitor.active_workspace.id))
            })
            .ok_or(StashError::NoActiveMonitorWorkspace)?;

        let monitors = Monitors::get()?;
        let clients = Clients::get()?;
        let workspaces = Workspaces::get()?;

        Ok(Data {
            monitors,
            clients,
            workspaces,
            active_workspace,
            active_monitor,
        })
    }
}
