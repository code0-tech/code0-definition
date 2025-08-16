
## HTTP Method - Type
```json
{
  "variant": 2,
  "identifier": "HTTP_METHOD",
  "name": [
    {
      "code": "en-US",
      "content": "HTTP Method"
    }
  ],
  "rules": [
    {
      "config": {
        "ItemOfCollection": {
          "items": [
            {"kind":{"StringValue":"GET"}},
            {"kind":{"StringValue":"POST"}},
            {"kind":{"StringValue":"PUT"}},
            {"kind":{"StringValue":"DELETE"}},
            {"kind":{"StringValue":"PATCH"}},
            {"kind":{"StringValue":"HEAD"}}
          ]
        }
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
      "content": "HTTP Route"
    }
  ],
  "rules": [
    {
      "config": {
        "Regex": {
          "pattern": "/^\/\\w+(?:[.:~-]\\w+)*(?:\/\\w+(?:[.:~-]\\w+)*)*$/"
        }
      }
    }
  ],
  "generic_keys": []
}
```
