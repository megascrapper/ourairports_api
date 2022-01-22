# Runways

> GET `/api/v1/runways`
> 
> GET `/api/v1/runways/:id` (for individual runway)

## Schema

```json
[
  {
    "id": "Uint64",
    "airport_ref": "Uint64",
    "airport_ident": "String",
    "length_ft": "Int32?",
    "width_ft": "Int32?",
    "surface": "String",
    "lighted": "Bool",
    "closed": "Bool",
    "le_ident": "String",
    "le_latitude_deg": "Float64?",
    "le_longitude_deg": "Float64?",
    "le_elevation_ft": "Int32?",
    "le_heading_degT": "Float64?",
    "le_displaced_threshold_ft": "Int32?",
    "he_ident": "String",
    "he_latitude_deg": "Float64?",
    "he_longitude_deg": "Float64?",
    "he_elevation_ft": "Int32?",
    "he_heading_degT": "Float64?",
    "he_displaced_threshold_ft": "Int32?"
  }
]
```

`/api/v1/runways/:id` are not contained in an array (i.e. only returns the object).

## Examples

> GET `/api/v1/runways/234512`

```json
{
    "id": 234512,
    "airport_ref": 1941,
    "airport_ident": "CYVR",
    "length_ft": 9940,
    "width_ft": 200,
    "surface": "CON",
    "lighted": true,
    "closed": false,
    "le_ident": "08L",
    "le_latitude_deg": 49.205,
    "le_longitude_deg": -123.201,
    "le_elevation_ft": 13,
    "le_heading_degT": 100.0,
    "le_displaced_threshold_ft": null,
    "he_ident": "26R",
    "he_latitude_deg": 49.2006,
    "he_longitude_deg": -123.16,
    "he_elevation_ft": 9,
    "he_heading_degT": 280.0,
    "he_displaced_threshold_ft": null
}
```


