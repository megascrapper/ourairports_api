# Airport frequencies

> GET `/api/v1/airport-frequencies`

## Schema

```js
[
    {
        "id": String,
        "airport": {
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
        },
        "type": String,
        "description", String,
        "frequency_mhz", String
    }
]
```
