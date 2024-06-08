use std::env;
use anyhow::Context as _;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;

mod util;
mod commands;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {

    let discord_token: String;
    match env::var("SHUTTLE") {
        Ok(_val) => {
            println!("Started on Shuttle!");
            discord_token = secret_store
                .get("DEPLOYMENT_DISCORD_TOKEN")
                .context("'DEPLOYMENT_DISCORD_TOKEN' was not found!")?;
        },
        Err(_e) => {
            println!("Started locally!");
            discord_token = secret_store
                .get("DEVELOPEMENT_DISCORD_TOKEN")
                .context("'DEVELOPEMENT_DISCORD_TOKEN' was not found!")?;
        },
    }

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: util::command_utils::get_commands(),//vec![commands::Hello::hello()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = ClientBuilder::new(discord_token, GatewayIntents::non_privileged())
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}