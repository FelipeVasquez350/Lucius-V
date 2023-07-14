pub mod message;

use crate::{Data, Error};
use poise::serenity_prelude::{self as serenity};

pub async fn event_handler(
  ctx: &serenity::Context,
  event: &poise::Event<'_>,
  framework: poise::FrameworkContext<'_, Data, Error>,
  data: &Data,
) -> Result<(), Error> {

  println!("Got an event in event handler: {:?}", event.name());
   
  match event {
    poise::Event::Message { new_message } => {
      let invocation_data = tokio::sync::Mutex::new(Box::new(()) as _);
      let ctx = poise::PrefixContext {
        data: data,
        serenity_context: ctx,
        msg: new_message,
        framework,
        // Have to declare these fields, but they're not used
        command: &framework.options().commands[0],
        invoked_command_name: "",
        prefix: "",
        args: "",
        invocation_data: &invocation_data,
        action: |_| unreachable!(),
        trigger: poise::MessageDispatchTrigger::MessageCreate,
        parent_commands: &[],
         __non_exhaustive: (), 
      };

      if new_message.content.len() > 0 && !new_message.author.bot {
        println!("Got a message event saying: {:?}", new_message);
        message::on_message(ctx).await?;
      } 
    }

    _ => {}
  }
  Ok(())
}