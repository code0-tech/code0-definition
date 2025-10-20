import {Reader} from './reader.js';
import {
    DataType, DataTypeIdentifier,
    DataTypeRule,
    DataTypeRuleConnection, DataTypeRulesConfig,
    DataTypeRulesVariant,
    DataTypeVariant,
    FlowType, FlowTypeSetting,
    FunctionDefinition, ParameterDefinitionConnection,
    TranslationConnection
} from "@code0-tech/sagittarius-graphql-types";
import {
    DefinitionDataType,
    DefinitionDataType_Variant,
    DefinitionDataTypeRule
} from "@code0-tech/tucana/pb/shared.data_type_pb.js";
import {Translation} from "@code0-tech/tucana/pb/shared.translation_pb.js"
import {FlowType as TucanaFlowType, FlowTypeSetting as TucanaFlowTypeSetting} from "@code0-tech/tucana/pb/shared.flow_definition_pb.js"
import {RuntimeFunctionDefinition as TucanaFunction, RuntimeParameterDefinition} from "@code0-tech/tucana/pb/shared.runtime_function_pb.js"
import {Feature, Meta, MetaType} from "./types.js";

export const Definition = (rootPath: string): Feature[] => {
    const meta = Reader(rootPath);
    if (!meta) return [];
    const features: Feature[] = [];

    for (const m of meta) {
        let feature = features.find((f) => f.name === m.name);

        if (feature) {
            appendMeta(feature, m);
        } else {
            feature = {
                name: m.name,
                dataTypes: [],
                flowTypes: [],
                runtimeFunctions: [],
            };
            appendMeta(feature, m);
            features.push(feature);
        }
    }

    return features;
}

function appendMeta(feature: Feature, meta: Meta): void {
    const definition = meta.data;
    try {
        switch (meta.type) {
            case MetaType.DataType: {
                const parsed = JSON.parse(definition) as DefinitionDataType;
                const dataType: DataType = {
                    genericKeys: parsed.genericKeys,
                    identifier: parsed.identifier,
                    name: createTranslation(parsed.name),
                    rules: createRules(parsed.rules),
                    variant: getDataTypeVariant(parsed.variant)
                }
                feature.dataTypes.push(dataType);
                break;
            }
            case MetaType.FlowType: {
                const parsed = JSON.parse(definition) as TucanaFlowType;
                const flowType: FlowType = {
                    identifier: parsed.identifier,
                    inputType: getDataType(parsed.inputTypeIdentifier),
                    returnType: getDataType(parsed.returnTypeIdentifier),
                    flowTypeSettings: createFlowTypeSetting(parsed.settings),
                    names: createTranslation(parsed.name),
                    descriptions: createTranslation(parsed.description),
                    editable: parsed.editable
                }
                feature.flowTypes.push(flowType);
                break;
            }
            case MetaType.RuntimeFunction: {
                const parsed = JSON.parse(definition) as TucanaFunction;
                const functionDefinition: FunctionDefinition = {
                    genericKeys: parsed.genericKeys,
                    names: createTranslation(parsed.name),
                    descriptions: createTranslation(parsed.description),
                    documentations: createTranslation(parsed.documentation),
                    deprecationMessages: createTranslation(parsed.deprecationMessage),
                    throwsError: parsed.throwsError,
                    returnType: getDataTypeIdentifier(parsed.returnTypeIdentifier),
                    parameterDefinitions: getParameterDefinitionConnection(parsed.runtimeParameterDefinitions),
                }

                feature.runtimeFunctions.push(functionDefinition);
                break;
            }
        }
    } catch (err: any) {
        console.error(`Error parsing ${meta.type} ${meta.name} ${definition}:`, err);
    }
}

function createFlowTypeSetting(settings: TucanaFlowTypeSetting[]): FlowTypeSetting[] {
    return settings.map(setting => {
        return {
            names: createTranslation(setting.name),
            descriptions: createTranslation(setting.description),
            dataType: getDataType(setting.dataTypeIdentifier),
            identifier: setting.identifier,
            unique: setting.unique
        }
    })
}

function getParameterDefinitionConnection(def: RuntimeParameterDefinition[]): ParameterDefinitionConnection {
    return {
        count: def.length,
        nodes: def.map(node => {
            return {
                names: createTranslation(node.name),
                descriptions: createTranslation(node.description),
                documentations: createTranslation(node.documentation),
                dataType: getDataTypeIdentifier(node.dataTypeIdentifier)
            }
        })
    }
}

function getDataType(identifier: string | undefined): DataType {
    // TODO
    // @ts-ignore
    return null
}

function getDataTypeIdentifier(identifier:  string): DataTypeIdentifier {
    // TODO
    // @ts-ignore
    return null
}

function createTranslation(translation: Translation[]): TranslationConnection {
    return {
        count: translation.length,
        nodes: translation,
    }
}

function mapDefinitionRuleToDataTypeRule(
    rule: DefinitionDataTypeRule
): DataTypeRulesConfig | null {
    const { config } = rule;

    switch (config.oneofKind) {
        case "containsKey":
            return {
                __typename: "DataTypeRulesContainsKeyConfig",
                key: config.containsKey.key,
                dataTypeIdentifier: config.containsKey.dataTypeIdentifier,
            };

        case "containsType":
            return {
                __typename: "DataTypeRulesContainsTypeConfig",
                dataTypeIdentifier: config.containsType.dataTypeIdentifier,
            };

        case "itemOfCollection":
            return {
                __typename: "DataTypeRulesItemOfCollectionConfig",
                items: config.itemOfCollection.items,
            };

        case "numberRange":
            return {
                __typename: "DataTypeRulesNumberRangeConfig",
                from: Number(config.numberRange.from),
                to: Number(config.numberRange.to),
                steps: config.numberRange.steps ? Number(config.numberRange.steps) : undefined,
            };

        case "regex":
            return {
                __typename: "DataTypeRulesRegexConfig",
                pattern: config.regex.pattern,
            };

        case "inputTypes":
            return {
                __typename: "DataTypeRulesInputTypesConfig",
                inputTypes: config.inputTypes.inputTypes.map(input => ({
                    __typename: "DataTypeRulesInputTypeConfig",
                    inputType: input.dataTypeIdentifier
                        ? { identifier: input.dataTypeIdentifier.type.dataTypeIdentifier ?? undefined }
                        : undefined,
                    dataTypeIdentifier: input.dataTypeIdentifier,
                })),
            };

        case "returnType":
            return {
                __typename: "DataTypeRulesReturnTypeConfig",
                dataTypeIdentifier: config.returnType.dataTypeIdentifier,
            };

        case "parentType":
            return {
                __typename: "DataTypeRulesParentTypeConfig",
                dataTypeIdentifier: config.parentType.parentType,
            };

        default:
            return null;
    }
}


function createRules(rule: DefinitionDataTypeRule[]): DataTypeRuleConnection {
    return {
        count: rule.length,
        nodes: rule.map(r => {
                const rule: DataTypeRule = {
                    variant: getRuleTypeValue(r.config.oneofKind),
                    config: mapDefinitionRuleToDataTypeRule(r)
                }
                return rule;
            }
        )
    }
}

function getRuleTypeValue(rule: string | unknown): DataTypeRulesVariant {
    switch (rule) {
        case "containsKey":
            return DataTypeRulesVariant.ContainsKey;
        case "containsType":
            return DataTypeRulesVariant.ContainsType;
        case "itemOfCollection":
            return DataTypeRulesVariant.ItemOfCollection;
        case "numberRange":
            return DataTypeRulesVariant.NumberRange;
        case "regex":
            return DataTypeRulesVariant.Regex;
        case "inputTypes":
            return DataTypeRulesVariant.InputType;
        case "returnType":
            return DataTypeRulesVariant.ReturnType;
        case "parentType":
            return DataTypeRulesVariant.ParentType;
        default:
            throw new Error(`Unknown rule: ${rule}`);
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