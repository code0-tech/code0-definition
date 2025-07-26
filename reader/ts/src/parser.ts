import {Meta, MetaType, Reader} from './reader';
import {DataType, FlowType, RuntimeFunctionDefinition} from "@code0-tech/sagittarius-graphql-types";

export interface Feature {
    name: string;
    data_types: DataType[];
    flow_types: FlowType[];
    runtime_functions: RuntimeFunctionDefinition[];
}

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
                data_types: [],
                flow_types: [],
                runtime_functions: [],
            };
            appendMeta(feature, m);
            features.push(feature);
        }
    }

    return features;
}

function appendMeta(feature: Feature, meta: Meta): void {
    for (const definition of meta.data) {
        try {
            switch (meta.type) {
                case MetaType.DataType: {
                    const parsed = JSON.parse(definition) as DataType;
                    feature.data_types.push(parsed);
                    break;
                }
                case MetaType.FlowType: {
                    const parsed = JSON.parse(definition) as FlowType;
                    feature.flow_types.push(parsed);
                    break;
                }
                case MetaType.RuntimeFunction: {
                    const parsed = JSON.parse(definition) as RuntimeFunctionDefinition;
                    feature.runtime_functions.push(parsed);
                    break;
                }
            }
        } catch (err: any) {
            console.error(`Error parsing ${meta.type} ${meta.name}:`, err);
        }
    }
}