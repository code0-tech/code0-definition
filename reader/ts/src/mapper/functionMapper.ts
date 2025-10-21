import {Meta, MetaType} from "../types.ts";
import {FunctionDefinition, ParameterDefinitionConnection} from "@code0-tech/sagittarius-graphql-types";
import {
    RuntimeFunctionDefinition as TucanaFunction,
    RuntimeParameterDefinition
} from "@code0-tech/tucana/pb/shared.runtime_function_pb.ts";
import {getDataTypeIdentifier, getTranslationConnection} from "./helper.ts";

function mapFunction(meta:Meta): FunctionDefinition | null {
    if (meta.type != MetaType.RuntimeFunction) {
        console.error(`Expected RuntimeFunction, got ${meta.type}`);
        return null;
    }

    const parsed = JSON.parse(meta.data) as TucanaFunction;
     return {
        genericKeys: parsed.genericKeys,
        names: getTranslationConnection(parsed.name),
        descriptions: getTranslationConnection(parsed.description),
        documentations: getTranslationConnection(parsed.documentation),
        deprecationMessages: getTranslationConnection(parsed.deprecationMessage),
        throwsError: parsed.throwsError,
        returnType: getDataTypeIdentifier(parsed.returnTypeIdentifier),
        parameterDefinitions: getParameterDefinitionConnection(parsed.runtimeParameterDefinitions),
    }
}

function getParameterDefinitionConnection(def: RuntimeParameterDefinition[]): ParameterDefinitionConnection {
    return {
        count: def.length,
        nodes: def.map(node => {
            return {
                names: getTranslationConnection(node.name),
                descriptions: getTranslationConnection(node.description),
                documentations: getTranslationConnection(node.documentation),
                dataType: getDataTypeIdentifier(node.dataTypeIdentifier)
            }
        })
    }
}

export {mapFunction}