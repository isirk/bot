use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Show this help menu
#[poise::command(slash_command, prefix_command, track_edits)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Says Hello World
#[poise::command(slash_command, prefix_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Hello, World!").await?;
    Ok(())
}

/// Paginate through some content
#[poise::command(slash_command, prefix_command)]
pub async fn paginate(ctx: Context<'_>) -> Result<(), Error> {
    let pages = &[
        "Content of first page",
        "Content of second page",
        "Content of third page",
        "Content of fourth page",
    ];

    poise::samples::paginate(ctx, pages).await?;

    Ok(())
}

/// Shuts down the bot
#[poise::command(prefix_command, owners_only, hide_in_help)]
pub async fn shutdown(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Shutting down...").await?;
    ctx.framework().shard_manager().shutdown_all().await;
    Ok(())
}
