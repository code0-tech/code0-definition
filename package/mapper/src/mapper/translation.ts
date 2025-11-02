import {Translation} from "@code0-tech/tucana/pb/shared.translation_pb.js";
import {TranslationConnection} from "@code0-tech/sagittarius-graphql-types";

export function getTranslationConnection(translation: Translation[]): TranslationConnection {
    return {
        count: translation.length,
        nodes: translation,
    }
}
