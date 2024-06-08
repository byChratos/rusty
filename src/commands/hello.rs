#[poise::command(slash_command)]
pub async fn hello(ctx: crate::Context<'_>) -> Result<(), crate::Error> {
    ctx.say("world!").await?;
    Ok(())
}