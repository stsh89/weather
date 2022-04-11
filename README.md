# Weather
It is a weather CLI for Linux/MacOS, which is responsible for showing weather to a user. The target consumer of the CLI is a human. Currently it uses following weather providers:

1. https://openweathermap.org/
2. https://www.weatherapi.com/

You can register there and obtain credentials that can be further used within app.

## Usage

Common usage flow, would be as following:

        weather list-providers

This command outputs list of available providers, which names you can use further within command line. Current provider names are: open_weather and weatherapi.

Next step would be to configure provider, for example:

        weather configure openweather

User would be prompted to enter api key for the given provider.
Next step would be to set current provider.

        weather set-provider openweather

Which can be verified by

        weather current-provider

After configuration and setting provider, user can get actual weather data, for example following command would show temperature for France now:

        weather get Paris,FR

or, temperature for France on Apr, 12:

        weather get Paris,FR 2022-04-12

## Limitations
Only supported address kinds for now are like <City,CountryCode>, for example: Paris,FR, Dnipro,UA, etc.

Only supported date format is YYYY-mm-dd, for example 2022-04-12, 2023-01-01, etc. Also you cannot check weather for all dates, providers limits their free capabilities to 3-7 days. So actually you can check weather only for several days ahead.


## Full command list
```
    weather <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    configure           Configure weather provider
    current-provider    Show selected provider
    get                 Get weather information,
    help                Print this message or the help of the given subcommand(s)
    list-providers      List available weather providers
    set-provider        Set weather provider that will provide weather information
    show-provider       Show configuration for selected provider
```
