import { FlowType as TucanaFlowType } from "@code0-tech/tucana/pb/shared.flow_definition_pb.js";
import { RuntimeFunctionDefinition as TucanaFunction } from "@code0-tech/tucana/pb/shared.runtime_function_pb.js";
import { DefinitionDataType as TucanaDataType } from "@code0-tech/tucana/pb/shared.data_type_pb.js";

import type { DataType, FlowType, FunctionDefinition } from "@code0-tech/sagittarius-graphql-types";
import { getDataType } from "../mapper/dataTypeMapper.js";
import { mapFlowType } from "../mapper/flowTypeMapper.js";
import { mapFunction } from "../mapper/functionMapper.js";
import {Reader} from "./reader";

export interface ConstructedDataTypes {
    scannedTucanaTypes: TucanaDataType[];
    constructedDataTypes: DataType[];
    id: number;
}

export interface Feature {
    name: string;
    dataTypes: DataType[];
    flowTypes: FlowType[];
    runtimeFunctions: FunctionDefinition[];
}
export function getID(constructedDataTypes: ConstructedDataTypes) {
    const last = constructedDataTypes.id;
    constructedDataTypes.id += 1;
    return last;
}

export async function DefinitionMapper(path: string): Promise<Feature[]> {
    const dataTypes: { feature: string; type: TucanaDataType }[] = [];
    const flowTypes: { feature: string; flow: TucanaFlowType }[] = [];
    const runtimeFunctions: { feature: string; func: TucanaFunction }[] = [];

    const tucanaFeature = await Reader.fromPath(path)

    tucanaFeature.forEach(feature => {
        feature.data_types.forEach(dataType => {
            dataTypes.push({
                feature: feature.name,
                type: dataType
            })
        })

        feature.flow_types.forEach(flowType => {
            flowTypes.push({
                feature: feature.name,
                flow: flowType
            })
        })

        feature.runtime_functions.forEach(runtimeFunction => {
            runtimeFunctions.push({
                feature: feature.name,
                func: runtimeFunction
            })
        })
    })

    const constructed: ConstructedDataTypes = {
        scannedTucanaTypes: dataTypes.map((d) => d.type),
        constructedDataTypes: [],
        id: 0,
    };

    const features: Feature[] = [];
    const getFeature = (name: string): Feature => {
        let f = features.find((x) => x.name === name);
        if (!f) {
            f = { name, dataTypes: [], flowTypes: [], runtimeFunctions: [] };
            features.push(f);
        }
        return f;
    };

    dataTypes
        .map((f) => ({ name: f.feature, type: getDataType(f.type.identifier, constructed) }))
        .forEach((dt) => dt.type && getFeature(dt.name).dataTypes.push(dt.type));

    runtimeFunctions
        .map((f) => ({ name: f.feature, type: mapFunction(f.func, constructed) }))
        .forEach((rf) => rf.type && getFeature(rf.name).runtimeFunctions.push(rf.type));

    flowTypes
        .map((f) => ({ name: f.feature, type: mapFlowType(f.flow, constructed) }))
        .forEach((ft) => ft.type && getFeature(ft.name).flowTypes.push(ft.type));

    return features;
}
