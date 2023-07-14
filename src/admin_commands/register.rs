use crate::{Context, Error};

/// Sync the guild/global commands
#[poise::command(prefix_command, slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn register(
  ctx: Context<'_>
) -> Result<(), Error> {
  poise::builtins::register_application_commands_buttons(ctx).await?;
  Ok(())
}
