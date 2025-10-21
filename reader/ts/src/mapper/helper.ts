import {DataType, DataTypeIdentifier, TranslationConnection} from "@code0-tech/sagittarius-graphql-types";
import {Translation} from "@code0-tech/tucana/pb/shared.translation_pb.ts";

function getDataType(identifier: string | undefined): DataType {
    // @ts-ignore
    return null
}

function getDataTypeIdentifier(identifier: string): DataTypeIdentifier {
    // @ts-ignore
    return null
}

function getTranslationConnection(translation: Translation[]): TranslationConnection {
    return {
        count: translation.length,
        nodes: translation,
    }
}

export {getDataType, getDataTypeIdentifier, getTranslationConnection}