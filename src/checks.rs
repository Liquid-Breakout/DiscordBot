use crate::{Context, Error};

pub async fn check_is_ingame_mod(ctx: Context<'_>) -> Result<bool, Error> {
	let author = ctx.author_member().await.ok_or("Failed to fetch server member.")?;

	let has_role = author.roles.contains(&ctx.data().ingame_mod_roleid);

	if !has_role {
        ctx.send(
            poise::CreateReply::default()
                .content("This command requires In-Game Mod role to execute.")
                .ephemeral(true)
        ).await?;
	}

	Ok(has_role)
}