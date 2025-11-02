import { unzipSync } from "fflate";
import {
    DefinitionDataType as TucanaDataType
} from "@code0-tech/tucana/pb/shared.data_type_pb.js";
import {
    FlowType as TucanaFlowType
} from "@code0-tech/tucana/pb/shared.flow_definition_pb.js";
import {
    RuntimeFunctionDefinition as TucanaFunction
} from "@code0-tech/tucana/pb/shared.runtime_function_pb.js";
import {DataType, FlowType, FunctionDefinition} from "@code0-tech/sagittarius-graphql-types";
import { DefinitionDataType } from "@code0-tech/tucana/pb/shared.data_type_pb.js";

import { mapFlowType } from "../mapper/flowTypeMapper.js";
import { mapFunction } from "../mapper/functionMapper.js";
import { getDataType } from "../mapper/dataTypeMapper.js";

export interface ConstructedDataTypes {
    scannedTucanaTypes: DefinitionDataType[];
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

const definitionCache = new Map<string, Feature[]>();

/*
 * Browser version: downloads a ZIP file, unzips in memory,
 * decodes the Tucana proto binaries, and constructs Feature[].
 */
export async function DefinitionMapper(zipUrl: string): Promise<Feature[]> {
    // return cached result if available
    const cached = definitionCache.get(zipUrl);
    if (cached) return cached;

    // 1. fetch zip as ArrayBuffer
    const res = await fetch(zipUrl);
    if (!res.ok) throw new Error(`Failed to download: ${res.statusText}`);
    const arrayBuffer = await res.arrayBuffer();
    const bytes = new Uint8Array(arrayBuffer);

    // 2. unzip using fflate (browser-compatible)
    const zipEntries = unzipSync(bytes);

    const dataTypes: { feature: string; type: TucanaDataType }[] = [];
    const runtimeFunctions: { feature: string; func: TucanaFunction }[] = [];
    const flowTypes: { feature: string; flow: TucanaFlowType }[] = [];

    // 3. iterate files and decode
    for (const [filename, content] of Object.entries(zipEntries)) {
        if (filename.endsWith("/")) continue; // skip folder entries

        const parts = filename.split("/");
        const base = parts[parts.length - 1];
        const featureName = base.split("_")[0];
        if (!featureName) throw new Error(`Missing feature name for ${filename}`);

        if (base.includes("data_type")) {
            const decoded = TucanaDataType.fromBinary(content);
            dataTypes.push({ feature: featureName, type: decoded });
        } else if (base.includes("function")) {
            const decoded = TucanaFunction.fromBinary(content);
            runtimeFunctions.push({ feature: featureName, func: decoded });
        } else if (base.includes("flow_type")) {
            const decoded = TucanaFlowType.fromBinary(content);
            flowTypes.push({ feature: featureName, flow: decoded });
        }
    }

    const features: Feature[] = [];
    const constructed: ConstructedDataTypes = {
        scannedTucanaTypes: dataTypes.map((f) => f.type),
        constructedDataTypes: [],
        id: 0,
    };

    function getFeature(name: string): Feature {
        let f = features.find((x) => x.name === name);
        if (!f) {
            f = { name, dataTypes: [], flowTypes: [], runtimeFunctions: [] };
            features.push(f);
        }
        return f;
    }

    dataTypes
        .map((f) => ({ name: f.feature, type: getDataType(f.type.identifier, constructed) }))
        .forEach((dt) => {
            if (dt.type) getFeature(dt.name).dataTypes.push(dt.type);
        });

    runtimeFunctions
        .map((f) => ({ name: f.feature, type: mapFunction(f.func, constructed) }))
        .forEach((rf) => {
            if (rf.type) getFeature(rf.name).runtimeFunctions.push(rf.type);
        });

    flowTypes
        .map((f) => ({ name: f.feature, type: mapFlowType(f.flow, constructed) }))
        .forEach((ft) => {
            if (ft.type) getFeature(ft.name).flowTypes.push(ft.type);
        });

    // 5. cache result and return
    definitionCache.set(zipUrl, features);
    return features;
}
