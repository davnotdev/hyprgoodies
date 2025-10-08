use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StashedFullSession {
    pub stash_location: WorkspaceId,
    pub monitors: Vec<StashedMonitor>,
}

pub fn everything_stash(
    data: &Data,
    stash_workspace: WorkspaceId,
) -> Result<(StashedFullSession, Option<DispatchError>)> {
    let mut monitors = vec![];
    let mut dispatch_errors = DispatchError::default();

    for monitor in data.monitors.iter() {
        let (instance, new_dispatch_errors) = monitor_stash(data, monitor.id, stash_workspace)?;
        monitors.push(instance);
        if let Some(new_dispatch_errors) = new_dispatch_errors {
            dispatch_errors.append(new_dispatch_errors);
        }
    }

    let stashed = StashedFullSession {
        stash_location: stash_workspace,
        monitors,
    };

    Ok((stashed, dispatch_errors.into_optional()))
}

pub fn everything_pop(
    data: &Data,
    instance: &StashedFullSession,
    no_missing_monitors: bool,
    relative: bool,
) -> Result<()> {
    let missing_monitors = instance
        .monitors
        .iter()
        .filter_map(|monitor| {
            (!data
                .monitors
                .iter()
                .any(|m| m.id == monitor.original_monitor))
            .then_some(monitor.original_monitor)
        })
        .collect::<HashSet<_>>();

    if no_missing_monitors && !missing_monitors.is_empty() {
        return Err(StashError::MonitorNotFound(*missing_monitors.iter().next().unwrap()).into());
    }

    for monitor in instance.monitors.iter() {
        if missing_monitors.contains(&monitor.original_monitor) || relative {
            monitor_pop_relative(data, monitor, false)?;
        } else {
            monitor_pop_relative(data, monitor, true)?;
        }
    }

    Ok(())
}
