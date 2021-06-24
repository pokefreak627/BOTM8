use serenity::builder::CreateEmbed;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};
//all these commands are just standard ping commands but theyre the hidden commands so theyre kinda cool?
#[command]
async fn totsu(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "https://tenor.com/view/guilty-gear-guilty-gear-strive-may-totsugeki-dolphin-gif-22007409").await.unwrap();
    Ok(())
}
#[command]
async fn echo(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx.http,
            "Be who you are, because that's all that matters. Don't let anyone tell you otherwise.",
        )
        .await
        .unwrap();
    Ok(())
}
#[command]
async fn chito(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |c| {
        let mut embed = CreateEmbed::default();
        embed.title ("Congrats!:");    
        embed.description ("You found a hidden command. So far there are 5 hidden commands, try and find them all just don't clog any channels apart from <#850056003642982490> \n \nThis is Chito's art place thing. \nI think she does very cool art and if i remember correctly she does commissions so you can check out here: https://www.instagram.com/elaine.artwork/ \nDon't be afraid to dm her and ask about commms her art is truly amazing. You can see her artwork scattered around the server in emotes and pfp's if it's not already on her Instagram. \nNow what are you waiting for?! Go look at some beautiful art!");
        c.set_embed(embed.clone())
    }).await.unwrap();
    Ok(())
}
#[command]
async fn psy(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, " ''**KAH... RAK... TOR?**''")
        .await
        .unwrap();
    msg.channel_id
        .say(&ctx.http, "https://imgur.com/lxzwqbS")
        .await
        .unwrap();

    Ok(())
}
#[command]
async fn plug(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.author.id.0 == 400312224809484288 {
        msg.channel_id.send_message(&ctx.http, |c| {
        let mut embed = CreateEmbed::default();
        embed.title ("FUN FACT:");    
        embed.description (" JM8 has an amazing Youtube channel where he puts a lot of effort into his game analysis videos and game related videos.\n
        **DID YOU KNOW?** \nHe's also a qualified game designer, dyslexic and sound designer. \nGo subscribe and check out his videos! NEW VIDEOS OUT ON THE 3RD SUNDAY! 
        https://www.youtube.com/c/JM8GameDesign/
        \nYou can also jump right into his Discord channel : https://discord.gg/B9t7YCF
        \nSupport him further on his qualified Patreon : https://www.patreon.com/JM8/posts
        \n**THERE'S MORE?**
        \nWant more JM8 content? Join his streams where he complains about colours EVERY FRIDAY at 7PM: https://www.twitch.tv/itsjm8");
        c.set_embed(embed.clone())
      }).await.unwrap();
    } else {
        if let Err(e) = msg
            .channel_id
            .say(
                &ctx.http,
                "Sorry, you don't have a high enough permission to plug J's channel which you should already be subscribed to smh",
            )
            .await
        {
            println!("{}", e)
        }
    }
    Ok(())
}
#[command]
async fn hagrid(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "https://pbs.twimg.com/media/DGQpRcaVwAAocp4.jpg")
        .await
        .unwrap();
    Ok(())
}
#[command]
async fn yox(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.author.id.0 == 440232724280770563 {
        msg.channel_id
        .say(
            &ctx.http,
            "okay so the command might be stupidly long, but hear me out. You poke you are better than you think, stop saying these mean things to yourself you're literally not a bad person, what might seem like a natural thing for you is sometimes more than anything, and sometimes somethings that might seem bad about yourself,  people might not agree, so just. Please, believe in yourself more and credit yourself sometimes like you keep telling me to do.
            \n> be proud.",
        )
        .await
        .unwrap();
    } else {
        if let Err(e) = msg
            .channel_id
            .say(
                &ctx.http,
                "Sorry, you don't have a high enough permiss- you're not yox...",
            )
            .await
        {
            println!("{}", e)
        }
    }

    Ok(())
}
