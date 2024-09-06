use poise::serenity_prelude as serenity;
use crate::{Context, Error};

// Basic "Hello world" type slash command
#[poise::command(slash_command)]
pub async fn freakebob(context: Context<'_>) -> Result<(), Error> {
    context.say("https://tenor.com/view/freakbob-gif-11176155028712317218").await?;
    Ok(())
}

// Targets a specific user
#[poise::command(slash_command, prefix_command)]
pub async fn freake_finder(context: Context<'_>, #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| context.author());
    let response = format!("{} IS A FREAK", u.name);
    context.say(response).await?;
    Ok(())
}