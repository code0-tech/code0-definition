import {FunctionDefinition, ParameterDefinitionConnection} from "@code0-tech/sagittarius-graphql-types";
import {
    RuntimeFunctionDefinition as TucanaFunction,
    RuntimeParameterDefinition
} from "@code0-tech/tucana/pb/shared.runtime_function_pb.ts";
import {getDataTypeIdentifier} from "./dataTypeMapper.ts";
import {ConstructedDataTypes} from "../parser.ts";
import {getTranslationConnection} from "./translation.js";

function mapFunction(func: TucanaFunction, constructed: ConstructedDataTypes): FunctionDefinition | null {
     return {
        genericKeys: func.genericKeys,
        names: getTranslationConnection(func.name),
        descriptions: getTranslationConnection(func.description),
        documentations: getTranslationConnection(func.documentation),
        deprecationMessages: getTranslationConnection(func.deprecationMessage),
        throwsError: func.throwsError,
        returnType: getDataTypeIdentifier(func.returnTypeIdentifier, constructed),
        parameterDefinitions: getParameterDefinitionConnection(func.runtimeParameterDefinitions, constructed),
    }
}

function getParameterDefinitionConnection(def: RuntimeParameterDefinition[], constructed: ConstructedDataTypes): ParameterDefinitionConnection {
    return {
        count: def.length,
        nodes: def.map(node => {
            return {
                names: getTranslationConnection(node.name),
                descriptions: getTranslationConnection(node.description),
                documentations: getTranslationConnection(node.documentation),
                dataType: getDataTypeIdentifier(node.dataTypeIdentifier, constructed)
            }
        })
    }
}

export {mapFunction}