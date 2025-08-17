```json
{
  "variant": 5,
  "identifier": "HTTP_HEADER_MAP",
  "name": [
    {
      "code": "en-US",
      "content": "HTTP Headers"
    }
  ],
  "rules": [
    {
      "config": {
        "ParentType": {
          "parent_type": {
            "type": {
              "GenericType": {
                "data_type_identifier": "ARRAY",
                "generic_mappers": [
                  {
                    "source": [
                      {
                        "type": {
                          "DataTypeIdentifier": "HTTP_HEADER_ENTRY"
                        }
                      }
                    ],
                    "target": "T",
                    "generic_combinations": []
                  }
                ]
              }
            }
          }
        }
      }
    }
  ],
  "generic_keys": []
}
```
