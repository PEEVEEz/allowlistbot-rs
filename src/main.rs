mod database;

use crate::database::*;
use serenity::{
    async_trait, client::bridge::gateway::GatewayIntents, model::channel::Message,
    model::gateway::Ready, model::id::ChannelId, prelude::*,
};
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Connected as {}", ready.user.tag());
    }
    async fn message(&self, ctx: Context, message: Message) {
        if !message.author.bot {
            let send_channel_id = env::var("SEND_CHANNEL")
                .unwrap()
                .parse::<u64>()
                .expect("Lähetys kanavan ID:tä ei löytynyt");
            let check_channel_id = env::var("CHECK_CHANNEL")
                .unwrap()
                .parse::<u64>()
                .expect("Tarkistus kanavan ID:tä ei löytynyt");

            //Tarkastaa onko kanava oikea
            if message.channel_id == send_channel_id {
                let check_channel = ChannelId::from(check_channel_id);

                //Lähetä hakemus tarkistettavaksi
                let msg_id = check_channel
                    .send_message(ctx.http, |m| {
                        m.embed(|e| {
                            e.title(format!("Hakemus - {}", message.author.tag()));
                            e.description(format!("```{}```", message.content));
                            e
                        })
                    })
                    .await
                    .unwrap()
                    .id;

                save_allowlist(Allowlist {
                    message_id: msg_id.to_string(),
                    author_id: message.author.id.to_string(),
                });
            }
        };
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    //discord
    let bot_token = env::var("BOT_TOKEN").expect("Token not found!");

    let mut client = Client::builder(&bot_token)
        .event_handler(Handler)
        .intents(
            GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::GUILD_MEMBERS
                | GatewayIntents::GUILD_INTEGRATIONS,
        )
        .await
        .expect("Error creating client");

    if let Err(e) = client.start().await {
        println!("Client error: {}", e);
    }
}
