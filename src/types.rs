/// User data, which is stored and accessible in all command invocations
pub struct Data {
    pub accuweather_api_key: String,
    pub http_client: reqwest::Client,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Context<'a> = poise::Context<'a, Data, Error>;
