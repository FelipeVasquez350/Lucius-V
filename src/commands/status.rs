use crate::{Context, Error};

/// Show the bot's status
#[poise::command(slash_command)]
pub async fn status(
  ctx: Context<'_>, 
) -> Result<(), Error> {
  //TODO: add ping
  ctx.send(|msg|{
    let lower_bound = ctx.data().lock().lower_bound;
    let upper_bound = ctx.data().lock().upper_bound;
    msg.embed(|e| {
      e.title("Status")
      .description("The bot is online.")
      .field("Lower Bound", lower_bound, true)
      .field("Upper Bound", upper_bound, true)
    })
  }).await?;
  Ok(())
}