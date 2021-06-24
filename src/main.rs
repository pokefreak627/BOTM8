//all this use stuff is just libraries thatre in the rust language? that this can use to have extra keywords predefined
use std::{collections::HashMap, fs::OpenOptions, str::FromStr};

use date_time::date_tuple::DateTuple;

use serde::{Deserialize, Serialize};
use serenity::builder::CreateEmbed;

use dev_commands::*;
use emotes::*;
use secrets::*;
use serenity::client::bridge::gateway::GatewayIntents;
use serenity::{
    async_trait,
    framework::{
        standard::{
            macros::{command, group},
            CommandResult,
        },
        StandardFramework,
    },
    model::{
        channel::Message,
        id::{RoleId, UserId},
    },
    prelude::*,
};
mod dev_commands;
mod emotes;
mod secrets;
//damn mobile users. this just defines prefixes
const PREFIXES: &[&str] = &["j/", "J/"];

#[derive(Default, Deserialize, Serialize)]

struct Birthdays {
    inner: HashMap<DateTuple, Vec<u64>>,
}

struct Date;

impl TypeMapKey for Date {
    type Value = Birthdays;
}
//some private messages for things that are saved as a "slice" kinda like a small array
const MESSAGES: &[&str] = include!("../.messages");
#[group]
#[commands(
    add_command,
    calm,
    test,
    add_birthday,
    help,
    sigh,
    shrug,
    wave,
    cry,
    laugh,
    plug,
    yox,
    psy,
    totsu,
    chito,
    echo,
    hagrid
)]
struct Birthday;

struct Checked;

impl TypeMapKey for Checked {
    type Value = DateTuple;
}
struct Handler;

//this is how events are handled, basically when something happens in the server, do the thing
#[async_trait]
impl EventHandler for Handler {
    //when someone joins, give them the M8 role
    async fn guild_member_addition(
        &self,
        ctx: Context,
        _guild_id: serenity::model::id::GuildId,
        mut new_member: serenity::model::guild::Member,
    ) {
        new_member
            .add_role(ctx.http, 723133417788670022)
            .await
            .unwrap();
    }
//when a message is sent check if it starts with a prefix, if it does then see what comes after and see if it matches any of the commands, then do what that command says. a safety net is in place here to make sure the bot only sees the prefix if it isnt the bot seeing it
    async fn message(&self, ctx: Context, msg: Message) {
        let mut d = DateTuple::today();
        let message = msg.content.trim();
        if msg.author.id != UserId(844801500790390791) {
            for prefix in PREFIXES.iter() {
                if message.starts_with(prefix) {
                    let commands: Commands =
                        ron::de::from_bytes(&std::fs::read("commands.ron").unwrap())
                            .unwrap_or_default();

                    if let Some(content) = commands.inner.get(&message[prefix.len()..]) {
                        if let Err(e) = msg.channel_id.say(&ctx.http, content).await {
                            println!("{}", e);
                        }
                    }
                }
            }
        }
        let should_update = {
            let last_checked = *ctx.data.read().await.get::<Checked>().unwrap();

            last_checked != d
        };
//this just handles the birthday stuff like checking what day it is and seeing if it matches any dates in birthdays.ron
        if should_update {
            *ctx.data.write().await.get_mut::<Checked>().unwrap() = d;

            d.subtract_years(d.get_year());

            if let Some(users) = ctx.data.write().await.get::<Date>().unwrap().inner.get(&d) {
                for user in users {
                    if let Ok(channel) = UserId(*user).create_dm_channel(&ctx.http).await {
                        if let Err(e) = match user {
                            97304001430708224 => channel.say(&ctx.http, MESSAGES[0]).await,
                            105381778633584640 => channel.say(&ctx.http, MESSAGES[1]).await,
                            _ => channel.say(&ctx.http, "Happy birthday, i hope you have a lovely day").await,
                        } {
                            println!("Error {} whilst sending birthday message", e);
                        }
                    }
                }
            }
        }
    }
}
//the help command that taught me how to use embeds, more annoying than i thought itd be to set up so thats why its very bare bones
#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |c| {
        let mut embed = CreateEmbed::default();
        embed.title ("Hello! I'm BOTM8");    
        embed.description ("I'm a bot m8. I'm the resident bot of the OnlyM8s server. 
        \n**My prefixes are ''j/'' and ''J/''** 
        \nMy commands are as follows:
        \n**EMOTES**
        \n`wave`. `laugh`, `sigh`, `cry`, `shrug`, 
        \n**UTILITY**
        \n`help` To display this  message again, 
        \n`add_birthday` type ''j/add_birthday'' followed by your birthday in the format yyyy-mm-dd. The hyphens are needed, 
        \n`add_command` Make your own command, j/add_command [command name] ''what you want the command to say''. This is only available to patrons so if you want this ability and many other perks go check it out [here](https://www.patreon.com/JM8/posts), 
        \nOh and maybe theres a few hidden *commands* ;)");
        c.set_embed(embed.clone())
    }).await.unwrap();

    Ok(())
}

//add birthday command, it makes sure theres no years so it just checks the days so when its that day/month etc itll be all good and send every year
#[command]
async fn add_birthday(ctx: &Context, msg: &Message) -> CommandResult {
    let mut data = ctx.data.write().await;

    let birthdays = match data.get_mut::<Date>() {
        Some(birthdays) => birthdays,
        None => {
            data.insert::<Date>(Birthdays::default());
            data.get_mut::<Date>().unwrap()
        }
    };

    let mut whitespace = msg.content.split_whitespace();

    whitespace.next();

    let mut date = DateTuple::from_str(whitespace.next().unwrap()).unwrap();

    println!("{:?}", date);

    date.subtract_years(date.get_year());
    let entry = birthdays.inner.entry(date);

    let id = msg.author.id.0;

    entry
        .and_modify(|ids| {
            if !ids.contains(&id) {
                ids.push(id)
            }
        })
        .or_insert_with(|| vec![id]);

    let file = OpenOptions::new()
        .write(true)
        .open("birthdays.ron")
        .unwrap();
    ron::ser::to_writer(file, birthdays).unwrap();
    msg.channel_id
        .say(&ctx.http, "birthday added!")
        .await
        .unwrap();

    Ok(())
}
//I HATE YOU PSY WHY DID YOU SUGGEST THIS COMMAND. THIS ONE COMMAND BROKE EVERY SINGLE TIME SOMETHING CHANGED AND EVEN THEN IT TOOK WAY TOO LONG FOR IT TO FUNCTION INITIALLY
#[command]
async fn add_command(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    if msg
        .author
        .has_role(&ctx.http, guild_id, RoleId(761542415219425311))
        .await
        .unwrap_or(false)
    {
        if let Some((_, content)) = msg.content.split_once(' ') {
            if let Some(name) = read_next(content).await {
                if let Some(string) = read_next(&content[name.len()..].trim_start()).await {
                    let name = name.to_string();
                    let string = string.to_string();

                    println!("{}", string);

                    let file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open("commands.ron")
                        .unwrap();

                    let mut commands: Commands = ron::de::from_reader(&file).unwrap_or_default();

                    commands.inner.insert(name, string);

                    ron::ser::to_writer(file, &commands).unwrap();

                    if let Err(e) = msg.channel_id.say(&ctx.http, "Command added").await {
                        println!("{}", e)
                    }
                }
            }
        }
    } else {
        if let Err(e) = msg
            .channel_id
            .say(
                &ctx.http,
                "Sorry, you don't have a high enough permission to add commands. To access this command, become a patron of JM8 and help ya boi out",
            )
            .await
        {
            println!("{}", e)
        }
    }

    Ok(())
}
//ive actually forgotten what this does i just know its important
async fn read_next(input: &str) -> Option<&str> {
    match input.chars().next() {
        Some(t) => {
            if t == '"' {
                input.get(
                    t.len_utf8()
                        ..=input
                            .chars()
                            .skip(1)
                            .position(|a| a == '"')
                            .unwrap_or_else(|| input.len()),
                )
            } else {
                input.get(
                    0..input
                        .chars()
                        .position(|a| a == ' ')
                        .unwrap_or_else(|| input.len()),
                )
            }
        }
        None => None,
    }
}
//this is the passive existence of the bot and what gets sent to discord and how it recieves stuff from discord ig
#[tokio::main]
async fn main() {
    let token = include_str!("../.token");
    let framework = StandardFramework::new()
        .configure(|p| p.allow_dm(false).prefixes(PREFIXES))
        .group(&BIRTHDAY_GROUP);

    let intents = GatewayIntents::GUILD_MEMBERS | GatewayIntents::GUILD_MESSAGES;

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .intents(intents)
        .await
        .expect("Err creating client");

    let file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("birthdays.ron")
        .unwrap();

    let birthdays: Birthdays = ron::de::from_reader(file).unwrap_or_default();

    {
        let mut data = client.data.write().await;

        data.insert::<Date>(birthdays);

        let mut date = DateTuple::today();
        date.subtract_days(1);
        data.insert::<Checked>(date);

        data.insert::<Killer>(client.shard_manager.clone());
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
