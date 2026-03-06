import { FlowType as TucanaFlowType, FlowTypeSetting as TucanaFlowTypeSetting } from "@code0-tech/tucana/pb/shared.flow_definition_pb.js"
import { FlowType, FlowTypeSetting } from "@code0-tech/sagittarius-graphql-types";
import { getDataType } from "./dataTypeMapper.js";
import { getTranslationConnection } from "./translation.js";
import { ConstructedDataTypes, getID } from "../definition/mapper.js";

function mapFlowType(flowType: TucanaFlowType, constructed: ConstructedDataTypes): FlowType | null {
	return {
		__typename: "FlowType",
		id: `gid://sagittarius/FlowType/${getID(constructed)}`,
		identifier: flowType.identifier,
		// @ts-ignore
		inputType: flowType.inputTypeIdentifier,
		// @ts-ignore
		returnType: flowType.returnTypeIdentifier,
		flowTypeSettings: createFlowTypeSetting(flowType.settings, constructed),
		names: getTranslationConnection(flowType.name),
		descriptions: getTranslationConnection(flowType.description),
		aliases: getTranslationConnection(flowType.alias),
		displayMessages: getTranslationConnection(flowType.displayMessage),
		editable: flowType.editable
	}
}

function createFlowTypeSetting(settings: TucanaFlowTypeSetting[], constructed: ConstructedDataTypes): FlowTypeSetting[] {
	return settings.map(setting => {
		const flowSetting: FlowTypeSetting = {
			__typename: "FlowTypeSetting",
			id: `gid://sagittarius/FlowTypeSetting/${getID(constructed)}`,
			names: getTranslationConnection(setting.name),
			descriptions: getTranslationConnection(setting.description),
			dataType: getDataType(setting.dataTypeIdentifier, constructed),
			identifier: setting.identifier,
			// @ts-ignore
			unique: setting.unique,
		}

		return flowSetting;
	})
}

export { mapFlowType }
