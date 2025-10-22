import {DataType, FlowType, FunctionDefinition} from "@code0-tech/sagittarius-graphql-types";

export interface Feature {
    name: string;
    dataTypes: DataType[];
    flowTypes: FlowType[];
    runtimeFunctions: FunctionDefinition[];
}