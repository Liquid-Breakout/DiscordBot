use crate::{Context, Error, checks::check_is_ingame_mod};

#[poise::command(
    slash_command,
    prefix_command,
    ephemeral,
    check = "check_is_ingame_mod"
)]
pub async fn getnumberid(
    ctx: Context<'_>,
    #[description = "ID to be converted."]
    id: String
) -> Result<(), Error> {
    let backend = &ctx.data().backend;
    let new_id = backend.get_number_id(id);

    match new_id {
        Ok(new_id) => ctx.reply(format!("Your numeric ID is: ``{new_id}``")).await?,
        Err(_) => ctx.reply(format!("An error occured while converting your ID.")).await?
    };
    Ok(())
}