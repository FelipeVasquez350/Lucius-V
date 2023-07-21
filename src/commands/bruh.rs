use crate::{Context, Error};
use crate::database;
use markov::Chain;
use crate::utils::get_random_number;

pub fn get_message() -> String {
  let mut database = database::connect();
  let rng = get_random_number(0, 10);
  if rng != 1 {
    let mut chain = Chain::new();
    let messages = database.get_all_messages();

    match messages {
      Ok(messages) => {
        for message in messages {
            chain.feed_str(&message);
        }
      },
      Err(e) => println!("Error getting messages: {}", e),
    }
    chain.generate_str()
  } else {
    let messages = database.get_all_links();
    match messages {
      Ok(messages) => {
        let random_number = get_random_number(0, messages.len() as u32 - 1);
        messages[random_number as usize].clone()
      },
      Err(e) => { println!("Error getting messages: {}", e);
        String::from("Error getting messages")
      }
    }
  } 
}

/// Show the bot's status
#[poise::command(slash_command)]
pub async fn bruh(
  ctx: Context<'_>, 
) -> Result<(), Error> {
  let msg = get_message();
  ctx.send(|m| 
    m.content(msg)
     .allowed_mentions(|am| am.empty_parse())
  ).await?;
  Ok(())
}