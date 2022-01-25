# Airports

> GET `/api/v1/airports`
> 
> GET `/api/v1/airports/:id` (for individual airports)

## Schema

```json
[
    {
        "id": "Uint64",
        "ident": "String",
        "type": "String",
        "name": "String",
        "latitude_deg": "Float64",
        "longitude_deg": "Float64",
        "elevation_ft": "Int32?",
        "continent": "String",
        "iso_country": "String",
        "iso_region": "String",
        "municipality": "String",
        "scheduled_service": "Bool",
        "gps_code": "String",
        "iata_code": "String",
        "local_code": "String",
        "home_link": "String",
        "wikipedia_link": "String",
        "keywords": ["String"]
    }
]
```

`/api/v1/airports/:id` are not contained in an array (i.e. only returns the object).

## Parameters
- `ident`: the text identifier used in the OurAirports URL (corresponds to the `ident` field)
- `iso_country`
- `iso_region`
- `gps_code`
- `iata_code`
- `local_code`

## Example

> GET `/api/v1/airports/2434`

```json
{
    "id": 2434,
    "ident": "EGLL",
    "type": "large_airport",
    "name": "London Heathrow Airport",
    "latitude_deg": 51.4706,
    "longitude_deg": -0.461941,
    "elevation_ft": 83,
    "continent": "EU",
    "iso_country": "GB",
    "iso_region": "GB-ENG",
    "municipality": "London",
    "scheduled_service": true,
    "gps_code": "EGLL",
    "iata_code": "LHR",
    "local_code": "",
    "home_link": "http://www.heathrowairport.com/",
    "wikipedia_link": "https://en.wikipedia.org/wiki/Heathrow_Airport",
    "keywords": [
        "LON",
        "Londres"
    ]
}
```