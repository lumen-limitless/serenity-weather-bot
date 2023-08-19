mod weather;

mod types;

use crate::{
    types::{Context, Data, Error},
    weather::get_forecast,
};
use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use shuttle_poise::ShuttlePoise;
use shuttle_secrets::SecretStore;

/// Responds with "pong!"
#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong!").await?;
    Ok(())
}

/// Responds with the weather forecast for a city
#[poise::command(slash_command)]
async fn weather(
    ctx: Context<'_>,
    #[description = "The city to get the weather for"] city: String,
) -> Result<(), Error> {
    match get_forecast(
        &city,
        &ctx.data().accuweather_api_key,
        &ctx.data().http_client,
    )
    .await
    {
        Ok((location, forecast)) => {
            ctx.say(format!(
                "Weather for {}:\n{}",
                location, forecast.headline.overview
            ))
            .await?;
        }
        Err(e) => {
            ctx.say(format!("Error: {}", e)).await?;
        }
    }

    Ok(())
}

#[shuttle_runtime::main]
async fn poise(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> ShuttlePoise<Data, Error> {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let accuweather_api_key = secret_store
        .get("ACCUWEATHER_API_KEY")
        .context("'ACCUWEATHER_API_KEY' was not found")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), weather()],
            ..Default::default()
        })
        .token(discord_token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    accuweather_api_key,
                    http_client: reqwest::Client::new(),
                })
            })
        })
        .build()
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(framework.into())
}
