use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StashedWorkspace {
    pub stash_location: WorkspaceId,
    pub original_workspace: WorkspaceId,
    pub client_addresses: Vec<Address>,
}

pub fn workspace_stash(
    data: &Data,
    workspace: WorkspaceId,
    stash_workspace: WorkspaceId,
) -> Result<(StashedWorkspace, Option<DispatchError>)> {
    let clients = &data.clients;
    let client_addresses = clients
        .iter()
        .filter_map(|client| (client.workspace.id == workspace).then_some(client.address.clone()))
        .collect::<Vec<_>>();

    let dispatch_error = move_clients_to_workspace(stash_workspace, &client_addresses);

    let stashed = StashedWorkspace {
        stash_location: stash_workspace,
        original_workspace: workspace,
        client_addresses,
    };

    Ok((stashed, dispatch_error))
}

pub fn workspace_pop(
    data: &Data,
    instance: &StashedWorkspace,
    target: Option<WorkspaceId>,
) -> Result<()> {
    let target = target.unwrap_or(instance.original_workspace);
    let existing_clients = &data.clients;
    let existing_clients = existing_clients
        .iter()
        .map(|client| (client.address.clone(), client))
        .collect::<HashMap<_, _>>();
    let valid_clients = instance
        .client_addresses
        .iter()
        .filter(|client| {
            if let Some(real_client) = existing_clients.get(client)
                && real_client.workspace.id == instance.stash_location
            {
                true
            } else {
                false
            }
        })
        .cloned()
        .collect::<Vec<_>>();

    let dispatch_error = move_clients_to_workspace(target, &valid_clients);

    if let Some(error) = dispatch_error {
        Err(StashError::Dispatch(error).into())
    } else {
        Ok(())
    }
}
