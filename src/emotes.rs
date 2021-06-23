use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

#[command]
async fn sigh(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx.http,
            "https://tenor.com/view/sigh-relief-anime-gif-15119014",
        )
        .await
        .unwrap();
    Ok(())
}
#[command]
async fn cry(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx.http,
            "https://tenor.com/view/anime-umaru-cry-crying-tears-gif-5184314",
        )
        .await
        .unwrap();
    Ok(())
}
#[command]
async fn wave(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx.http,
            "https://tenor.com/view/wave-blushing-smiling-gif-14386503",
        )
        .await
        .unwrap();
    Ok(())
}
#[command]
async fn shrug(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "https://tenor.com/view/shrug-smug-smile-miss-kobayashi-kobayashisan-chi-no-maid-dragon-gif-13119038").await.unwrap();
    Ok(())
}
#[command]
async fn laugh(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx.http,
            "https://tenor.com/view/puffybear-puffy-cute-lol-happy-gif-12628636",
        )
        .await
        .unwrap();
    Ok(())
}
