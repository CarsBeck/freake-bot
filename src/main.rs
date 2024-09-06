use anyhow::Context as _;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use poise::serenity_prelude as serenity;
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// TODO: Store all commands in a separate file? Or an enum/array/something else???

// Basic "Hello world" type slash commands
#[poise::command(slash_command)]
async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Hello freaks!").await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn freake(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("GAVIN IS A FREAK!").await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn freakebob(
    ctx: Context<'_>
) -> Result<(), Error> {
    ctx.say("https://tenor.com/view/freakbob-gif-11176155028712317218").await?;
    Ok(())
}

// Display a user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s, account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

// main function used by the shuttle tool
#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![hello(), freake(), freakebob(), age()],
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
