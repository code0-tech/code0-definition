## PREDICATE
```json
{
  "identifier": "PREDICATE",
  "variant": 7,
  "rules": [
    {
      "config": {
        "ReturnType": {
          "data_type_identifier": {
            "type": {
              "DataTypeIdentifier": "BOOLEAN"
            }
          }
        }
      }
    },
    {
      "config": {
          "InputTypes": {
            "input_types": [
              {
                "data_type_identifier": {
                  "type": {
                    "GenericKey": "T"
                  }
                },
                "input_identifier": "predicate"
              }
            ]
          }
      }
    }
  ],
  "generic_keys": ["T"],
  "name": [
    {
      "code": "en-US",
      "content": "Predicate"
    }
  ]
}
```

## CONSUMER
```json
{
  "identifier": "CONSUMER",
  "variant": 7,
  "rules": [
    {
      "config": {
        "InputTypes": {
          "input_types": [
            {
              "data_type_identifier": {
                "type": {
                  "GenericKey": "T"
                }
              },
              "input_identifier": "consumer"
            }
          ]
        }
      }
    }
  ],
  "generic_keys": ["T"],
  "name": [
    {
      "code": "en-US",
      "content": "Consumer"
    }
  ]

}
```

## TRANSFORM
```json
{
  "identifier": "TRANSFORM",
  "variant": 7,
  "rules": [
    {
      "config": {
          "ReturnType": {
            "data_type_identifier": {
              "type": {
                "GenericKey": "R"
              }
            }
          }
      }
    },
    {
      "config": {
          "InputTypes": {
            "input_types": [
              {
                "data_type_identifier": {
                  "type": {
                    "GenericKey": "I"
                  }
                },
                "input_identifier": "transform"
              }
            ]
          }
      }
    }
  ],
  "generic_keys": ["I", "R"],
  "name": [
    {
      "code": "en-US",
      "content": "Transform"
    }
  ]
}
```

## COMPARITOR
```json
{
  "identifier": "COMPARATOR",
  "variant": 7,
  "rules": [
    {
      "config": {
        "ReturnType": {
          "data_type_identifier": {
            "type": {
              "DataTypeIdentifier": "NUMBER"
            }
          }
        }
      }
    },
    {
      "config": {
       "InputTypes": {
         "input_types": [
           {
             "data_type_identifier": {
               "type": {
                 "GenericKey": "I"
               }
             },
             "input_identifier": "left"
           },
           {
             "data_type_identifier": {
               "type": {
                 "GenericKey": "I"
               }
             },
             "input_identifier": "right"
           }
         ]
       }
      }
    }
  ],
  "generic_keys": ["I"],
  "name": [
    {
      "code": "en-US",
      "content": "Comparator"
    }
  ]
}
```
