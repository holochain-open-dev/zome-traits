use holochain_client::*;
use holochain_zome_types::zome::ZomeName;

// Returns all the zomes in the given cell that implement the given zome trait
pub async fn find_zomes_with_zome_trait(
    admin_ws: &AdminWebsocket,
    app_ws: &AppWebsocket,
    cell_id: &CellId,
    zome_trait_hash: [u8; 32],
) -> anyhow::Result<Vec<ZomeName>> {
    let dna_def = admin_ws
        .get_dna_definition(cell_id.dna_hash().clone())
        .await?;

    let mut zomes = vec![];

    for (coordinator_zome, _) in dna_def.coordinator_zomes {
        let traits = get_implemented_traits(
            app_ws,
            ZomeCallTarget::CellId(cell_id.clone()),
            coordinator_zome.clone(),
        )
        .await?;

        if traits.iter().any(|t| t.eq(&zome_trait_hash)) {
            zomes.push(coordinator_zome);
        }
    }

    Ok(zomes)
}

// Returns all the traits implemented in the given cell and zome 
pub async fn get_implemented_traits(
    app_ws: &AppWebsocket,
    cell: ZomeCallTarget,
    zome_name: ZomeName,
) -> anyhow::Result<Vec<[u8; 32]>> {
    let Ok(response) = app_ws
        .call_zome(
            cell,
            zome_name,
            "__implemented_zome_traits".into(),
            ExternIO::encode(()).unwrap(),
        )
        .await
    else {
        return Ok(vec![]);
    };
    let implemented_zome_traits: Vec<[u8; 32]> = response.decode()?;

    Ok(implemented_zome_traits)
}
