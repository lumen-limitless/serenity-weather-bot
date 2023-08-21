use crate::{weather::get_forecast, Context, Error};

/// Responds with "pong!"
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong!").await?;
    Ok(())
}

/// Responds with the weather forecast for a city
#[poise::command(slash_command)]
pub async fn weather(
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
            let _ = ctx
                .send(|m| {
                    m.content("Weather for: ");
                    m.embed(|e| {
                        e.title(location);
                        e.description(forecast.headline.overview);
                        e.footer(|f| f.text("Powered by AccuWeather"));
                        e.color(0x00ff00);
                        e
                    });

                    m
                })
                .await?;
        }

        Err(e) => {
            ctx.say(format!("Error: {}", e)).await?;
        }
    }

    Ok(())
}
