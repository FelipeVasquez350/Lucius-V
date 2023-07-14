use regex::Regex;

use crate::{Context, Error};

/// Sync the database to the channel messages
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn sync(
  ctx: Context<'_>, 
) -> Result<(), Error> {
  ctx.send(|builder| {builder
    .content("Syncing...")
    .ephemeral(true)
  }).await?;

  let channel = ctx.channel_id();
  let messages = channel.messages(&ctx, |retriever| {
    retriever.limit(1)
  }).await?;
  let mut message_id = *messages[0].id.as_u64();
  
  #[allow(while_true)]
  while true {
    let messages = channel.messages(&ctx, |retriever| {
      retriever.before(message_id).limit(100)
    }).await?;

    println!("Got {} messages", messages.len());
    if messages.len() == 0 {
      break;
    }

    for message in messages {
      let mut database = crate::db::connect();
      let reg = Regex::new(r"(http|ftp|https):\/\/([\w_-]+(?:(?:\.[\w_-]+)+))([\w.,@?^=%&:\/~+#-]*[\w@?^=%&\/~+#-])").unwrap();
      let is_link = reg.is_match(&message.content);
      let msg = if is_link {
        reg.captures_iter(&message.content).into_iter().next().unwrap()[0].to_string()
      } else {
        message.content.clone()
      };
      let result = database.insert_text_beyond_my_comprehension(&msg, is_link);
      match result {
        Ok(_) => println!("Inserted successfully"),
        Err(e) => println!("Error inserting: {}", e),
      }
      message_id = message.id.0;
    }
  }
  ctx.send(|builder| {builder
    .content("Synced!")
    .ephemeral(true)
  }).await?;

  Ok(())
}