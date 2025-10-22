import {
    DefinitionDataType,
    DefinitionDataTypeRule
} from "@code0-tech/tucana/pb/shared.data_type_pb.ts";
import {DataTypeIdentifier} from "@code0-tech/tucana/pb/shared.data_type_pb.ts";

function topologicalSort(items: { type: string; dependsOn: string[] }[]) {
    const map = new Map<string, { type: string; dependsOn: string[] }>();
    const visited = new Set<string>();
    const visiting = new Set<string>();
    const result: { type: string; dependsOn: string[] }[] = [];

    // Build a lookup map
    for (const item of items) {
        map.set(item.type, item);
    }

    // Recursive DFS
    function visit(type: string) {
        if (visited.has(type)) return;
        if (visiting.has(type)) {
            throw new Error(`Cyclic dependency detected involving "${type}"`);
        }

        visiting.add(type);
        const item = map.get(type);
        if (item) {
            for (const dep of item.dependsOn) {
                visit(dep);
            }
            result.push(item);
        }
        visiting.delete(type);
        visited.add(type);
    }

    // Visit all nodes
    for (const item of items) {
        visit(item.type);
    }

    return result;
}

function resolveDataTypeIdentifier(dataTypeIdentifier: DataTypeIdentifier | undefined): string[] {
    const result: string[] = [];

    if (dataTypeIdentifier == undefined) {
        return result
    }
    const dataType: any = dataTypeIdentifier!

    if (dataType.type.GenericType) {
        result.push(dataType.type.GenericType.data_type_identifier)
        // @ts-ignore
        dataType.type.GenericType.generic_mappers.forEach(mapper => {
            // @ts-ignore
            mapper.source.forEach(source => {
                result.push(...resolveDataTypeIdentifier(source))
            })
        })
        return result
    }

    if (dataType.type.DataTypeIdentifier) {
        return [dataType.type.DataTypeIdentifier]
    }

    return result;
}

function getDependencies(rule: DefinitionDataTypeRule): string[] {
    const config: any = rule.config;
    if (config.ContainsType) {
        const dataTypeIdentifier = config.ContainsType.data_type_identifier
        return resolveDataTypeIdentifier(dataTypeIdentifier)
    }

    if (config.InputTypes) {
        // @ts-ignore
        const dataTypes = config.InputTypes.input_types.map(input => input.data_type_identifier)
        return dataTypes.map(resolveDataTypeIdentifier).flat()
    }

    if (config.ParentType) {
        return resolveDataTypeIdentifier(config.ParentType.parent_type)
    }

    if (config.ReturnType) {
        return resolveDataTypeIdentifier(config.ReturnType.data_type_identifier)
    }

    return []
}

function sortDataTypes(types: DefinitionDataType[]) {
    const identifiable = types.map(type => {
        const dependencies = type.rules?.map(rule => getDependencies(rule)).flat() ?? []
        return {
            type: type.identifier,
            dependsOn: dependencies
        }
    })

    const sorted = topologicalSort(identifiable)

    for (const item of sorted) {
        console.log(item)
    }

    return sorted.map(item => types.find(type => type.identifier === item.type))
}

export {sortDataTypes}