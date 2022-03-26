mod database;

use crate::database::*;
use serenity::{
    async_trait, client::bridge::gateway::GatewayIntents, model::channel::Message,
    model::gateway::Ready, model::id::ChannelId,
    model::interactions::message_component::ButtonStyle, model::interactions::Interaction,
    prelude::*,
};
use std::env;
use tokio::time::{sleep, Duration};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("Connected as {}", ready.user.tag());
    }
    async fn message(&self, ctx: Context, message: Message) {
        if !message.author.bot {
            // lähetys kanavan id .env tiedostossa
            let send_channel_id = env::var("SEND_CHANNEL")
                .unwrap()
                .parse::<u64>()
                .expect("Lähetys kanavan ID:tä ei löytynyt .env tiedostosta");

            //tarkastaa onko kanava oikea
            if message.channel_id == send_channel_id {
                //tarkistus kanavan id .env tiedostossa
                let check_channel_id = env::var("CHECK_CHANNEL")
                    .unwrap()
                    .parse::<u64>()
                    .expect("Tarkistus kanavan ID:tä ei löytynyt .env tiedostosta");

                let check_channel = ChannelId::from(check_channel_id);

                //Lähetä hakemus tarkistettavaksi
                let msg_id = check_channel
                    .send_message(ctx.http, |m| {
                        m.embed(|e| {
                            e.title(format!("Hakemus - {}", message.author.tag()));
                            e.description(format!("```{}```", message.content));
                            e
                        });
                        m.components(|c| {
                            c.create_action_row(|r| {
                                r.create_button(|b| {
                                    b.label("Hyväksy");
                                    b.style(ButtonStyle::Success);
                                    b.custom_id("hyvaksy_hakemus")
                                });
                                r.create_button(|b| {
                                    b.label("Hylkää");
                                    b.style(ButtonStyle::Danger);
                                    b.custom_id("hylkaa_hakemus")
                                })
                            })
                        })
                    })
                    .await
                    .unwrap()
                    .id;

                //TODO: tallentaa hakemuksen tietoja databaseen
            }
        };
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::MessageComponent(button) => {
                let guild_id: u64 = button.guild_id.unwrap().0;
                let channel_id: u64 = button.channel_id.0;
                let message_id: u64 = button.message.id.0;

                if button.data.custom_id == "hyvaksy_hakemus" {
                    //TODO: hakee databasesta hakemuksen tekiän
                    let allowlist_author_id: u64 = 936258404920475748;

                    //hakee hakemuksen tekiän discordin apista
                    let allowlist_user = ctx.http.get_user(allowlist_author_id).await.unwrap();
                    let mut allowlist_member = ctx
                        .http
                        .get_member(guild_id, allowlist_author_id)
                        .await
                        .unwrap();

                    //roolin id .env tiedostossa
                    let allowlist_role_id = env::var("ALLOWLIST_ROLE")
                        .unwrap()
                        .parse::<u64>()
                        .expect("Allowlist roolin ID:tä ei löytynyt .env tiedostosta");

                    //odottaa 1 sekunnin ennen jatkamista
                    sleep(Duration::from_secs(2)).await;

                    //lisää allowlist roolin
                    allowlist_member
                        .add_role(&ctx.http, allowlist_role_id)
                        .await
                        .unwrap();

                    //lähettää dm hakemus hyväksytty
                    allowlist_user
                        .direct_message(&ctx.http, |m| {
                            m.embed(|e| {
                                e.description("Hakemuksesi on hyväksytty");
                                e
                            })
                        })
                        .await
                        .unwrap();

                    //poistaa viestin kanavalta
                    ctx.http
                        .delete_message(channel_id, message_id)
                        .await
                        .unwrap();

                    //TODO: poistaa datan databasesta
                } else if button.data.custom_id == "hylkaa_hakemus" {
                    //TODO: hakee databasesta hakemuksen tekiän
                    let allowlist_author_id: u64 = 936258404920475748;

                    //hakee discordin apista hakemuksen tekijän
                    let allowlist_user = ctx.http.get_user(allowlist_author_id).await.unwrap();

                    //odottaa 1 sekunnin ennen jatkamista
                    sleep(Duration::from_secs(2)).await;

                    //lähettää dm hakemus hylätty
                    allowlist_user
                        .direct_message(&ctx.http, |m| {
                            m.embed(|e| {
                                e.description("Hakemuksesi on hylätty");
                                e
                            })
                        })
                        .await
                        .unwrap();

                    //poistaa viestin kanavalta
                    ctx.http
                        .delete_message(channel_id, message_id)
                        .await
                        .unwrap();

                    //TODO: poistaa datan databasesta
                }
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    //discord
    let bot_token = env::var("BOT_TOKEN").expect("Tokenia ei löytynyt .env tiedostosta!");
    let application_id = env::var("APPLICATION_ID")
        .unwrap()
        .parse::<u64>()
        .expect("Application ID:tä ei löytynyt .env tiedostosta");

    let mut client = Client::builder(&bot_token)
        .application_id(application_id)
        .event_handler(Handler)
        .intents(
            GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::GUILD_MEMBERS
                | GatewayIntents::GUILD_INTEGRATIONS,
        )
        .await
        .expect("Clienttiä ei voitu luoda");

    client.start().await.unwrap();
}
