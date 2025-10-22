import {
    DataType,
    DataTypeIdentifier, DataTypeRule,
    DataTypeRuleConnection, DataTypeRulesContainsKeyConfig, DataTypeRulesContainsTypeConfig,
    DataTypeRulesInputTypeConfig,
    DataTypeRulesInputTypesConfig,
    DataTypeRulesItemOfCollectionConfig, DataTypeRulesNumberRangeConfig,
    DataTypeRulesParentTypeConfig, DataTypeRulesRegexConfig, GenericCombinationStrategy,
    TranslationConnection
} from "@code0-tech/sagittarius-graphql-types";
import {Translation} from "@code0-tech/tucana/pb/shared.translation_pb.ts";
import {
    DataTypeIdentifier as TucanaDataTypeIdentifier,
    DefinitionDataType_Variant, DefinitionDataTypeRule
} from "@code0-tech/tucana/pb/shared.data_type_pb.ts"
import {GenericMapper as TucanaGenericMapper} from "@code0-tech/tucana/pb/shared.data_type_pb.ts"
import {ConstructedDataTypes} from "../parser.js";
import {getTranslationConnection} from "./translation.js";

export enum GenericMapper_GenericCombinationStrategy {
    UNKNOWN = 0,
    AND = 1,
    OR = 2
}

export enum GenericCombinationStrategyType {
    And = 'AND',
    Or = 'OR'
}

enum DataTypeVariant {
    Array = 'ARRAY',
    DataType = 'DATA_TYPE',
    Error = 'ERROR',
    Node = 'NODE',
    Object = 'OBJECT',
    Primitive = 'PRIMITIVE',
    Type = 'TYPE'
}

enum DataTypeRulesVariant {
    ContainsKey = 'CONTAINS_KEY',
    ContainsType = 'CONTAINS_TYPE',
    InputType = 'INPUT_TYPE',
    ItemOfCollection = 'ITEM_OF_COLLECTION',
    NumberRange = 'NUMBER_RANGE',
    ParentType = 'PARENT_TYPE',
    Regex = 'REGEX',
    ReturnType = 'RETURN_TYPE'
}

function getDataType(identifier: string, constructedDataTypes: ConstructedDataTypes): DataType | null {
    const dataType = constructedDataTypes.constructedDataTypes.find(dt => dt.identifier === identifier)
    if (dataType == undefined) {
        const tucanaDataType = constructedDataTypes.scannedTucanaTypes.find(dt => dt.identifier === identifier)
        if (tucanaDataType == undefined) {
            console.error("Skipping Identifier because it can't be identified:" + identifier)
            return null
        }
        const constructed = {
            genericKeys: tucanaDataType.genericKeys,
            identifier: tucanaDataType.identifier,
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
                            dataTypeIdentifier: getDataTypeIdentifier(r.config.containsType.dataTypeIdentifier, constructedDataTypes), //TODO
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
                            items: r.config.itemOfCollection.items, //TODO: This needs to be checked
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
                                    inputType: null, //TODO (Later): This field is wrong in GraphQL => should be a string
                                }
                                return input;
                            }),
                        }
                        const rule : DataTypeRule = {
                            variant: DataTypeRulesVariant.InputType,
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
                genericType: {
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
                                    type: type
                                }
                            }),
                            sources: mapper.source.map(id =>
                                getDataTypeIdentifier(id, constructedDataTypes)
                            ).filter(id => id != null),
                            target: mapper.target,
                        }
                    }),
                }
            }
        }

        case "dataTypeIdentifier": {
            return {
                dataType: getDataType(identifier.type.dataTypeIdentifier, constructedDataTypes)
            }
        }

        case "genericKey": {
            return {
                genericKey: identifier.type.genericKey,
            }
        }
    }

    return null;
}

export {getDataType, getDataTypeIdentifier}