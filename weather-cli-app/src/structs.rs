use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeoLocation {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub country: String,
    pub state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherRoot {
    #[serde(rename = "current_weather")]
    pub current_weather: CurrentWeather,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentWeather {
    pub temperature: f64,
    pub windspeed: f64,
    pub winddirection: f64,
    pub weathercode: f64,
    pub time: String,
}