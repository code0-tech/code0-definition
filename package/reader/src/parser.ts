import {readdirSync, readFileSync} from "node:fs";
import {FlowType} from "@code0-tech/tucana/pb/shared.flow_definition_pb.js";
import {RuntimeFunctionDefinition} from "@code0-tech/tucana/pb/shared.runtime_function_pb.js";
import {DefinitionDataType} from "@code0-tech/tucana/pb/shared.data_type_pb.js";
import { join } from "node:path";

export interface Feature {
    dataTypes: DefinitionDataType[];
    flowTypes: FlowType[];
    runtimeFunctions: RuntimeFunctionDefinition[];
}

export const Definition = (path: string): Feature => {

    readdirSync(path, { withFileTypes: true }).forEach(file => {
        const featureName = file.name.split("_")[0]
        if (featureName == null) {
            throw new Error("Feature name is null")
        }

        const filePath = join(file.parentPath, file.name)
        const content = readFileSync(filePath);

        if (file.name.includes("data_type")) {
            const decoded = TucanaDataType.fromBinary(content);
            dataTypes.push(
                {
                    feature: featureName,
                    type: decoded,
                }
            )
        }

        if (file.name.includes("function")) {
            const decoded = TucanaFunction.fromBinary(content);
            runtimeFunctions.push(
                {
                    feature: featureName,
                    func: decoded,
                }
            )
        }

        if (file.name.includes("flow_type")) {
            const decoded = TucanaFlowType.fromBinary(content);
            flowTypes.push(
                {
                    feature: featureName,
                    flow: decoded,
                }
            )
        }
    })

    const features: Feature[] = []
    const constructed: ConstructedDataTypes = {
        scannedTucanaTypes: dataTypes.map(f => f.type),
        constructedDataTypes: [],
        id: 0
    }

    function getFeature(name:string): Feature {
        const feature = features.find((f) => f.name === name);
        if (feature != undefined) {
            return feature;
        }

        const newFeature = {
            name: name,
            dataTypes: [],
            flowTypes: [],
            runtimeFunctions: [],
        };

        features.push(newFeature);
        return newFeature;
    }

    dataTypes.map(f => {
        return {
            name: f.feature,
            type: getDataType(f.type.identifier, constructed)
        }
    }).forEach(dt => {
        if (dt.type != null) {
            const feature = getFeature(dt.name)
            feature.dataTypes.push(dt.type)
        }
    })

    runtimeFunctions.map(f => {
        return {
            name: f.feature,
            type: mapFunction(f.func, constructed)
        }
    }).forEach(dt => {
        if (dt.type != null) {
            const feature = getFeature(dt.name)
            feature.runtimeFunctions.push(dt.type)
        }
    })

    flowTypes.map(f => {
        return {
            name: f.feature,
            type: mapFlowType(f.flow, constructed)
        }
    }).forEach(dt => {
        if (dt.type != null) {
            const feature = getFeature(dt.name)
            feature.flowTypes.push(dt.type)
        }
    })

    return features;
}
