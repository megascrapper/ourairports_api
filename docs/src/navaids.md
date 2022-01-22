# Navaids

> GET `/api/v1/navaids`
> 
> GET `/api/v1/navaids/:id` (for individual navigation aid)

## Schema

```json
[
  {
    "id": "Uint64",
    "filename": "String",
    "ident": "String",
    "name": "String",
    "type": "String",
    "frequency_khz": "String",
    "latitude_deg": "Float64?",
    "longitude_deg": "Float64?",
    "elevation_ft": "Int32?",
    "iso_country": "String",
    "dme_frequency_khz": "String",
    "dme_channel": "String",
    "dme_latitude_deg": "Float64?",
    "dme_longitude_deg": "Float64?",
    "dme_elevation_ft": "Int32?",
    "slaved_variation_deg": "Float64?",
    "magnetic_variation_deg": "Float64?",
    "usageType": "String?",
    "power": "String?",
    "associated_airport": "String"
  }
]
```

`/api/v1/navaids/:id` are not contained in an array (i.e. only returns the object).

## Examples

> GET `/api/v1/navaids/86738`

```json
{
    "id": 86738,
    "filename": "Christchurch_VOR-DME_NZ",
    "ident": "CH",
    "name": "Christchurch",
    "type": "VOR-DME",
    "frequency_khz": "115300",
    "latitude_deg": -43.50410079956055,
    "longitude_deg": 172.51499938964844,
    "elevation_ft": 123,
    "iso_country": "NZ",
    "dme_frequency_khz": "115300",
    "dme_channel": "100X",
    "dme_latitude_deg": -43.5042,
    "dme_longitude_deg": 172.515,
    "dme_elevation_ft": 140,
    "slaved_variation_deg": 23.007,
    "magnetic_variation_deg": 23.188,
    "usageType": "BOTH",
    "power": "HIGH",
    "associated_airport": "NZCH"
}
```