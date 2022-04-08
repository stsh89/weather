mod forecast;
mod providers;

use providers::OpenWeather;

fn main() {
    let request = forecast::Request {
        latitude: 51.5073219,
        longitude: -0.1276474,
    };

    let result = forecast::show(
        Box::new(OpenWeather {
            appid: "".to_string(),
        }),
        request,
    );

    match result {
        Ok(weather) => println!("{}", weather.temperature),
        Err(error) => println!("{:?}", error),
    }
}
