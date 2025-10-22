import {DefinitionDataType as TucanaDataType} from "@code0-tech/tucana/pb/shared.data_type_pb.ts";
import {Feature} from "./types.ts";
import {readdirSync, readFileSync} from "node:fs";
import {FlowType as TucanaFlowType} from "@code0-tech/tucana/pb/shared.flow_definition_pb.ts";
import {RuntimeFunctionDefinition as TucanaFunction} from "@code0-tech/tucana/pb/shared.runtime_function_pb.ts";
import path from "node:path";
import {mapFlowType} from "./mapper/flowTypeMapper.ts";
import {mapFunction} from "./mapper/functionMapper.ts";
import {DataType, FlowType, FunctionDefinition} from "@code0-tech/sagittarius-graphql-types";
import {DefinitionDataType} from "@code0-tech/tucana/pb/shared.data_type_pb.ts";
import {getDataType} from "./mapper/helper.ts";

export interface ConstructedDataTypes {
    scannedTucanaTypes: DefinitionDataType[]
    constructedDataTypes: DataType[]
}

export const Definition = (rootPath: string) => {
    const dataTypes: TucanaDataType[] = []
    const runtimeFunctions: TucanaFunction[] = [];
    const flowTypes: TucanaFlowType[] = [];
    console.log(rootPath)
    path.join()
    readdirSync(rootPath, { withFileTypes: true }).forEach(file => {
        console.log(file)
        const filePath = path.join(file.parentPath, file.name)
        console.log(filePath)

        const content = readFileSync(filePath);
        if (file.name.includes("data_type")) {
            const decoded = TucanaDataType.fromBinary(content);
            dataTypes.push(decoded)
        }

        if (file.name.includes("function")) {
            const decoded = TucanaFunction.fromBinary(content);
            runtimeFunctions.push(decoded)
        }

        if (file.name.includes("flow_type")) {
            const decoded = TucanaFlowType.fromBinary(content);
            flowTypes.push(decoded)
        }
    })
    const constructed: ConstructedDataTypes = {
        scannedTucanaTypes: dataTypes,
        constructedDataTypes: []
    }
    dataTypes.map(f => getDataType(f.identifier, constructed)).forEach((d: DataType | null) => console.dir(d, {depth: null}))
    runtimeFunctions.map(f => mapFunction(f, constructed)).forEach((f: FunctionDefinition | null) => console.dir(f, {depth: null}))
    flowTypes.map(f => mapFlowType(f, constructed)).forEach((f: FlowType | null) => console.dir(f, {depth: null}))

    //throw new Error("Not implemented")
    /*
    const dt = meta.filter(m => m.type == MetaType.DataType).map(m => {
     //   console.log(m.data)
     //   return DefinitionDataType.fromJsonString(m.data)
     //   console.dir(def, {depth: null})
        return null;
    });
    for (const d of dt) {
      //  console.dir(d, {depth: null})
    }
    const sortedDt = sortDataTypes(dt).map(d => mapDataType(d))

    for (const dt of sortedDt) {
        console.log(dt)
    }

    for (const m of meta) {
        let feature = features.find((f) => f.name === m.name);

        if (feature) {
            appendMeta(feature, m);
        } else {
            feature = {
                name: m.name,
                dataTypes: [],
                flowTypes: [],
                runtimeFunctions: [],
            };
            appendMeta(feature, m);
            features.push(feature);
        }
    }

    return features;

     */
}
