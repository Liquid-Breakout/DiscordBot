use std::env;
use poise::serenity_prelude as serenity;
use liquid_breakout_backend::Backend;
mod commands;
pub mod checks;

pub struct Data {
    backend: Backend,
    ingame_mod_roleid: serenity::RoleId,
    privilege_users: Vec<serenity::UserId>
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start DiscordBot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error);
            let _ = ctx.say(format!("Error in command `{}`: {:?}", ctx.command().name, error)).await;
        }
        poise::FrameworkError::ArgumentParse { error, ctx, .. } => {
            println!("Error while parsing argument(s) for command `{}`: {:?}", ctx.command().name, error);
            let _ = ctx.say(format!("Error while parsing argument(s) for command `{}`: {:?}", ctx.command().name, error)).await;
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // for render web service stuff
    let ip = "0.0.0.0:8001";
    let _ = tokio::net::TcpListener::bind(ip).await.expect("Unable to create listener.");

    let discord_token = env::var("DISCORD_TOKEN").expect("DiscordBot cannot start: Failed to read DISCORD_TOKEN from environment");
    let roblox_cookie = env::var("ROBLOX_COOKIE").expect("DiscordBot cannot start: Failed to read ROBLOX_COOKIE from environment");
    let mongodb_url = env::var("MONGODB_URL").expect("DiscordBot cannot start: Failed to read MONGODB_URL from environment");

    println!("Server starting up.");

    let mut backend = Backend::new(
        roblox_cookie,
        vec![
            "123456789*=+-aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ".to_string(),
            "0123456789".to_string()
        ]
    );
    let connect_result = backend.connect_mongodb(mongodb_url, None).await;
    match connect_result {
        Ok(_) => {},
        Err(e) => panic!("DiscordBot cannot start: Failed to connect to MongoDB, reason: {}", (*e).to_string())
    }

    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::whitelist(),
                commands::getnumberid(),
                commands::getshareableid()
            ],
            prefix_options: poise::PrefixFrameworkOptions {
				prefix: Some("?".into()),
                ..Default::default()
            },
            on_error: |error| Box::pin(on_error(error)),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                ctx.set_activity(Some(serenity::ActivityData::listening(
                    format!(
                        "Prefix: {} - Bot by shiinazzz/Koda",
                        framework.options().prefix_options.prefix.clone().unwrap_or("".to_string())
                    )
                )));
                Ok(Data {
                    backend,
                    ingame_mod_roleid: 1185747952162058390.into(),
                    privilege_users: vec![
                        915410908921077780.into(),
                        849118831251030046.into(),
                        456202569740713986.into(),
                        268973336392892416.into(),
                        876892716208889897.into()
                    ]
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}