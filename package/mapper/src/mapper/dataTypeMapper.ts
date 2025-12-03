import {
    DataType,
    DataTypeIdentifier, DataTypeRule,
    DataTypeRuleConnection, DataTypeRulesContainsKeyConfig, DataTypeRulesContainsTypeConfig,
    DataTypeRulesInputTypeConfig,
    DataTypeRulesInputTypesConfig,
    DataTypeRulesItemOfCollectionConfig, DataTypeRulesNumberRangeConfig,
    DataTypeRulesParentTypeConfig, DataTypeRulesRegexConfig,
} from "@code0-tech/sagittarius-graphql-types";
import {
    DataTypeIdentifier as TucanaDataTypeIdentifier,
    DefinitionDataType_Variant, DefinitionDataTypeRule, GenericMapper_GenericCombinationStrategy
} from "@code0-tech/tucana/pb/shared.data_type_pb.js"
import {GenericMapper as TucanaGenericMapper} from "@code0-tech/tucana/pb/shared.data_type_pb.js"
import {ConstructedDataTypes, getID} from "../definition/mapper.js";
import {getTranslationConnection} from "./translation.js";
import {Value} from "@code0-tech/tucana/pb/shared.struct_pb.js";

enum GenericCombinationStrategyType {
    /** Represents a logical AND combination. */
    And = 'AND',
    /** Represents a logical OR combination. */
    Or = 'OR'
}

enum DataTypeRulesVariant {
    /** The rule checks if a key is present in the data type. */
    ContainsKey = 'CONTAINS_KEY',
    /** The rule checks if a specific type is present in the data type. */
    ContainsType = 'CONTAINS_TYPE',
    /** The rule checks if the data type matches a specific input type. */
    InputTypes = 'INPUT_TYPES',
    /** The rule checks if an item is part of a collection in the data type. */
    ItemOfCollection = 'ITEM_OF_COLLECTION',
    /** The rule checks if a number falls within a specified range. */
    NumberRange = 'NUMBER_RANGE',
    /** The rule checks if the data type is a child of a specific parent type. */
    ParentType = 'PARENT_TYPE',
    /** The rule checks if a string matches a specified regular expression. */
    Regex = 'REGEX',
    /** The rule checks if the data type matches a specific return type. */
    ReturnType = 'RETURN_TYPE'
}

enum DataTypeVariant {
    /** Represents an array */
    Array = 'ARRAY',
    /** Represents an data type containing a data type */
    DataType = 'DATA_TYPE',
    /** Represents a error */
    Error = 'ERROR',
    /** Represents a node */
    Node = 'NODE',
    /** Represents an object */
    Object = 'OBJECT',
    /** Represents a primitive datatype */
    Primitive = 'PRIMITIVE',
    /** Represents a type */
    Type = 'TYPE'
}

function getDataType(identifier: string, constructedDataTypes: ConstructedDataTypes): DataType | null {
    const dataType = constructedDataTypes.constructedDataTypes.find(dt => dt.identifier === identifier)
    if (dataType == undefined) {
        const tucanaDataType = constructedDataTypes.scannedTucanaTypes.find(dt => dt.identifier === identifier)
        if (tucanaDataType == undefined) {
            console.error("Skipping Identifier because it can't be identified:" + identifier)
            return null
        }
        const constructed: DataType = {
            __typename: "DataType",
            id: `gid://sagittarius/DataType/${getID(constructedDataTypes)}`,
            genericKeys: tucanaDataType.genericKeys,
            identifier: tucanaDataType.identifier,
            aliases: getTranslationConnection(tucanaDataType.alias),
            displayMessages: getTranslationConnection(tucanaDataType.displayMessage),
            name: getTranslationConnection(tucanaDataType.name),
            rules: createRules(tucanaDataType.rules, constructedDataTypes),
            variant: getDataTypeVariant(tucanaDataType.variant),
        }

        constructedDataTypes.constructedDataTypes.push(constructed)
        return constructed;
    }
    return dataType;
}

function createRules(rule: DefinitionDataTypeRule[], constructedDataTypes: ConstructedDataTypes) : DataTypeRuleConnection {
    return {
        count: rule.length,
        nodes: rule.map(r => {
                switch (r.config.oneofKind) {
                    case "containsType": {
                        const ruleConfig: DataTypeRulesContainsTypeConfig = {
                            dataTypeIdentifier: getDataTypeIdentifier(r.config.containsType.dataTypeIdentifier, constructedDataTypes),
                        }
                        const rule: DataTypeRule = {
                            variant: DataTypeRulesVariant.ContainsType,
                            config: ruleConfig
                        }
                        return rule;
                    }
                    case "containsKey": {
                        const ruleConfig: DataTypeRulesContainsKeyConfig = {
                            dataTypeIdentifier: getDataTypeIdentifier(r.config.containsKey.dataTypeIdentifier, constructedDataTypes),
                            key: r.config.containsKey.key,
                        }
                        const rule: DataTypeRule = {
                            variant: DataTypeRulesVariant.ContainsKey,
                            config: ruleConfig
                        }
                        return rule;
                    }
                    case "itemOfCollection": {
                        const ruleConfig: DataTypeRulesItemOfCollectionConfig = {
                            items: r.config.itemOfCollection.items.map(valueToJSON),
                        }
                        const rule: DataTypeRule = {
                            variant: DataTypeRulesVariant.ItemOfCollection,
                            config: ruleConfig
                        }
                        return rule;
                    }
                    case "numberRange": {
                        const ruleConfig: DataTypeRulesNumberRangeConfig = {
                            from: Number(r.config.numberRange.from),
                            steps: r.config.numberRange.steps ? Number(r.config.numberRange.steps) : undefined,
                            to: Number(r.config.numberRange.to),
                        }
                        const rule : DataTypeRule = {
                            variant: DataTypeRulesVariant.NumberRange,
                            config: ruleConfig
                        }
                        return rule;
                    }
                    case "regex": {
                        const ruleConfig: DataTypeRulesRegexConfig = {
                            pattern: r.config.regex.pattern,
                        }
                        const rule : DataTypeRule = {
                            variant: DataTypeRulesVariant.Regex,
                            config: ruleConfig
                        }
                        return rule;
                    }
                    case "inputTypes": {
                        const ruleConfig: DataTypeRulesInputTypesConfig = {
                            inputTypes: r.config.inputTypes.inputTypes.map(i => {
                                console.log("AF: " + i.inputIdentifier)
                                const input: DataTypeRulesInputTypeConfig = {
                                    dataTypeIdentifier: getDataTypeIdentifier(i.dataTypeIdentifier, constructedDataTypes),
                                    inputIdentifier: i.inputIdentifier,
                                }
                                return input;
                            }),
                        }
                        const rule : DataTypeRule = {
                            variant: DataTypeRulesVariant.InputTypes,
                            config: ruleConfig
                        }
                        return rule;
                    }

                    case "returnType": {
                        const ruleConfig: DataTypeRulesParentTypeConfig = {
                            dataTypeIdentifier: getDataTypeIdentifier(r.config.returnType.dataTypeIdentifier, constructedDataTypes),
                        }
                        const rule : DataTypeRule = {
                            variant: DataTypeRulesVariant.ReturnType,
                            config: ruleConfig
                        }
                        return rule;
                    }

                    case "parentType": {
                        const ruleConfig: DataTypeRulesParentTypeConfig = {
                            dataTypeIdentifier: getDataTypeIdentifier(r.config.parentType.parentType, constructedDataTypes),
                        }
                        const rule : DataTypeRule = {
                            variant: DataTypeRulesVariant.ParentType,
                            config: ruleConfig
                        }
                        return rule;
                    }
                }
                throw new Error(`Unknown rule: ${rule}`)
            }
        )
    }
}

function getDataTypeVariant(variant: DefinitionDataType_Variant): DataTypeVariant {
    switch (variant) {
        case DefinitionDataType_Variant.ARRAY:
            return DataTypeVariant.Array
        case DefinitionDataType_Variant.DATATYPE:
            return DataTypeVariant.DataType;
        case DefinitionDataType_Variant.ERROR:
            return DataTypeVariant.Error;
        case DefinitionDataType_Variant.NODE:
            return DataTypeVariant.Node;
        case DefinitionDataType_Variant.OBJECT:
            return DataTypeVariant.Object;
        case DefinitionDataType_Variant.PRIMITIVE:
            return DataTypeVariant.Primitive;
        case DefinitionDataType_Variant.TYPE:
            return DataTypeVariant.Type;
        default:
            throw new Error(`Unknown variant: ${variant}`);
    }
}

function getDataTypeIdentifier(identifier: TucanaDataTypeIdentifier | undefined, constructedDataTypes: ConstructedDataTypes): DataTypeIdentifier | null {
    if (identifier == undefined) {
        return null
    }

    switch (identifier.type.oneofKind) {
        case "genericType": {
            return {
                id: `gid://sagittarius/DataTypeIdentifier/${getID(constructedDataTypes)}`,
                genericType: {
                    id: `gid://sagittarius/GenericType/${getID(constructedDataTypes)}`,
                    dataType: getDataType(identifier.type.genericType.dataTypeIdentifier, constructedDataTypes),
                    genericMappers: identifier.type.genericType.genericMappers.map((mapper: TucanaGenericMapper) => {
                        return {
                            genericCombinationStrategies: mapper.genericCombinations.map(m => {
                                let type = undefined
                                switch (m) {
                                    case GenericMapper_GenericCombinationStrategy.AND:
                                        type = GenericCombinationStrategyType.And
                                        break
                                    case GenericMapper_GenericCombinationStrategy.OR:
                                        type = GenericCombinationStrategyType.Or
                                        break
                                    default:
                                        throw new Error("GenericCombinationStrategy was Unknown");
                                }
                                return {
                                    id: `gid://sagittarius/GenericCombinationStrategy/${getID(constructedDataTypes)}`,
                                    type: type
                                }
                            }),
                            sourceDataTypeIdentifiers: mapper.source.map(id =>
                                getDataTypeIdentifier(id, constructedDataTypes)
                            ).filter(id => id != null),
                            target: mapper.target,
                            id: `gid://sagittarius/GenericMapper/${getID(constructedDataTypes)}`,
                        }
                    }),
                }
            }
        }

        case "dataTypeIdentifier": {
            return {
                id: `gid://sagittarius/DataTypeIdentifier/${getID(constructedDataTypes)}`,
                dataType: getDataType(identifier.type.dataTypeIdentifier, constructedDataTypes)
            }
        }

        case "genericKey": {
            return {
                id: `gid://sagittarius/DataTypeIdentifier/${getID(constructedDataTypes)}`,
                genericKey: identifier.type.genericKey,
            }
        }
    }

    return null;
}

// @ts-ignore
function valueToJSON(value: Value) {
    switch (value.kind.oneofKind) {
        case "nullValue":
            return null
        case "numberValue":
            return value.kind.numberValue
        case "stringValue":
            return value.kind.stringValue
        case "boolValue":
            return value.kind.boolValue
        case "structValue":
            return Object.fromEntries(
                // @ts-ignore
                Object.entries(value.kind.structValue.fields).map(([k, v]) => {
                    return  [k, valueToJSON(v)]
                })
            )
        case "listValue":
            return value.kind.listValue.values.map(valueToJSON)
    }
}

export {getDataType, getDataTypeIdentifier}