use crate::utilities::geo_utils::get_coordinates;

use chrono::prelude::*;

use itertools::Itertools;

use reqwest::blocking::Client;
use reqwest::Url;

use serde::Deserialize;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

use std::env;

#[derive(Debug, Deserialize)]
struct Response {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    pub currently: Currently,
    pub daily: Daily,
}

#[derive(Debug, Deserialize)]
struct Currently {
    time: i64,
    summary: String,
    icon: String,
    #[serde(rename = "nearestStormDistance")]
    nearest_storm_distance: Option<usize>,
    temperature: f64,
    #[serde(rename = "dewPoint")]
    dew_point: f64,
    humidity: f64,
    #[serde(rename = "windSpeed")]
    wind_speed: f64,
    #[serde(rename = "uvIndex")]
    uv_index: u8,
    visibility: f64,
}

#[derive(Debug, Deserialize)]
struct Daily {
    summary: String,
    icon: String,
    data: Vec<DailyData>,
}

#[derive(Debug, Deserialize)]
struct DailyData {
    time: i64,
    summary: String,
    #[serde(rename = "temperatureHigh")]
    high: f64,
    #[serde(rename = "temperatureLow")]
    low: f64,
}

#[command]
#[description = "Looks up the forecast for a specified location."]
#[usage = "<location>"]
#[aliases("weather", "forecast")]
pub fn weather(context: &mut Context, message: &Message, arguments: Args) -> CommandResult {
    if arguments.rest().is_empty() {
        message.channel_id.send_message(&context, |m| {
            m.embed(|e| {
                e.title("Error: Invalid location name provided.");
                e.description("You have provided an invalid location. Please try again.");
                e.color(0x00FF_0000)
            })
        })?;
        return Ok(());
    }

    let location = arguments.rest().to_string();
    let user_agent: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    let api_key = env::var("DARKSKY").expect("Couldn't acccess the provided API key.").to_string();

    let coordinates = get_coordinates(location)?;
    let coordinates_result = coordinates.results.first().unwrap();
    let address = &coordinates_result.formatted_address;
    let latitude = coordinates_result.geometry.location.latitude;
    let longitude = coordinates_result.geometry.location.longitude;

    let client = Client::builder().user_agent(user_agent).build()?;
    let url = Url::parse(format!("https://api.darksky.net/forecast/{}/{},{}", api_key, latitude, longitude).as_str())?;
    let request: Response = client.get(url).query(&[("units", "si")]).send()?.json()?;

    let summary = request.daily.summary;
    let icon = format!("https://darksky.net/images/weather-icons/{}.png", request.currently.icon);
    let condition = &request.daily.data.first().unwrap().summary;

    let temperature = request.currently.temperature.trunc().round();
    let temperature_f = temperature * 1.8 + 32.0;
    let temperature_high = request.daily.data.first().unwrap().high.round();
    let temperature_high_f = temperature_high * 1.8 + 32.0;
    let temperature_low = request.daily.data.first().unwrap().low.round();
    let temperature_low_f = temperature_low * 1.8 + 32.0;
    let humidity = request.currently.humidity * 100.0;
    let visibility = request.currently.visibility.round();
    let wind_speed = request.currently.wind_speed;
    let wind_speed_km = request.currently.wind_speed.round() * 3.6;
    let uv_index = request.currently.uv_index;

    let forecast = request.daily.data[0..5].iter().map(|d: &DailyData| {
        let day = NaiveDateTime::from_timestamp(d.time, 0).format("%A");
        let summary = &d.summary;
        let temp_high = &d.high.round();
        let temp_high_f = temp_high * 1.8 + 32.0;
        let temp_low = &d.low.round();
        let temp_low_f = temp_low * 1.8 + 32.0;
        format!("**{}**: {} ({} °C | {} °F, {} °C | {} °F)", day, summary, temp_high, temp_high_f.round(), temp_low, temp_low_f.round())
    }).join("\n");

    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.author(|author| {
                author.name(address);
                author.icon_url(icon)
            });
            embed.color(0x8cbed6);
            embed.description(format!(
                "{}\n\n\
                **Current Condition**: {}\n\
                **Current Temperature**: {} °C | {} °F\n\
                **Today's High**: {} °C | {} °F\n\
                **Today's Low**: {} °C | {} °F\n\
                **Visibility**: {} km/h\n\
                **Humidity**: {}%\n\
                **Wind Speed**: {} m/s | {} km/h\n\
                **UV Index**: {}\n\n\
                **5 day forecast**:\n\
                {}",
                summary,
                condition,
                temperature,
                temperature_f.round(),
                temperature_high,
                temperature_high_f.round(),
                temperature_low,
                temperature_low_f.round(),
                visibility,
                humidity.round(),
                wind_speed,
                wind_speed_km,
                uv_index,
                forecast
            ));
            embed.footer(|f| f.text("Powered by Dark Sky."))
        })
    })?;

    return Ok(());
}
