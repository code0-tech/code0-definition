```json
{
  "variant": 3,
  "identifier": "HTTP_HEADER_ENTRY",
  "name": [
    {
      "code": "en-US",
      "content": "HTTP Header Entry"
    }
  ],
  "rules": [
    {
      "config": {
        "ContainsKey": {
          "key": "key",
          "data_type_identifier": {
            "type": {
              "DataTypeIdentifier": "TEXT"
            }
          }
        }
      }
    },
    {
      "config": {
        "ContainsKey": {
          "key": "value",
          "data_type_identifier": {
            "type": {
              "DataTypeIdentifier": "TEXT"
            }
          }
        }
      }
    },
    {
      "config": {
        "ParentType": {
          "parent_type": {
            "type": {
              "DataTypeIdentifier": "OBJECT"
            }
          }
        }
      }
    }
  ],
  "generic_keys": []
}
```

```json
{
  "variant": 3,
  "identifier": "HTTP_REQUEST_OBJECT",
  "name": [
    {
      "code": "en-US",
      "content": "HTTP Request"
    }
  ],
  "rules": [
    {
      "config": {
        "ContainsKey": {
          "key": "method",
          "data_type_identifier": {
            "type": {
              "DataTypeIdentifier": "HTTP_METHOD"
            }
          }
        }
      }
    },
    {
      "config": {
        "ContainsKey": {
          "key": "url",
          "data_type_identifier": {
            "type": {
              "DataTypeIdentifier": "HTTP_URL"
            }
          }
        }
      }
    }, 
    {
      "config": {
        "ContainsKey": {
          "key": "body",
          "data_type_identifier": {
            "type": {
              "DataTypeIdentifier": "OBJECT"
            }
          }
        }
      }
    },
    {
      "config": {
        "ContainsKey": {
          "key": "headers",
          "data_type_identifier": {
            "type": {
              "DataTypeIdentifier": "HTTP_HEADER_MAP"
            }
          }
        }
      }
    },
    {
      "config": {
        "ParentType": {
          "parent_type": {
            "type": {
              "DataTypeIdentifier": "OBJECT"
            }
          }
        }
      }
    }
  ],
  "generic_keys": []
}
```

```json
{
  "variant": 3,
  "identifier": "HTTP_RESPONSE_OBJECT",
  "name": [
    {
      "code": "en-US",
      "content": "HTTP Response"
    }
  ],
  "rules": [
    {
      "config": {
        "ContainsKey": {
          "key": "headers",
          "data_type_identifier": {
            "type": {
              "DataTypeIdentifier": "HTTP_HEADER_MAP"
            }
          }
        }
      }
    },
    {
      "config": {
        "ContainsKey": {
         "key": "body",
          "data_type_identifier": {
            "type": {
              "DataTypeIdentifier": "OBJECT"
            }
          }
        }
      }
    },
    {
      "config": {
        "ParentType": {
          "parent_type": {
            "type": {
              "DataTypeIdentifier": "OBJECT"
            }
          }
        }
      }
    }
  ],
  "generic_keys": []
}
```
