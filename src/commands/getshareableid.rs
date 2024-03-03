use crate::{Context, Error};

#[poise::command(
    slash_command,
    prefix_command,
    ephemeral
)]
pub async fn getshareableid(
    ctx: Context<'_>,
    #[description = "ID to be converted."]
    id: String
) -> Result<(), Error> {
    let backend = &ctx.data().backend;
    let new_id = backend.get_shareable_id(id);
    
    match new_id {
        Ok(new_id) => ctx.reply(format!("Your shareable ID is: ``{new_id}``")).await?,
        Err(_) => ctx.reply(format!("An error occured while converting your ID.")).await?
    };
    Ok(())
}