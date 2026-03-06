import {
	FunctionDefinition,
	ParameterDefinitionConnection
} from "@code0-tech/sagittarius-graphql-types";
import {
	RuntimeFunctionDefinition as TucanaFunction,
	RuntimeParameterDefinition
} from "@code0-tech/tucana/pb/shared.runtime_function_pb.js";
import { ConstructedDataTypes, getID } from "../definition/mapper.js";
import { getTranslationConnection } from "./translation.js";

export function mapFunction(func: TucanaFunction, constructed: ConstructedDataTypes): FunctionDefinition | null {
	return {
		__typename: "FunctionDefinition",
		id: `gid://sagittarius/FunctionDefinition/${getID(constructed)}`,
		names: getTranslationConnection(func.name),
		descriptions: getTranslationConnection(func.description),
		documentations: getTranslationConnection(func.documentation),
		deprecationMessages: getTranslationConnection(func.deprecationMessage),
		identifier: func.runtimeName,
		displayMessages: getTranslationConnection(func.displayMessage),
		aliases: getTranslationConnection(func.alias),
		throwsError: func.throwsError,
		// @ts-ignore
		linkedDataTypeIdentifiers: func.linkedDataTypeIdentifiers,
		signature: func.signature,
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
				__typename: "ParameterDefinition",
				id: `gid://sagittarius/ParameterDefinition/${getID(constructed)}`,
				names: getTranslationConnection(node.name),
				identifier: node.runtimeName,
				descriptions: getTranslationConnection(node.description),
				documentations: getTranslationConnection(node.documentation)
			}
		})
	}
}

