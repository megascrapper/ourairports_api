# Airport frequencies

> GET `/api/v1/airport-frequencies`
> 
> GET `/api/v1/airport-frequencies/:id` (for individual airport frequency)

## Schema

```json
[
  {
    "id": "Uint64",
    "airport_ref": "Uint64",
    "airport_ident": "String",
    "type": "String",
    "description": "String",
    "frequency_mhz": "String"
  }
]
```

`/api/v1/airport-frequencies/:id` are not contained in an array (i.e. only returns the object).

## Parameters
- `airport_ref`
- `airport_ident`

## Examples

> GET `/api/v1/airport-frequencies/54836` 

```json
{
    "id": 54836,
    "airport_ref": 3206,
    "airport_ident": "HKJK",
    "type": "TWR",
    "description": "Nairobi Tower",
    "frequency_mhz": "118.7"
}
```