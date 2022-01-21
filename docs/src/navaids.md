# Navaids

> GET `/api/v1/navaids`

## Schema

```js
[
    {
        "id": String,
        "filename": String,
        "ident": String,
        "name": String,
        "type": String,
        "frequency_khz": String,
        "coordinates": {
            "latitude_deg": Float64,
            "longitude_deg": Float64,
            "elevation_ft": Int32?
        },
        "country": {
            "id": String,
            "code": String,
            "name": String,
            "continent": String,
            "wikipedia_link": String,
            "keywords": [String]
        },
        "dme_freuquency_khz": String,
        "dme_channel": String,
        "dme_coordinates": ({
            "latitude_deg": Float64,
            "longitude_deg": Float64,
            "elevation_ft": Int32?
        })?,
        "slaved_variation_deg": Float64?,
        "magnetic_variation_deg": Float64?,
        "usage_type": String,
        "power": String,
        "associated_airport": {
            "id": String,
            "ident": String,
            "type": String,
            "name": String,
            "coordinates": {
                "latitude_deg": Float64,
                "longitude_deg": Float64,
                "elevation_ft": Int32?
            },
            "continent": String,
            "country": {
                "id": String,
                "code": String,
                "name": String,
                "continent": String,
                "wikipedia_link": String,
                "keywords": [String]
            },
            "region": {
                "id": String,
                "code": String,
                "local_code": String,
                "name": String,
                "continent": String,
                "country": {
                    "id": String,
                    "code": String,
                    "name": String,
                    "continent": String,
                    "wikipedia_link": String,
                    "keywords": [String]
                },
                "wikipedia_link": String,
                "keywords": [String]
            }
            "municipality": String,
            "scheduled_service": Bool,
            "gps_code": String?,
            "iata_code": String?,
            "local_code": String?,
            "wikipedia_link": String?,
            "keywords": [String]
        }
    }
]
```