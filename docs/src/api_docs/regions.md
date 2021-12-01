# Regions

> GET `/api/v1/regions`

## Schema

```js
[
    {
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
]
```
