# ourairports_api

This repository contains the library for using [OurAirports open data](https://ourairports.com/data/) in Rust and  a 
website providing JSON REST API of OurAirports data.

# Installation (web app)

## Docker
```
docker-compose up -d
```

## Manual installation
```
cargo build --release --bin web
```
Then go to `target/release` to find the executable (usually named `web` or `web.exe` on Windows)

# Limitations/feature wishlist

Currently, the web app only downloads the data during startup, and there is no auto-updating mechanism in place once 
the app has started. Since OurAirports data is updated mostly daily, it is recommended that you restart the app at least
daily (e.g. through a cron job) so you get the latest data.