# Countries

> GET `/api/v1/countries`
> 
> GET `/api/v1/countries/:id` (for individual country)

## Schema

```json
[
  {
    "id": "Uint64",
    "code": "String",
    "name": "String",
    "continent": "String",
    "wikipedia_link": "String",
    "keywords": ["String"]
  }
]
```

`/api/v1/countries/:id` are not contained in an array (i.e. only returns the object).

## Parameters

- `code`: The ISO country code
- `name`

## Examples

> GET `/api/v1/countries/302791` 

```json
{
    "id": 302791,
    "code": "BR",
    "name": "Brazil",
    "continent": "SA",
    "wikipedia_link": "https://en.wikipedia.org/wiki/Brazil",
    "keywords": [
        "Brasil",
        "Brasilian"
    ]
}
```