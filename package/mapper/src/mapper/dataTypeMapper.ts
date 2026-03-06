import {
	DataType,
	DataTypeIdentifier, DataTypeRule,
	DataTypeRuleConnection,
} from "@code0-tech/sagittarius-graphql-types";
import {
	DataTypeIdentifier as TucanaDataTypeIdentifier,
	DefinitionDataTypeRule
} from "@code0-tech/tucana/pb/shared.data_type_pb.js"
import { ConstructedDataTypes, getID } from "../definition/mapper.js";
import { getTranslationConnection } from "./translation.js";

enum DataTypeRulesVariant {
	/** The rule checks if a number falls within a specified range. */
	NumberRange = 'NUMBER_RANGE',
	/** The rule checks if a string matches a specified regular expression. */
	Regex = 'REGEX',
}

function getDataType(identifier: TucanaDataTypeIdentifier, constructedDataTypes: ConstructedDataTypes): DataType | null {
	const constructed: DataType = {
		__typename: "DataType",
		id: `gid://sagittarius/DataType/${getID(constructedDataTypes)}`,
		genericKeys: identifier.genericKeys,
		identifier: identifier.identifier,
		aliases: getTranslationConnection(identifier.alias),
		displayMessages: getTranslationConnection(identifier.displayMessage),
		name: getTranslationConnection(identifier.name),
		// @ts-ignore
		signature: identifier.signature,
		// @ts-ignore
		linkedDataTypeIdentifiers: identifier.linkedDataTypeIdentifier,
		rules: createRules(identifier.rules),
	}
	return constructed
}

function createRules(rule: DefinitionDataTypeRule[]): DataTypeRuleConnection {
	return {
		count: rule.length,
		nodes: rule.map(r => {
			switch (r.config.oneofKind) {
				case "numberRange": {
					const ruleConfig: DataTypeRulesNumberRangeConfig = {
						from: Number(r.config.numberRange.from),
						steps: r.config.numberRange.steps ? Number(r.config.numberRange.steps) : undefined,
						to: Number(r.config.numberRange.to),
					}
					const rule: DataTypeRule = {
						variant: DataTypeRulesVariant.NumberRange,
						config: ruleConfig
					}
					return rule;
				}
				case "regex": {
					const ruleConfig: DataTypeRulesRegexConfig = {
						pattern: r.config.regex.pattern,
					}
					const rule: DataTypeRule = {
						variant: DataTypeRulesVariant.Regex,
						config: ruleConfig
					}
					return rule;
				}
			}
			throw new Error(`Unknown rule: ${rule}`)
		}
		)
	}
}

export { getDataType }
