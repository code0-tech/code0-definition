```json
{
  "variant": "OBJECT",
  "identifier": "HTTP_HEADER_ENTRY",
  "name": [
    {
      "code": "en-US",
      "content": "HTTP Header Entry"
    }
  ],
  "rules": [
    {
      "contains_key": {
        "key": "key",
        "type": "TEXT"
      }
    },
    {
      "contains_key": {
        "key": "value",
        "type": "TEXT"
      }
    }
  ],
  "parent_type_identifier": "OBJECT"
}
```

```json
{
  "variant": "OBJECT",
  "identifier": "HTTP_REQUEST_OBJECT",
  "name": [
    {
      "code": "en-US",
      "content": "HTTP Request",
    }
  ],
  "rules": [
    {
      "contains_key": {
        "key": "method",
        "type": "HTTP_METHOD"
      }
    },
    {
      "contains_key": {
        "key": "url",
        "type": "HTTP_URL"
      }
    },
    {
      "contains_key": {
        "key": "body",
        "type": "OBJECT"
      }
    },
    {
      "contains_key": {
        "key": "headers",
        "type": "HTTP_HEADER_MAP"
      }
    }
  ],
  "parent_type_identifier": "OBJECT"
}
```

```json
{
  "variant": "OBJECT",
  "identifier": "HTTP_RESPONSE_OBJECT",
  "name": [
    {
      "code": "en-US",
      "content": "HTTP Response"
    }
  ],
  "rules": [
    {
      "contains_key": {
        "key": "headers",
        "type": "HTTP_HEADER_MAP"
      }
    },
    {
      "contains_key": {
        "key": "body",
        "type": "OBJECT"
      }
    }
  ],
  "parent_type_identifier": "OBJECT"
}
```
