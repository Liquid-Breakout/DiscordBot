use crate::{Context, Error};

pub async fn check_is_privilege(ctx: Context<'_>) -> Result<bool, Error> {
	let author = ctx.author();

	let has_perm = ctx.data().privilege_users.contains(&author.id);

	if !has_perm {
        ctx.send(
        poise::CreateReply::default()
            .content("This command requires Privilege permission to execute.")
            .ephemeral(true)
        ).await?;
	}

	Ok(has_perm)
}

pub async fn check_is_ingame_mod(ctx: Context<'_>) -> Result<bool, Error> {
    let is_privilege = check_is_privilege(ctx).await;
    match is_privilege {
        Ok(has_perm) => {
            if has_perm {
                return Ok(true)
            }
        },
        Err(_) => {}
    };

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