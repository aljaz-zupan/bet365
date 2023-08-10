use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::UserId},
    prelude::*,
};

pub use serenity::{framework::StandardFramework, prelude::GatewayIntents, Client};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!start" {
            // Assuming you have the user IDs
            let user_id_1 = UserId(282566557710680065); // Jaz
                                                        /* let user_id_2 = UserId(539330297188057089); // David */
            // Get private channels to these users
            if let Ok(channel) = user_id_1.create_dm_channel(&ctx).await {
                // Send message to the first user
                let _ = channel
                    .say(&ctx.http, "Bla bla bla tekma bla bla bla!")
                    .await;
            }
            /* if let Ok(channel) = user_id_2.create_dm_channel(&ctx).await {
                // Send message to the second user
                let _ = channel.say(&ctx.http, "Hello, User 2!").await;
            } */
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Bot is ready, username: {}", ready.user.name);
    }
}
