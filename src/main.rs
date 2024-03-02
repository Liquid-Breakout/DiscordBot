use std::env;
use poise::serenity_prelude as serenity;
use liquid_breakout_backend::Backend;
mod commands;
pub mod checks;

pub struct Data {
    backend: Backend,
    ingame_mod_roleid: serenity::RoleId
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
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
            on_error: |error| Box::pin(on_error(error)),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { 
                    backend,
                    ingame_mod_roleid: 1185747952162058390.into()
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}