
## HTTP Method - Type
```json
{
  "variant": 2,
  "identifier": "HTTP_METHOD",
  "name": [
    {
      "code": "en-US",
      "content": "HTTP Method",
    }
  ],
  "rules": [
    {
      "item_of_collection": {
        "items": [ "GET", "POST", "PUT", "DELETE", "PATCH", "HEAD"]
      }
    }
  ],
  "generic_keys": []
}
```

## HTTP URL - Type

```json
{
  "variant": 2,
  "identifier": "HTTP_URL",
  "name": [
    {
      "code": "en-US",
      "content": "HTTP Route",
    }
  ],
  "rules": [
    {
      "regex": {
        "pattern": "/^\/\w+(?:[.:~-]\w+)*(?:\/\w+(?:[.:~-]\w+)*)*$/"
      }
    }
  ],
  "generic_keys": []
}
```
