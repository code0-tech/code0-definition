import {Reader} from './reader.js';
import {DataType, FlowType, RuntimeFunctionDefinition} from "@code0-tech/sagittarius-graphql-types";
import {Feature, Meta, MetaType} from "./types.js";

export const Definition = (rootPath: string): Feature[] => {
    const meta = Reader(rootPath);
    if (!meta) return [];
    const features: Feature[] = [];

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
}

function appendMeta(feature: Feature, meta: Meta): void {
    const definition = meta.data;
    try {
        switch (meta.type) {
            case MetaType.DataType: {
                const parsed = JSON.parse(definition) as DataType;
                feature.dataTypes.push(parsed);
                break;
            }
            case MetaType.FlowType: {
                const parsed = JSON.parse(definition) as FlowType;
                feature.flowTypes.push(parsed);
                break;
            }
            case MetaType.RuntimeFunction: {
                const parsed = JSON.parse(definition) as RuntimeFunctionDefinition;
                feature.runtimeFunctions.push(parsed);
                break;
            }
        }
    } catch (err: any) {
        console.error(`Error parsing ${meta.type} ${meta.name} ${definition}:`, err);
    }
}