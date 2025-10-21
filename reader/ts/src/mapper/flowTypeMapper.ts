import {FlowType as TucanaFlowType, FlowTypeSetting as TucanaFlowTypeSetting} from "@code0-tech/tucana/pb/shared.flow_definition_pb.ts"
import {FlowType, FlowTypeSetting} from "@code0-tech/sagittarius-graphql-types";
import {getDataType, getTranslationConnection} from "./helper.ts";
import {Meta, MetaType} from "../types.ts";

function mapFlowType(meta: Meta): FlowType | null {
    if (meta.type != MetaType.FlowType) {
        console.error(`Expected FlowType, got ${meta.type}`);
        return null;
    }

    const parsed = JSON.parse(meta.data) as TucanaFlowType;
    return  {
        identifier: parsed.identifier,
        inputType: getDataType(parsed.inputTypeIdentifier),
        returnType: getDataType(parsed.returnTypeIdentifier),
        flowTypeSettings: createFlowTypeSetting(parsed.settings),
        names: getTranslationConnection(parsed.name),
        descriptions: getTranslationConnection(parsed.description),
        editable: parsed.editable
    }
}

function createFlowTypeSetting(settings: TucanaFlowTypeSetting[]): FlowTypeSetting[] {
    return settings.map(setting => {
        return {
            names: getTranslationConnection(setting.name),
            descriptions: getTranslationConnection(setting.description),
            dataType: getDataType(setting.dataTypeIdentifier),
            identifier: setting.identifier,
            unique: setting.unique
        }
    })
}

export {mapFlowType}