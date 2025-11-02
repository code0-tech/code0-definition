import {readdir, readFile} from "node:fs/promises";
import {join, extname} from "node:path";

import {FlowType} from "@code0-tech/tucana/pb/shared.flow_definition_pb.js";
import {RuntimeFunctionDefinition} from "@code0-tech/tucana/pb/shared.runtime_function_pb.js";
import {DefinitionDataType} from "@code0-tech/tucana/pb/shared.data_type_pb.js";

export const enum MetaType {
    FlowType = "FlowType",
    DataType = "DataType",
    RuntimeFunction = "RuntimeFunction",
}

export interface DefinitionError {
    definition: string;
    definition_type: MetaType;
    error: string;
}

export interface Feature {
    name: string;
    data_types: DefinitionDataType[];
    flow_types: FlowType[];
    runtime_functions: RuntimeFunctionDefinition[];
    errors: DefinitionError[];
}

export class Reader {
    static async fromPath(root: string): Promise<Feature[]> {
        const features = new Map<string, Feature>();

        for (const featureDir of await safeReadDir(root)) {
            if (!featureDir.isDirectory()) continue;
            const featureName = featureDir.name;
            const featurePath = join(root, featureName);

            for (const typeDir of await safeReadDir(featurePath)) {
                const type = toMetaType(typeDir.name);
                if (!type) continue;

                const typePath = join(featurePath, typeDir.name);
                const jsonPaths = await collectJsonFiles(typePath);

                for (const file of jsonPaths) {
                    const def = await readFile(file, "utf8");
                    const f = features.get(featureName) ?? emptyFeature(featureName);
                    addDefinition(f, def, type);
                    features.set(featureName, f);
                }
            }
        }

        return Array.from(features.values());
    }
}

const toMetaType = (folder: string): MetaType | null =>
    ({
        flow_type: MetaType.FlowType,
        data_type: MetaType.DataType,
        runtime_definition: MetaType.RuntimeFunction
    } as const)[folder] ?? null;

const emptyFeature = (name: string): Feature => ({
    name,
    data_types: [],
    flow_types: [],
    runtime_functions: [],
    errors: [],
});

const safeReadDir = async (p: string) => {
    try {
        return await readdir(p, {withFileTypes: true});
    } catch {
        return [];
    }
};

const collectJsonFiles = async (dir: string): Promise<string[]> => {
    const entries = await safeReadDir(dir);
    const files = entries.filter(e => e.isFile() && extname(e.name) === ".json").map(e => join(dir, e.name));

    // include one nested level
    for (const e of entries.filter(e => e.isDirectory())) {
        const sub = (await safeReadDir(join(dir, e.name)))
            .filter(s => s.isFile() && extname(s.name) === ".json")
            .map(s => join(dir, e.name, s.name));
        files.push(...sub);
    }
    return files;
};

const addDefinition = (feature: Feature, def: string, type: MetaType) => {
    try {
        if (type === MetaType.DataType) feature.data_types.push(DefinitionDataType.fromJsonString(def));
        else if (type === MetaType.FlowType) feature.flow_types.push(FlowType.fromJsonString(def));
        else feature.runtime_functions.push(RuntimeFunctionDefinition.fromJsonString(def));
    } catch (err) {
        feature.errors.push({
            definition: extractIdentifier(def, type),
            definition_type: type,
            error: err instanceof Error ? err.message : String(err),
        });
    }
};

const extractIdentifier = (def: string, type: MetaType): string => {
    const key = type === MetaType.RuntimeFunction ? "runtime_name" : "identifier";
    return def.match(new RegExp(`"${key}"\\s*:\\s*"([^"]+)"`))?.[1] ?? def;
};

