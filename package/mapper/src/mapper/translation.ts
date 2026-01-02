import {Translation as TucanaTranslation} from "@code0-tech/tucana/pb/shared.translation_pb.js";
import {Translation} from "@code0-tech/sagittarius-graphql-types";

export function getTranslationConnection(translation: TucanaTranslation[]): Translation[] {
    return translation.map(t => ({
        __typename: 'Translation',
        code: t.code,
        content: t.content
    }))
}