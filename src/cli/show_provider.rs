use super::config::Config;

pub fn run(config: Config, name: &str) {
    match name {
        "dummy" => dummy_config(config),
        "open_weather" => open_weather_config(config),
        _ => println!("Invalid provider name!"),
    }
}

fn dummy_config(config: Config) {
    let dummy = config.providers.dummy;
    println!("Latitude: {:?}", dummy.latitude);
    println!("Latitude: {:?}", dummy.longitude);
}

fn open_weather_config(config: Config) {
    let open_weather = config.providers.open_weather;
    println!("Appid: {:?}", open_weather.appid);
}
