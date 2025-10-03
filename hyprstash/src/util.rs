use super::*;

pub fn move_clients_to_workspace(
    target: WorkspaceId,
    clients: &[Address],
) -> Option<DispatchError> {
    let mut errors = vec![];
    for client in clients.iter() {
        let res = Dispatch::call(DispatchType::MoveToWorkspaceSilent(
            WorkspaceIdentifierWithSpecial::Id(target),
            Some(WindowIdentifier::Address(client.clone())),
        ));
        if let Err(error) = res {
            errors.push(error);
        }
    }

    if errors.is_empty() {
        None
    } else {
        Some(DispatchError(errors))
    }
}
