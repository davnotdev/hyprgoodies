use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StashedMonitor {
    pub workspaces: Vec<StashedWorkspace>,
    pub layout: Vec<WorkspaceId>,
    pub original_monitor: MonitorId,
}

pub fn monitor_stash(
    data: &Data,
    monitor: MonitorId,
    stash_workspace: WorkspaceId,
) -> Result<(StashedMonitor, Option<DispatchError>)> {
    // Ensure that monitor does exist.
    let _ = data
        .monitors
        .iter()
        .find(|m| m.id == monitor)
        .ok_or(StashError::MonitorNotFound(monitor))?;

    let workspaces = data
        .workspaces
        .iter()
        .filter(|workspace| workspace.monitor_id == Some(monitor))
        .collect::<Vec<_>>();

    let layout = workspaces
        .iter()
        .map(|workspace| workspace.id)
        .collect::<Vec<_>>();

    let mut stashed_workspaces = vec![];
    let mut dispatch_errors = DispatchError::default();

    for workspace in workspaces.iter() {
        let (instance, new_dispatch_errors) = workspace_stash(data, workspace.id, stash_workspace)?;
        stashed_workspaces.push(instance);
        if let Some(new_dispatch_errors) = new_dispatch_errors {
            dispatch_errors.append(new_dispatch_errors);
        }
    }

    let stashed = StashedMonitor {
        workspaces: stashed_workspaces,
        layout,
        original_monitor: data.active_monitor,
    };

    Ok((stashed, dispatch_errors.into_optional()))
}

pub fn monitor_pop_absolute(
    data: &Data,
    instance: &StashedMonitor,
    target: Option<MonitorId>,
) -> Result<()> {
    let target = target.unwrap_or(instance.original_monitor);

    let mut max_workspace = data
        .workspaces
        .iter()
        .map(|w| w.id)
        .max()
        .unwrap_or(WorkspaceId::default());
    let monitor_workspaces = data
        .workspaces
        .iter()
        .filter(|w| w.monitor_id == Some(target))
        .collect::<Vec<_>>();
    let old_new_workspace_map = monitor_workspaces
        .iter()
        .enumerate()
        .map(|(idx, workspace)| {
            let old = instance.layout.get(idx).copied().unwrap_or(max_workspace);
            max_workspace += 1;
            (old, workspace.id)
        })
        .collect::<HashMap<_, _>>();
    for workspace in instance.workspaces.iter() {
        let new_workspace = old_new_workspace_map[&workspace.original_workspace];
        workspace_pop(data, workspace, Some(new_workspace))?;
    }

    Ok(())
}

pub fn monitor_pop_relative(data: &Data, instance: &StashedMonitor) -> Result<()> {
    for workspace in instance.workspaces.iter() {
        workspace_pop(data, workspace, None)?;
    }

    Ok(())
}
