## containsKey
```json
{
  "runtime_name": "std::object::contains_key",
  "runtime_parameter_definitions": [
    {
      "data_type_identifier": {
        "data_type_identifier": "OBJECT"
      },
      "runtime_name": "object",
      "name": [
        {
          "code": "en-US",
          "content": "Object"
        }
      ],
      "description": [
        {
          "code": "en-US",
          "content": "The object to check for the presence of a key."
        }
      ],
      "documentation": [
        {
          "code": "en-US",
          "content": "The object within which the existence of the specified key will be checked."
        }
      ]
    },
    {
      "data_type_identifier": {
        "data_type_identifier": "TEXT"
      },
      "runtime_name": "key",
      "name": [
        {
          "code": "en-US",
          "content": "Key"
        }
      ],
      "description": [
        {
          "code": "en-US",
          "content": "The key to check for existence in the object."
        }
      ],
      "documentation": [
        {
          "code": "en-US",
          "content": "The property key whose presence in the object is being tested."
        }
      ]
    }
  ],
  "return_type_identifier": {
    "data_type_identifier": "BOOLEAN"
  },
  "name": [
    {
      "code": "en-US",
      "content": "Contains Key"
    }
  ],
  "description": [
    {
      "code": "en-US",
      "content": "Checks whether the specified key exists in the object."
    }
  ],
  "documentation": [
    {
      "code": "en-US",
      "content": "Returns true if the given key is a property of the object; otherwise, returns false."
    }
  ],
  "deprecation_message": [],
  "generic_keys": [],
  "generic_mappers": [],
  "throws_error": false
}
```


## keys
```json
{
  "runtime_name": "std::object::keys",
  "runtime_parameter_definitions": [
    {
      "data_type_identifier": {
        "data_type_identifier": "OBJECT"
      },
      "runtime_name": "object",
      "name": [
        {
          "code": "en-US",
          "content": "Object"
        }
      ],
      "description": [
        {
          "code": "en-US",
          "content": "The object whose keys will be retrieved."
        }
      ],
      "documentation": [
        {
          "code": "en-US",
          "content": "Returns an array of all the keys (property names) of the given object."
        }
      ]
    }
  ],
  "return_type_identifier": {
    "data_type_identifier": "ARRAY"
  },
  "generic_keys": ["T"],
  "generic_mappers": [
    {
      "source": [
        {
          "data_type_identifier": "TEXT"
        }
      ],
      "target": "T",
      "generic_combinations": []
    }
  ],
  "name": [
    {
      "code": "en-US",
      "content": "Get Object Keys"
    }
  ],
  "description": [
    {
      "code": "en-US",
      "content": "Retrieves all the keys from the given object as an array of text values."
    }
  ],
  "documentation": [
    {
      "code": "en-US",
      "content": "Returns an array containing all enumerable property names (keys) of the specified object."
    }
  ],
  "deprecation_message": [],
  "throws_error": false
}
```

## size
```json
{
  "runtime_name": "std::object::size",
  "runtime_parameter_definitions": [
    {
      "data_type_identifier": {
        "data_type_identifier": "OBJECT"
      },
      "runtime_name": "object",
      "name": [
        {
          "code": "en-US",
          "content": "Object"
        }
      ],
      "description": [
        {
          "code": "en-US",
          "content": "The object whose size (number of keys) will be calculated."
        }
      ],
      "documentation": [
        {
          "code": "en-US",
          "content": "Returns the number of enumerable keys (properties) present in the given object."
        }
      ]
    }
  ],
  "return_type_identifier": {
    "data_type_identifier": "NUMBER"
  },
  "name": [
    {
      "code": "en-US",
      "content": "Get Object Size"
    }
  ],
  "description": [
    {
      "code": "en-US",
      "content": "Calculates the number of keys in the provided object."
    }
  ],
  "documentation": [
    {
      "code": "en-US",
      "content": "Returns an integer count of all enumerable property keys in the specified object."
    }
  ],
  "deprecation_message": [],
  "generic_keys": [],
  "generic_mappers": [],
  "throws_error": false
}
```

## set
```json
{
  "runtime_name": "std::object::set",
  "runtime_parameter_definitions": [
    {
      "data_type_identifier": {
        "data_type_identifier": "OBJECT"
      },
      "runtime_name": "object",
      "name": [
        {
          "code": "en-US",
          "content": "Object"
        }
      ],
      "description": [
        {
          "code": "en-US",
          "content": "The object in which the key-value pair will be set."
        }
      ],
      "documentation": [
        {
          "code": "en-US",
          "content": "The original object that will be modified with the specified key-value pair."
        }
      ]
    },
    {
      "data_type_identifier": {
        "data_type_identifier": "TEXT"
      },
      "runtime_name": "key",
      "name": [
        {
          "code": "en-US",
          "content": "Key"
        }
      ],
      "description": [
        {
          "code": "en-US",
          "content": "The key to set or update in the object."
        }
      ],
      "documentation": [
        {
          "code": "en-US",
          "content": "The property name under which the value will be stored in the object."
        }
      ]
    },
    {
      "data_type_identifier": {
        "generic_key": "I"
      },
      "runtime_name": "value",
      "name": [
        {
          "code": "en-US",
          "content": "Value"
        }
      ],
      "description": [
        {
          "code": "en-US",
          "content": "The value to set for the specified key."
        }
      ],
      "documentation": [
        {
          "code": "en-US",
          "content": "The value to assign to the object property identified by the key."
        }
      ]
    }
  ],
  "return_type_identifier": {
    "data_type_identifier": "OBJECT"
  },
  "name": [
    {
      "code": "en-US",
      "content": "Set Object Key"
    }
  ],
  "description": [
    {
      "code": "en-US",
      "content": "Sets or updates a key-value pair in the given object."
    }
  ],
  "documentation": [
    {
      "code": "en-US",
      "content": "Returns a new object with the specified key set to the given value."
    }
  ],
  "generic_keys": ["I"],
  "generic_mappers": [
    {
      "parameter_id": "value",
      "source": [
        {
          "generic_key": "I"
        }
      ],
      "target": "I",
      "generic_combinations": []
    }
  ],
  "deprecation_message": [],
  "throws_error": false
}
```
