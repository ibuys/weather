use config::Config;
use std::collections::HashMap;
use serde_json::Value;
use dirs;


fn weatherconfigs() -> HashMap<String, String> {

    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let config_path = home_dir.join(".weather.toml");

    let settings = Config::builder()
        .add_source(config::File::from(config_path))
        .build()
        .unwrap();

    settings.try_deserialize::<HashMap<String, String>>().unwrap()
}

fn value_to_num(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::Number(num) => Some(num.to_string()),
        _ => None, // Return None for other types
    }
}

fn icon_emoji(icon: &str) -> &'static str {
    match icon {
        "01d" => "☀️",
        "01n" => "🌕",
        "02d" | "03d" | "04d" => "🌤",
        "02n" | "03n" | "04n" => "☁️",
        "09d" | "10d" => "🌦️",
        "09n" | "10n" => "⛈️",
        "13d" | "13n" => "❄️",
        "50d" => "🌪️",
        "50n" => "🌫",
        _ => "",
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {


    // Call weatherconfigs() to get configurations
    let configs = weatherconfigs();

    // Retrieve values from configs HashMap and assign them to variables
    let lat = configs.get("lat").unwrap();
    let long = configs.get("long").unwrap();
    let apikey = configs.get("apikey").unwrap();
        
    let request_url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units=imperial",
        lat, long, apikey
    );

    let body = reqwest::get(&request_url)
        .await?
        .text()
        .await?;

    let v: Value = serde_json::from_str(&body)?;

    if let Some(summary) = &v["weather"][0]["description"].as_str() {
        if let Some(temperature) = value_to_num(&v["main"]["temp"]) {
            if let Some(icon) = &v["weather"][0]["icon"].as_str() {
                println!("{}  {} {}°F", icon_emoji(icon), summary, temperature);
            }
        }
    }
    
    Ok(())
}