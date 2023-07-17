mod commands {
  pub mod status;
  pub mod bruh;
  pub mod help;
}

mod admin_commands {
  pub mod sync;
  pub mod register;
  pub mod interval;
}

pub mod utils;

use commands::{status, bruh, help};

use admin_commands::{sync, register, interval};

pub mod db;

mod events;
use db::ServerConfig;
use events::event_handler;

use dotenv::dotenv;
use poise::serenity_prelude::{self as serenity, Activity};

pub struct Counter {
  pub interaction_counter: u32,
  pub upper_bound: u32,
  pub lower_bound: u32,
}

impl Counter {
  pub fn increase_interaction_counter(&mut self) {
    self.interaction_counter += 1;
    if self.interaction_counter % self.upper_bound == 0 {
      self.interaction_counter = 0;
    }
    println!("Interaction counter: {}", self.interaction_counter);
  }

  pub fn reset_interaction_counter(&mut self) {
    self.interaction_counter = 0;
  }

  pub fn set_bounds(&mut self, lower_bound: u32, upper_bound: u32) {
    self.lower_bound = lower_bound;
    self.upper_bound = upper_bound;
  }
}

pub struct Data {
  pub counter: std::sync::Mutex<Counter>,
} 

impl Data {
  pub fn new(config: ServerConfig) -> Self {
    Self {
      counter: std::sync::Mutex::new(Counter {
        interaction_counter: 0,
        upper_bound: config.upper_bound,
        lower_bound: config.lower_bound,
      }),
    }
  }

  pub fn lock(&self) -> std::sync::MutexGuard<'_, Counter> {
    self.counter.lock().unwrap()
  }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type PrefixContext<'a> = poise::PrefixContext<'a, Data, Error>;

#[tokio::main]
async fn main() {
  dotenv().ok();
  let framework = poise::Framework::builder()
    .options(poise::FrameworkOptions {
      prefix_options: poise::PrefixFrameworkOptions {
        prefix: Some("~".into()),
        edit_tracker: Some(poise::EditTracker::for_timespan(std::time::Duration::from_secs(60))),
        ..Default::default()
      },
      pre_command: |ctx| {
        Box::pin(async move {
          println!("Executing command {}...", ctx.command().qualified_name);
        })
      },
      post_command: |ctx| {
        Box::pin(async move {
          println!("Executed command {}!", ctx.command().qualified_name);
        })
      },
      event_handler: |_ctx, event, _framework, _data| {
        Box::pin(async move {
          event_handler(_ctx, event, _framework, _data).await?;
          Ok(())
        })
      },
      commands: vec![
        status::status(), 
        register::register(), 
        bruh::bruh(), 
        help::help(), 
        sync::sync(), 
        interval::set_interval()
        ],
      ..Default::default()
    })
    .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
    .intents(serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT)
    .setup(|ctx, _ready, _framework| {
      Box::pin(async move {
        ctx.set_activity(Activity::playing("Praise the crab :crab:")).await;
        println!("{} is now online!", _ready.user.name);
        let config = db::connect().get_guild_config("1102689880821219368");
        match config {
          Ok(config) => {         
            Ok(Data::new(config))
          },
          Err(e) => {
            println!("Error getting config: {}", e);
            Ok(Data::new(ServerConfig {
              upper_bound: 10,
              lower_bound: 5,
            }))
          }
        }
      })
    });
    println!("Starting framework...");
  framework.run().await.unwrap();
}