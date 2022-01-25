# ourairports_api

This crate contains the website providing JSON REST API of OurAirports data.

# Installation (web app)

## Docker

```
docker-compose up -d
```

## Manual installation

### Requirements

* Rust
* mdbook (`cargo install mdbook`)

### Steps

This guide assumes current working directory is `web`.

1. Build the application binary:

```
cargo build --release -p ourairports_api
```

2. `cd` to `docs` folder and build the JSON API docs:

```
mdbook build
```

3. Copy the contents of `book` folder to `../static/docs`

In the end you should have the following folder structure:

```
web
├── docs
├── src
└── static
    ├── docs
    ├── index.html
    └── starter-template.css
```
4. Run with `cargo run --release`


If you want to run without `cargo`, the `static` directory must be on the same directory as the executable, so it should look like this:

```
<current_dir>
├── ourairports_api(.exe)
└── static
    ├── docs
    ├── index.html
    └── starter-template.css
```

# Limitations/feature wishlist

Currently, the web app only downloads the data during startup, and there is no auto-updating mechanism in place once the
app has started. Since OurAirports data is updated mostly daily, it is recommended that you restart the app at least
daily (e.g. through a cron job) so you get the latest data.