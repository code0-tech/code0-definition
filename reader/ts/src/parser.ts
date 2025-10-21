import {Reader} from './reader.ts';
import {DefinitionDataType} from "@code0-tech/tucana/pb/shared.data_type_pb.ts";
import {Feature, Meta, MetaType} from "./types.ts";
import {sortDataTypes} from "./mapper/dataTypeSorter.ts";
import {mapDataType} from "./mapper/dataTypeMapper.ts";

export const Definition = (rootPath: string): Feature[] => {
    const meta = Reader(rootPath);
    if (!meta) return [];
    const features: Feature[] = [];

    const dt = meta.filter(m => m.type == MetaType.DataType).map(m => JSON.parse(m.data) as DefinitionDataType);
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
}


function appendMeta(feature: Feature, meta: Meta): void {

}