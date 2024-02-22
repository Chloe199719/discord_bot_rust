use poise::serenity_prelude as serenity;

use ::serenity::{all::ChannelType, builder::CreateChannel};
use serde::Deserialize;
use serenity::prelude::*;

use std::env;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

use serenity::model::gateway::GatewayIntents;
// #[group]
// #[commands(ping, am_i_admin, am_i_cool, random_quote, test)]
// struct General;

// struct Handler;

// #[async_trait]
// impl EventHandler for Handler {
//     async fn message(&self, ctx: Context, msg: Message) {
//         println!("{}: {}", msg.author.name, msg.content);
//         if msg.content == "ping" {
//             let response = MessageBuilder::new()
//                 .push("Pong!")
//                 .mention(&msg.author)
//                 .build();
//             if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
//                 println!("Error sending message: {:?}", why);
//             }
//         }
//     }
// }
// struct ShardManagerContainer;

// impl TypeMapKey for ShardManagerContainer {
//     type Value = Arc<ShardManager>;
// }

// struct CommandCounter;

// impl TypeMapKey for CommandCounter {
//     type Value = HashMap<String, u64>;
// }
#[poise::command(slash_command, prefix_command)]
async fn cr_ca(
    ctx: Context<'_>,
    #[description = "Name of the new channel"] channel_name: String,
) -> Result<(), Error> {
    let x = ctx.guild_id().unwrap().channels(&ctx).await.unwrap();
    for i in x {
        if i.1.name == channel_name {
            i.0.delete(&ctx).await.unwrap();
        }
    }
    // x.create_channel(
    //     &ctx,
    //     CreateChannel::new(channel_name).kind(ChannelType::Text),
    // )
    // .await?;
    poise::say_reply(
        ctx,
        format!("Channel with {} name been deleted", channel_name),
    )
    .await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn cr_parent(
    ctx: Context<'_>,
    #[description = "Name of the new channel"] channel_name: String,
    #[description = "Name of the category"] category_name: String,
    #[description = "Type of the channel"] channel_type: String,
) -> Result<(), Error> {
    let x = ctx.guild_id().unwrap().channels(&ctx).await.unwrap();
    let guild = ctx.guild_id().unwrap();
    let channel_type = if let "voice" = channel_type.as_str() {
        ChannelType::Voice
    } else if channel_type == "category" {
        ChannelType::Category
    } else {
        ChannelType::Text
    };
    for i in x {
        if i.1.name == category_name {
            guild
                .create_channel(
                    &ctx,
                    CreateChannel::new(&channel_name)
                        .kind(channel_type)
                        .category(i.0),
                )
                .await
                .unwrap();
        }
    }
    // x.create_channel(
    //     &ctx,
    //     CreateChannel::new(channel_name).kind(ChannelType::Text),
    // )
    // .await?;
    poise::say_reply(
        ctx,
        format!("Channel with {} name been created", channel_name),
    )
    .await?;

    Ok(())
}
#[poise::command(slash_command, prefix_command)]
async fn cr(
    ctx: Context<'_>,
    #[description = "Name of the new channel"] channel_name: String,
) -> Result<(), Error> {
    let x = ctx.guild_id().unwrap();
    x.create_channel(
        &ctx,
        CreateChannel::new(channel_name).kind(ChannelType::Text),
    )
    .await?;
    poise::say_reply(ctx, "Channel created successfully!").await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn cr_di(
    ctx: Context<'_>,
    #[description = "Name of the new channel"] channel_name: String,
) -> Result<(), Error> {
    let x = ctx.guild_id().unwrap();
    match x
        .create_channel(
            &ctx,
            CreateChannel::new(channel_name).kind(ChannelType::Category),
        )
        .await
    {
        Ok(_) => poise::say_reply(ctx, "Category created successfully!").await?,
        Err(e) => poise::say_reply(ctx, format!("Error creating category: {}", e)).await?,
    };

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn current_time(ctx: Context<'_>) -> Result<(), Error> {
    let time = chrono::Utc::now();
    ctx.say(time.naive_utc().to_string()).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn get_weather(
    ctx: Context<'_>,
    #[description = "Select a City"] weather: Option<String>,
) -> Result<(), Error> {
    let x = if let Some(weather) = weather {
        weather.to_lowercase()
    } else {
        "munich".to_string()
    };
    let request = reqwest::Client::new();
    let response = request
        .get(format!("https://api.api-ninjas.com/v1/weather?city={}", x))
        .header("X-Api-Key", "AkW2CJ7sXNWjKVtACMI8GGYIYBE4XU8haDniaYUt")
        .send()
        .await?;
    let data: WeatherResponse = response.json().await?;

    ctx.say(format!("Weather at {} :Current Wind {} , at {}, temperature is  {}, with an humidity of {} feels like {}  today minimal temp is {} and max is {}",x, data.wind_speed, data.wind_degrees, data.temp, data.humidity, data.feels_like, data.min_temp ,data.max_temp)).await?;
    Ok(())
}

#[derive(Deserialize, Debug, Clone)]
struct WeatherResponse {
    wind_speed: f64,
    wind_degrees: u16,
    temp: i32,
    humidity: i32,
    sunset: i64,
    min_temp: i32,
    cloud_pct: i32,
    feels_like: i32,
    sunrise: i64,
    max_temp: i32,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    // let framework = StandardFramework::new().group(&GENERAL_GROUP);
    // framework.configure(Configuration::new().prefix("!")); // set the bot's prefix to "~"
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                age(),
                current_time(),
                get_weather(),
                cr(),
                cr_ca(),
                cr_di(),
                cr_parent(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::all();

    let mut client = Client::builder(token, intents)
        // .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

// #[command]
// async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
//     msg.reply(ctx, "Pong!").await?;

//     Ok(())
// }

// #[command]
// async fn am_i_admin(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
//     let is_admin = if let (Some(member), Some(guild)) = (&msg.member, msg.guild(&ctx.cache)) {
//         member.roles.iter().any(|role| {
//             guild
//                 .roles
//                 .get(role)
//                 .is_some_and(|r| r.has_permission(Permissions::ADMINISTRATOR))
//         })
//     } else {
//         false
//     };

//     if is_admin {
//         msg.channel_id.say(&ctx.http, "Yes, you are.").await?;
//     } else {
//         msg.channel_id.say(&ctx.http, "No, you are not.").await?;
//     }

//     Ok(())
// }
// #[command]
// async fn am_i_cool(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
//     if msg.author.id.to_string().as_str() == "122111855219965954" {
//         msg.reply(&ctx.http, "No you are not").await?;
//     } else {
//         msg.reply(&ctx.http, "Yes you are").await?;
//     }

//     Ok(())
// }

// #[command]
// async fn random_quote(ctx: &Context, msg: &Message) -> CommandResult {
//     let req: reqwest::Client = reqwest::Client::new();
//     let res = req
//         .get("https://api.api-ninjas.com/v1/quotes?category=happiness")
//         .header("X-Api-Key", "AkW2CJ7sXNWjKVtACMI8GGYIYBE4XU8haDniaYUt")
//         .send()
//         .await?;

//     println!("{:?}", res.status());
//     let data: Vec<QuoteResponse> = res.json().await?;
//     msg.channel_id
//         .say(
//             &ctx.http,
//             format!("{} by: Author: {}", data[0].quote, data[0].author),
//         )
//         .await?;

//     Ok(())
// }

// #[derive(Deserialize, Debug, Clone)]
// struct QuoteResponse {
//     quote: String,
//     author: String,
//     _category: String,
// }
// #[command]
// async fn test(_ctx: &Context, msg: &Message) -> CommandResult {
//     let content = &msg.content;
//     println!("{}", content);
//     Ok(())
// }
