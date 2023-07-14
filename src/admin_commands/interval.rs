use crate::{Context, Error, db};

/// set the interval range for the bot to send messages
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn set_interval(
    ctx: Context<'_>,
    #[description = "The minimum number of messages after which the bot will send a message"]
    min: u32,
    #[description = "The maximum number of messages after which the bot will send a message"]
    max: u32,
) -> Result<(), Error> {
    ctx.data().lock().set_bounds(min, max);
    match db::connect().set_guild_config(&ctx.guild_id().unwrap().to_string(), min, max) {
        Ok(_) => {},
        Err(e) => println!("Error setting guild config: {}", e),
    }
    ctx.say(format!("The bot will now send messages after {} to {} messages.", min, max)).await?;
    Ok(())
}
