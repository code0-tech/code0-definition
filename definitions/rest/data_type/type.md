
## HTTP Method - Type
```json
{
  "variant": "TYPE",
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
  "parent_type_identifier": null
}
```

## HTTP URL - Type

```json
{
  "variant": "TYPE",
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
  "parent_type_identifier": null
}
```
