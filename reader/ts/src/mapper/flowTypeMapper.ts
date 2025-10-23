import {FlowType as TucanaFlowType, FlowTypeSetting as TucanaFlowTypeSetting} from "@code0-tech/tucana/pb/shared.flow_definition_pb.ts"
import {FlowType, FlowTypeSetting} from "@code0-tech/sagittarius-graphql-types";
import {getDataType} from "./dataTypeMapper.ts";
import {ConstructedDataTypes, getID} from "../parser.ts";
import {getTranslationConnection} from "./translation.js";

function mapFlowType(flowType: TucanaFlowType, constructed: ConstructedDataTypes): FlowType | null {
    return  {
        id: `gid://sagittarius/TypesFlowType/${getID(constructed)}`,
        identifier: flowType.identifier,
        inputType: getDataType(flowType.inputTypeIdentifier!!, constructed),
        returnType: getDataType(flowType.returnTypeIdentifier!!, constructed),
        flowTypeSettings: createFlowTypeSetting(flowType.settings, constructed),
        names: getTranslationConnection(flowType.name),
        descriptions: getTranslationConnection(flowType.description),
        editable: flowType.editable
    }
}

function createFlowTypeSetting(settings: TucanaFlowTypeSetting[], constructed: ConstructedDataTypes): FlowTypeSetting[] {
    return settings.map(setting => {
        const flowSetting: FlowTypeSetting = {
            id: `gid://sagittarius/FlowTypeSetting/${getID(constructed)}`,
            names: getTranslationConnection(setting.name),
            descriptions: getTranslationConnection(setting.description),
            dataType: getDataType(setting.dataTypeIdentifier, constructed),
            identifier: setting.identifier,
            unique: setting.unique
        }

        return flowSetting;
    })
}

export {mapFlowType}