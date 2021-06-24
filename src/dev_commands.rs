use date_time::date_tuple::DateTuple;
use serde::{Deserialize, Serialize};
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::standard::{macros::command, CommandResult},
    model::{channel::Message, id::RoleId},
    prelude::*,
};
use std::{collections::HashMap, sync::Arc};
pub struct Killer;

impl TypeMapKey for Killer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[derive(Default, Deserialize, Serialize)]
pub struct Commands {
    pub inner: HashMap<String, String>,
}

pub struct Command;

impl TypeMapKey for Command {
    type Value = Commands;
}
pub struct Checked;

impl TypeMapKey for Checked {
    type Value = DateTuple;
}
//standard ping command just here to check the bot is working. ironically this command has failed me the most
#[command]
async fn test(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    if msg
        .author
        .has_role(&ctx.http, guild_id, RoleId(850040055312678963))
        .await
        .unwrap_or(false)
    {
        msg.channel_id.say(&ctx.http, "working!").await.unwrap();
    } else {
        if let Err(e) = msg
            .channel_id
            .say(
                &ctx.http,
                "Sorry, you don't have a high enough permission to use this. If I am acting up ping poke and they will help calm me down",
            )
            .await
            {
                println!("{}", e)
        }
    }
    Ok(())
}
//a kill switch. i just didnt have the heart for someone to be able to kill my bot. fuck you i get attached to things easily ok? dont judge me...
#[command]
async fn calm(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    if msg
        .author
        .has_role(&ctx.http, guild_id, RoleId(850040055312678963))
        .await
        .unwrap_or(false)
    {
        msg.channel_id.say(&ctx.http, "bye bye...").await.unwrap();

        msg.channel_id
            .say(
                &ctx.http,
                "https://tenor.com/view/sleeping-drool-deep-sleep-gif-15687825",
            )
            .await
            .unwrap();
            //simple terms this just clones the client the bot is in so it just goes "AAAAAAAA" and melts
        let mut data = ctx.data.write().await;
        let shard_manager = data.get_mut::<Killer>().unwrap();

        shard_manager.lock().await.shutdown_all().await;
    } else {
        if let Err(e) = msg
        .channel_id
        .say(
            &ctx.http,
            "Sorry, you don't have a high enough permission to use this. If I am acting up ping poke and they will help calm me down",
        )
        .await
        {
            println!("{}", e)
        }
    }
    Ok(())
}
