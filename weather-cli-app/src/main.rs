//Weather cli app - brain dump
// using a external package or crate in rust language
// take input from user
// hit an open weather api and get the weather
// multiline comments in rust just like C - done

//Thanks to these apis

//for geolocation co-ordinates
//https://api.openweathermap.org/geo/1.0/direct?q={city}&limit=5&appid={appid}

//for weather api
// https://api.open-meteo.com/v1/forecast?latitude=28.6353&longitude=77.2250&hourly=temperature_2m


mod structs;
use structs::{GeoLocation, CurrentWeather, WeatherRoot};
use serde_json;
use std::io;

extern crate dotenv;

#[macro_use]
extern crate dotenv_codegen;

fn take_input() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut buffer) {
        Ok(_) => buffer.to_string().strip_suffix('\n').unwrap().to_string(),
        Err(e) => e.to_string(),
    }
}

async fn get_coordinates(city: &str) -> Result<Vec<GeoLocation>, reqwest::Error> {
    // get your own app id here kid
    let appid = dotenv!("APP_ID");
    let request_url = format!(
        "https://api.openweathermap.org/geo/1.0/direct?q={city}&limit=5&appid={appid}",
        city = city,
        appid = appid
    );

    let response: String = reqwest::get(request_url).await?.text().await?;
    let decoded: Vec<GeoLocation> = serde_json::from_str(&response).unwrap();

    Ok(decoded)
}

async fn get_weather(lat: f64, lon: f64) -> Result<CurrentWeather, reqwest::Error> {
    let request_url = format!("https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}&current_weather=true", lat = lat, lon = lon);
    let response: String = reqwest::get(request_url).await?.text().await?;

    let decoded: WeatherRoot = serde_json::from_str(&response).unwrap();

    Ok(decoded.current_weather)
}

#[tokio::main]
async fn main() {
    println!("Welcome to the weather app");
    dotenv::from_filename(".env").ok();
    println!("Enter your city");
    let city = take_input();
    if city.len() < 3 {
        println!("How do you think I work, eh? \n More than 3 letters please");
    } else {
        let coordinates = match get_coordinates(&city).await {
        Ok(n) => n,
        Err(_e) => Vec::<GeoLocation>::with_capacity(0),
    };

    if coordinates.len() > 0 {
        let weather = match get_weather(coordinates[0].lat, coordinates[0].lon).await {
            Ok(n) => n,
            Err(_) => panic!("Something went wrong"), //poor implementation don't know what else to do
        };

        println!("The weather in {} is {}", city, weather.temperature);
    } else {
        println!("Sorry no city found for this name")
    }
    }
    
}
