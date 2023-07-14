use crate::commands::bruh::get_message;
use crate::{PrefixContext, Error};
use crate::db;
use db::connect;
use poise::serenity_prelude::UserId;
use crate::utils::get_random_number;
use regex::Regex;

pub async fn on_message(
  ctx: PrefixContext<'_>
) -> Result<(), Error> {
  let mut database = connect();
  let reg = Regex::new(r"(http|ftp|https):\/\/([\w_-]+(?:(?:\.[\w_-]+)+))([\w.,@?^=%&:\/~+#-]*[\w@?^=%&\/~+#-])").unwrap();
  let is_link = reg.is_match(&ctx.msg.content);
  let message = if is_link {
    reg.captures_iter(&ctx.msg.content).into_iter().next().unwrap()[0].to_string()
  } else {
    ctx.msg.content.clone()
  };
  if is_link {
    println!("Link");
  } else {
    println!("Text:");
  }
  let result = database.insert_text_beyond_my_comprehension(&message, is_link);
  match result {
    Ok(_) => println!("Inserted successfully"),
    Err(e) => println!("Error inserting: {}", e),
  }
  ctx.data.lock().increase_interaction_counter();
  let upper_bound = ctx.data.lock().upper_bound;
  let lower_bound = ctx.data.lock().lower_bound;

  // Yes it's needed cuz it would otherwise create a new thread which doesn't allow to use Send
  let random_number = get_random_number(lower_bound,upper_bound);

  let ref_msg = ctx.msg.referenced_message.as_ref();
  let mut refer = false;
  match ref_msg {
    Some(msg) => {
      if msg.author.id.to_string() == "782571074415099924" {
        refer = true;
      }
    },
    None => ()
  }

  if ctx.data.lock().interaction_counter % (random_number) == 0 ||
    ctx.data.lock().interaction_counter == 0 ||
    ctx.msg.mentions_user_id(UserId(782571074415099924)) ||
    refer {
      ctx.data.lock().reset_interaction_counter();
      let message = get_message();
      ctx.say(message).await?;
  }

  Ok(())
}