use crate::{Context, Error};

/// Use in case you need help
#[poise::command(slash_command)]
pub async fn help(
  ctx: Context<'_>, 
) -> Result<(), Error> {
  ctx.send(|msg| {
    msg.embed(|e| {
      e.title("Help")
      .description("This is a bot that generates messages based on the messages in the server.\nIt uses a markov chain to generate messages.\nYou can use the /bruh command to generate a message. You can also use the /setinterval command to set the interval at which the bot sends messages.\nYou can also use the /status command to check if the bot is online.")
      .field("Commands", "bruh, help, status", false)
      .field("Admin Commands", "register, setinterval", false)
    })
  }).await?;
  Ok(())
}