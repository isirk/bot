use crate::{Context, Error};
use chrono::Utc;
use poise::serenity_prelude as serenity;

pub struct Todo {
    name: String,
    completed: bool,
    created: chrono::DateTime<chrono::Utc>,
    due: Option<chrono::DateTime<chrono::Utc>>,
    author: serenity::UserId,
}

let mut todos: Vec<Todo> = Vec::new();

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

#[poise::command(
    prefix_command,
    slash_command,
    subcommands("add", "remove", "list", "complete")
)]
pub async fn todo(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Hello there!").await?;
    Ok(())
}

/// Add a new todo
#[poise::command(prefix_command, slash_command)]
pub async fn add(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("You invoked the first child command!").await?;
    Ok(())
}

/// Remove a todo
#[poise::command(prefix_command, slash_command)]
pub async fn remove(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("You invoked the second child command!").await?;
    Ok(())
}

/// List all todos
#[poise::command(prefix_command, slash_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("You invoked the third child command!").await?;
    Ok(())
}

/// Mark a todo as complete
#[poise::command(prefix_command, slash_command)]
pub async fn complete(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("You invoked the fourth child command!").await?;
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
