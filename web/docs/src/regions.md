# Regions

> GET `/api/v1/regions`
> 
> GET `/api/v1/regions/:id` (for individual region)

## Schema

```json
[
  {
    "id": 306408,
    "code": "GE-KA",
    "local_code": "KA",
    "name": "Kakheti",
    "continent": "AS",
    "iso_country": "GE",
    "wikipedia_link": "https://en.wikipedia.org/wiki/Kakheti",
    "keywords": [
        "Kakhet'i",
        "კახეთი"
    ]
  }
]
```

`/api/v1/regions/:id` are not contained in an array (i.e. only returns the object).

## Parameters

- `iso_country`: The ISO country code
- `code`: the combined country code-local code
- `local_code`

## Examples

> GET `/api/v1/regions/306408`

```json
{
    "id": 306408,
    "code": "GE-KA",
    "local_code": "KA",
    "name": "Kakheti",
    "continent": "AS",
    "iso_country": "GE",
    "wikipedia_link": "https://en.wikipedia.org/wiki/Kakheti",
    "keywords": [
        "Kakhet'i",
        "კახეთი"
    ]
}
```