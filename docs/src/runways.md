# Runways

> GET `/api/v1/runways`

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
        "length_ft": Int32?,
        "width_ft": Int32?,
        "surface": String,
        "lighted": Bool,
        "closed": Bool,
        "le_ident": String,
        "le_coordinates": {
            "latitude_deg": Float64,
            "longitude_deg": Float64,
            "elevation_ft": Int32?
        },
        "le_heading_deg_true": Float64?,
        "le_displaced_threshold_ft": Int32?,
        "le_ident": String,
        "he_coordinates": {
            "latitude_deg": Float64,
            "longitude_deg": Float64,
            "elevation_ft": Int32?
        },
        "he_heading_deg_true": Float64?,
        "he_displaced_threshold_ft": Int32?,
    }
]
```
