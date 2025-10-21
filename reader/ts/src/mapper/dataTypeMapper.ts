import {
    DefinitionDataType,
    DefinitionDataType_Variant,
    DefinitionDataTypeRule
} from "@code0-tech/tucana/pb/shared.data_type_pb.ts";
import {getTranslationConnection} from "./helper.ts";
import {
    DataType,
    DataTypeRule,
    DataTypeRuleConnection, DataTypeRulesContainsKeyConfig,
    DataTypeRulesContainsTypeConfig, DataTypeRulesInputTypesConfig, DataTypeRulesItemOfCollectionConfig,
    DataTypeRulesNumberRangeConfig, DataTypeRulesParentTypeConfig, DataTypeRulesRegexConfig, DataTypeRulesVariant,
    DataTypeVariant
} from "@code0-tech/sagittarius-graphql-types";

function mapDataType(dataType: DefinitionDataType | undefined): DataType | null {
    if (dataType == undefined) {
        return null
    }

    return {
        genericKeys: dataType.genericKeys,
        identifier: dataType.identifier,
        name: getTranslationConnection(dataType.name),
        rules: createRules(dataType.rules),
        variant: getDataTypeVariant(dataType.variant)
    }
}

function createRules(rule: DefinitionDataTypeRule[]): DataTypeRuleConnection {
    return {
        count: rule.length,
        nodes: rule.map(r => {
            console.log(r)
            const config: any = r.config
            if (config.ContainsType) {
                const ruleConfig: DataTypeRulesContainsTypeConfig = {
                    dataTypeIdentifier: null, //TODO
                }
                const rule: DataTypeRule = {
                    variant: DataTypeRulesVariant.ContainsType,
                    config: ruleConfig
                }
                return rule;
            }

            if (config.ContainsKey) {
                const ruleConfig: DataTypeRulesContainsKeyConfig = {
                    dataTypeIdentifier: null, //TODO
                    key: null,
                }
                const rule: DataTypeRule = {
                    variant: DataTypeRulesVariant.ContainsKey,
                    config: ruleConfig
                }
                return rule;
            }

            if (config.ItemOfCollection) {
                const ruleConfig: DataTypeRulesItemOfCollectionConfig = {
                    items: null, //TODO
                }
                const rule: DataTypeRule = {
                    variant: DataTypeRulesVariant.ItemOfCollection,
                    config: ruleConfig
                }
                return rule;
            }

            if (config.NumberRange) {
                const ruleConfig: DataTypeRulesNumberRangeConfig = {
                    from: null, //TODO
                    steps: null, //TODO
                    to: null, //TODO
                }
                const rule : DataTypeRule = {
                    variant: DataTypeRulesVariant.NumberRange,
                    config: ruleConfig
                }
                return rule;
            }

            if (config.Regex) {
                const ruleConfig: DataTypeRulesRegexConfig = {
                    pattern: null, //TODO
                }
                const rule : DataTypeRule = {
                    variant: DataTypeRulesVariant.Regex,
                    config: ruleConfig
                }
                return rule;
            }

            if (config.InputTypes) {
                const ruleConfig: DataTypeRulesInputTypesConfig = {
                    inputTypes: null, //TODO
                }
                const rule : DataTypeRule = {
                    variant: DataTypeRulesVariant.InputType,
                    config: ruleConfig
                }
                return rule;
            }

            if (config.ReturnType) {
                const ruleConfig: DataTypeRulesParentTypeConfig = {
                    dataTypeIdentifier: null, //TODO
                }
                const rule : DataTypeRule = {
                    variant: DataTypeRulesVariant.ReturnType,
                    config: ruleConfig
                }
                return rule;
            }

            if (config.ParentType) {
                const ruleConfig: DataTypeRulesParentTypeConfig = {
                    dataTypeIdentifier: null, //TODO
                }
                const rule : DataTypeRule = {
                    variant: DataTypeRulesVariant.ParentType,
                    config: ruleConfig
                }
                return rule;
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

export {mapDataType}