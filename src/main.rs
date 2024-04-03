#![allow(unused)]
use chrono::DateTime;
use poise::serenity_prelude as serenity;

use ::serenity::{all::ChannelType, builder::CreateChannel};

use serde::Deserialize;
use serenity::prelude::*;

use std::env;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

use crate::car_types::Car;
mod car_types;

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
        // .header("X-Api-Key", "AkW2CJ7sXNWjKVtACMI8GGYIYBE4XU8haDniaYUt")
        .send()
        .await?;
    let data: WeatherResponse = response.json().await?;

    ctx.say(format!("Weather at {} :Current Wind {} , at {}, temperature is  {}, with an humidity of {} feels like {}  today minimal temp is {} and max is {}",x, data.wind_speed, data.wind_degrees, data.temp, data.humidity, data.feels_like, data.min_temp ,data.max_temp)).await?;
    Ok(())
}
#[poise::command(slash_command, prefix_command)]
async fn lock(
    ctx: Context<'_>,
    #[description = "Unlock or Lock"] lock: String,
) -> Result<(), Error> {
    if ctx.author().id == 139886040524652544 {
        if lock == "unlock" {
            let client = reqwest::Client::new();
            let car_response = client
                .get("https://api.tessie.com/LRW3E7FS6RC040530/command/unlock?retry_duration=40&wait_for_completion=true")
                .header(
                    "Authorization",
                    format!("Bearer {}", env::var("TESSIE_KEY").unwrap()),
                )
                .send()
                .await?;
            if car_response.status() != 200 {
                ctx.say("Failed to lock the car").await?;
                return Ok(());
            }
            ctx.say("Jessie is now unlocked").await?;
        } else {
            let client = reqwest::Client::new();
            let car_response = client
            .get("https://api.tessie.com/LRW3E7FS6RC040530/command/lock?retry_duration=40&wait_for_completion=true")
            .header(
                "Authorization",
                format!("Bearer {}", env::var("TESSIE_KEY").unwrap()),
            )
            .send()
            .await?;
            if car_response.status() != 200 {
                ctx.say("Failed to lock the car").await?;
                return Ok(());
            }
            ctx.say("Jessie is now locked").await?;
        }
    } else {
        ctx.say("You are not authorized to lock the car").await?;
    }

    Ok(())
}
#[poise::command(slash_command, prefix_command)]
async fn battery(ctx: Context<'_>) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let car_response = client
        .get("https://api.tessie.com/LRW3E7FS6RC040530/battery")
        .header(
            "Authorization",
            format!("Bearer {}", env::var("TESSIE_KEY").unwrap()),
        )
        .send()
        .await?;
    let car_data: CarBattery = car_response.json().await?;

    _ = ctx
        .say(format!(
            "Jessie Battery is {}% at {}",
            car_data.battery_level,
            DateTime::from_timestamp(car_data.timestamp, Default::default())
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
        ))
        .await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn car_state(ctx: Context<'_>) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let car_response = client
        .get("https://api.tessie.com/LRW3E7FS6RC040530/state?use_cache=true")
        .header(
            "Authorization",
            format!("Bearer {}", env::var("TESSIE_KEY").unwrap()),
        )
        .send()
        .await?;
    let car_data: Car = car_response.json().await?;

    _ = ctx
        .say(format!(
            "{} Battery is {}% at {}",
            car_data.display_name,
            car_data.charge_state.battery_level,
            DateTime::from_timestamp_millis(car_data.charge_state.timestamp)
                .unwrap()
                .format("%H:%M:%S %d-%m-%Y")
                .to_string()
        ))
        .await?;
    ctx.say(format!(
        "{} is in {} state",
        car_data.display_name, car_data.state
    ))
    .await?;
    ctx.say(format!(
        "Charge State:
        Battery Heater On: {}
        Battery Level: {}
        Battery Range: {}
        Charge Amps: {}
        Charge Current Request: {}
        Charge Current Request Max: {}
        Charge Enable Request: {}
        Charge Energy Added: {}
        Charge Limit SOC: {}
        Charge Limit SOC Max: {}
        Charge Limit SOC Min: {}
        Charge Limit SOC Std: {}
        Charge Miles Added Ideal: {}
        Charge Miles Added Rated: {}
        Charge Port Cold Weather Mode: {}
        Charge Port Color: {}
        Charge Port Door Open: {}
        Charge Port Latch: {}
        Charge Rate: {}
        Charger Actual Current: {}
        Charger Phases: {}
        Charger Pilot Current: {}
        Charger Power: {}
        Charger Voltage: {}
        Charging State: {}
        Estimated Battery Range: {}
        Fast Charger Present: {}
        Fast Charger Type: {}
        Ideal Battery Range: {}
        Managed Charging Active: {}
        Minutes To Full Charge: {}
        Not Enough Power To Heat: {}
        Scheduled Charging Pending: {}
        Scheduled Charging Start Time: {}
        Scheduled Charging Start Time App: {}
        Scheduled Departure Time: {}
        Scheduled Departure Time Minutes: {}
        Supercharger Session Trip Planner: {}
        Time To Full Charge: {}
        Usable Battery Level: {}
        ",
        car_data.charge_state.battery_heater_on,
        car_data.charge_state.battery_level,
        car_data.charge_state.battery_range,
        car_data.charge_state.charge_amps,
        car_data.charge_state.charge_current_request,
        car_data.charge_state.charge_current_request_max,
        car_data.charge_state.charge_enable_request,
        car_data.charge_state.charge_energy_added,
        car_data.charge_state.charge_limit_soc,
        car_data.charge_state.charge_limit_soc_max,
        car_data.charge_state.charge_limit_soc_min,
        car_data.charge_state.charge_limit_soc_std,
        car_data.charge_state.charge_miles_added_ideal,
        car_data.charge_state.charge_miles_added_rated,
        car_data.charge_state.charge_port_cold_weather_mode,
        car_data.charge_state.charge_port_color,
        car_data.charge_state.charge_port_door_open,
        car_data.charge_state.charge_port_latch,
        car_data.charge_state.charge_rate,
        car_data.charge_state.charger_actual_current,
        car_data.charge_state.charger_phases.unwrap_or(0),
        car_data.charge_state.charger_pilot_current,
        car_data.charge_state.charger_power,
        car_data.charge_state.charger_voltage,
        car_data.charge_state.charging_state,
        car_data.charge_state.est_battery_range,
        car_data.charge_state.fast_charger_present,
        car_data.charge_state.fast_charger_type,
        car_data.charge_state.ideal_battery_range,
        car_data.charge_state.off_peak_charging_enabled,
        car_data.charge_state.minutes_to_full_charge,
        car_data
            .charge_state
            .not_enough_power_to_heat
            .as_ref()
            .unwrap_or(&"None".to_string()),
        car_data.charge_state.scheduled_charging_pending,
        car_data
            .charge_state
            .scheduled_charging_start_time
            .unwrap_or(0),
        car_data.charge_state.scheduled_charging_start_time_app,
        car_data.charge_state.scheduled_departure_time,
        car_data.charge_state.scheduled_departure_time_minutes,
        car_data.charge_state.supercharger_session_trip_planner,
        car_data.charge_state.time_to_full_charge,
        car_data.charge_state.usable_battery_level,
    ))
    .await?;
    ctx.say(format!(
        "Climate State:
        Allow Cabin Overheat Protection: {}
        Auto Seat Climate Left : {}
        Auto Seat Climate Right: {}
        Auto Steering Wheel Heater: {}
        Battery Heater: {}
        Cabin Overheat Protection: {}
        Cabin Overheat Protection Active: {}
        Climate Keeper Mode: {}
        Defrost Mode: {}
        Driver Temperature Setting: {}
        Fan Status: {}
        Inside Temp: {}
        Is Auto Conditioned Air On: {}
        Is Climate On: {}
        Is Front Defroster On: {}
        Is Preconditioning: {}
        Is Rear Defroster On: {}
        Is Side Mirror Heater On: {}
        ",
        car_data.climate_state.is_climate_on,
        car_data.climate_state.auto_seat_climate_left,
        car_data.climate_state.auto_seat_climate_right,
        car_data.climate_state.auto_steering_wheel_heat,
        car_data.climate_state.battery_heater,
        car_data.climate_state.cabin_overheat_protection,
        car_data
            .climate_state
            .cabin_overheat_protection_actively_cooling,
        car_data.climate_state.climate_keeper_mode,
        car_data.climate_state.defrost_mode,
        car_data.climate_state.driver_temp_setting,
        car_data.climate_state.fan_status,
        car_data.climate_state.inside_temp,
        car_data.climate_state.is_auto_conditioning_on,
        car_data.climate_state.is_climate_on,
        car_data.climate_state.is_front_defroster_on,
        car_data.climate_state.is_preconditioning,
        car_data.climate_state.is_rear_defroster_on,
        car_data.climate_state.side_mirror_heaters,
    ))
    .await?;
    Ok(())
}

#[derive(Deserialize, Debug, Clone)]
struct CarBattery {
    battery_level: u32,
    timestamp: i64,
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
                battery(),
                car_state(),
                lock(),
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
