# JSON Schema

## Object
```json
{
  "type": "object",
  "properties": {
    "number":      { "type": "number" },
    "street_name": { "type": "string" },
    "street_type": { "type": "string",
                     "enum": ["Street", "Avenue", "Boulevard"]
                   }
  }
}
```

## Array
### length
```json
{
  "type": "array",
  "minItems": 2,
  "maxItems": 3
}
```
[ref](http://spacetelescope.github.io/understanding-json-schema/reference/array.html#length)
