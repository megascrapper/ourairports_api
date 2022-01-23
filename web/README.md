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

1. Create a new directory, like `app`.
2. Copy the static folder in the project root directory to `app`.
3. Build the application binary:

```
cargo build --release -p ourairports_api
```

4. Place the resulting executable (usually in `target/release/ourairports_api` or `target/release/ourairports_api.exe`)
   to the root of `app` directory.
5. Build the rustdoc files:

```
cargo doc
```

6. Place the output directory (usually in `target/doc`) to `app/static/rust-docs`
7. `cd` to `docs` folder in project and build the JSON API docs:

```
mdbook build
```

8. Copy the contents of `book` folder to `app/static/docs`

You should have the following final folder structure:

```
app/
├─ static/
│  ├─ docs/
│  ├─ rust-docs/
│  ├─ index.html
├─ web

```

# Limitations/feature wishlist

Currently, the web app only downloads the data during startup, and there is no auto-updating mechanism in place once the
app has started. Since OurAirports data is updated mostly daily, it is recommended that you restart the app at least
daily (e.g. through a cron job) so you get the latest data.