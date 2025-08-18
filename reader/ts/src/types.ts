import {DataType, FlowType, RuntimeFunctionDefinition} from "@code0-tech/sagittarius-graphql-types";

export enum MetaType {
    FlowType = 'FlowType',
    DataType = 'DataType',
    RuntimeFunction = 'RuntimeFunction',
}

export interface Meta {
    name: string;
    type: MetaType;
    data: string;
}

export interface Feature {
    name: string;
    dataTypes: DataType[];
    flowTypes: FlowType[];
    runtimeFunctions: RuntimeFunctionDefinition[];
}