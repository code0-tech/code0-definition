import {FunctionDefinition, ParameterDefinitionConnection} from "@code0-tech/sagittarius-graphql-types";
import {
    RuntimeFunctionDefinition as TucanaFunction,
    RuntimeParameterDefinition
} from "@code0-tech/tucana/pb/shared.runtime_function_pb.js";
import {getDataTypeIdentifier} from "./dataTypeMapper.js";
import {ConstructedDataTypes, getID} from "../parser.js";
import {getTranslationConnection} from "./translation.js";

function mapFunction(func: TucanaFunction, constructed: ConstructedDataTypes): FunctionDefinition | null {
     return {
        id: `gid://sagittarius/FunctionDefinition/${getID(constructed)}`,
        genericKeys: func.genericKeys,
        names: getTranslationConnection(func.name),
        descriptions: getTranslationConnection(func.description),
        documentations: getTranslationConnection(func.documentation),
        deprecationMessages: getTranslationConnection(func.deprecationMessage),
        throwsError: func.throwsError,
        returnType: getDataTypeIdentifier(func.returnTypeIdentifier, constructed),
        parameterDefinitions: getParameterDefinitionConnection(func.runtimeParameterDefinitions, constructed),
         runtimeFunctionDefinition: {
             id: `gid://sagittarius/RuntimeFunctionDefinition/${getID(constructed)}`,
             identifier: func.runtimeName
         }
    }
}

function getParameterDefinitionConnection(def: RuntimeParameterDefinition[], constructed: ConstructedDataTypes): ParameterDefinitionConnection {
    return {
        count: def.length,
        nodes: def.map(node => {
            return {
                id: `gid://sagittarius/ParameterDefinition/${getID(constructed)}`,
                names: getTranslationConnection(node.name),
                identifier: node.runtimeName,
                descriptions: getTranslationConnection(node.description),
                documentations: getTranslationConnection(node.documentation),
                dataTypeIdentifier: getDataTypeIdentifier(node.dataTypeIdentifier, constructed)
            }
        })
    }
}

export {mapFunction}