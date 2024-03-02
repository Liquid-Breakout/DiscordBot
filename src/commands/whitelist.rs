use crate::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn whitelist(
    ctx: Context<'_>,
    #[description = "Asset ID for whitelisting."]
    asset_id: u64
) -> Result<(), Error> {
    let backend = &ctx.data().backend;
    
    backend.whitelist_asset_without_user(asset_id).await?;
    let shareable_id = backend.get_shareable_id(asset_id.to_string())?;

    ctx.reply(format!("Whitelisted! Your shareable ID is: ``{shareable_id}``")).await?;
    Ok(())
}