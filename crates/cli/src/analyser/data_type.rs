use crate::analyser::core::{AnalysableDataType, Analyser};
use crate::diagnostics::diagnose::Diagnose;
use crate::diagnostics::kinds::DiagnosticKind;
use tucana::shared::DataTypeIdentifier;
use tucana::shared::data_type_identifier::Type;
use tucana::shared::definition_data_type_rule::Config;

impl Analyser {
    pub fn analyse_data_type(&mut self, adt: &AnalysableDataType) {
        let dt = &adt.definition_data_type;
        if self.index.has_data_type(&dt.identifier, Some(adt.id)) {
            self.reporter.add(Diagnose::new(
                dt.identifier.clone(),
                adt.original_definition.clone(),
                DiagnosticKind::DuplicateDataTypeIdentifier {
                    identifier: dt.identifier.clone(),
                },
            ));
        }

        if dt.variant == 0 {
            self.reporter.add(Diagnose::new(
                dt.identifier.clone(),
                adt.original_definition.clone(),
                DiagnosticKind::ForbiddenVariant,
            ));
        }

        let mut detected: Vec<String> = vec![];
        for optional_rule in &dt.rules {
            if let Some(config) = &optional_rule.config {
                match config {
                    Config::ContainsKey(rule) => {
                        if let Some(dti) = &rule.data_type_identifier {
                            self.walk_data_type_identifier(adt, dti, &mut detected);
                        } else {
                            self.null_field("definition_data_type_contains_key_rule".into(), adt);
                        }
                    }
                    Config::ContainsType(rule) => {
                        if let Some(dti) = &rule.data_type_identifier {
                            self.walk_data_type_identifier(adt, dti, &mut detected);
                        } else {
                            self.null_field("definition_data_type_contains_type_rule".into(), adt);
                        }
                    }
                    Config::ItemOfCollection(rule) => {
                        if rule.items.is_empty() {
                            self.null_field(
                                "definition_data_type_item_of_collection_rule".into(),
                                adt,
                            );
                        }
                    }
                    Config::NumberRange(_) | Config::Regex(_) => {}
                    Config::InputTypes(rule) => {
                        if rule.input_types.is_empty() {
                            self.null_field("definition_data_type_input_types_rule".into(), adt);
                        }
                        for input in &rule.input_types {
                            if let Some(dti) = &input.data_type_identifier {
                                self.walk_data_type_identifier(adt, dti, &mut detected);
                            } else {
                                self.reporter.add(Diagnose::new(
                                    dt.identifier.clone(),
                                    adt.original_definition.clone(),
                                    DiagnosticKind::UndefinedDataTypeIdentifier {
                                        identifier: dt.identifier.clone(),
                                    },
                                ));
                            }
                        }
                    }
                    Config::ReturnType(rule) => {
                        if let Some(dti) = &rule.data_type_identifier {
                            self.walk_data_type_identifier(adt, dti, &mut detected);
                        } else {
                            self.null_field("definition_data_type_return_type_rule".into(), adt);
                        }
                    }
                    Config::ParentType(rule) => {
                        if let Some(dti) = &rule.parent_type {
                            self.walk_data_type_identifier(adt, dti, &mut detected);
                        } else {
                            self.null_field("definition_data_type_parent_type_rule".into(), adt);
                        }
                    }
                }
            } else {
                self.null_field("rule".into(), adt);
            }
        }

        for key in dt.generic_keys.iter().filter(|k| !detected.contains(k)) {
            self.reporter.add(Diagnose::new(
                dt.identifier.clone(),
                adt.original_definition.clone(),
                DiagnosticKind::UnusedGenericKey { key: key.clone() },
            ));
        }
        for key in detected
            .into_iter()
            .filter(|k| !dt.generic_keys.contains(k))
        {
            self.reporter.add(Diagnose::new(
                dt.identifier.clone(),
                adt.original_definition.clone(),
                DiagnosticKind::UndefinedGenericKey { key },
            ));
        }

        if dt.name.is_empty() {
            self.reporter.add(Diagnose::new(
                dt.identifier.clone(),
                adt.original_definition.clone(),
                DiagnosticKind::UndefinedTranslation {
                    translation_field: "name".into(),
                },
            ));
        }
    }

    fn walk_data_type_identifier(
        &mut self,
        adt: &AnalysableDataType,
        dti: &DataTypeIdentifier,
        acc: &mut Vec<String>,
    ) {
        if let Some(t) = &dti.r#type {
            match t {
                Type::DataTypeIdentifier(identifier) => {
                    if !self.data_type_identifier_exists(identifier, Some(adt.id)) {
                        self.reporter.add(Diagnose::new(
                            adt.definition_data_type.identifier.clone(),
                            adt.original_definition.clone(),
                            DiagnosticKind::UndefinedDataTypeIdentifier {
                                identifier: identifier.clone(),
                            },
                        ));
                    }
                }
                Type::GenericType(generic) => {
                    if !self
                        .data_type_identifier_exists(&generic.data_type_identifier, Some(adt.id))
                    {
                        self.reporter.add(Diagnose::new(
                            adt.definition_data_type.identifier.clone(),
                            adt.original_definition.clone(),
                            DiagnosticKind::UndefinedDataTypeIdentifier {
                                identifier: generic.data_type_identifier.clone(),
                            },
                        ));
                    }
                    if generic.generic_mappers.is_empty() {
                        self.reporter.add(Diagnose::new(
                            adt.definition_data_type.identifier.clone(),
                            adt.original_definition.clone(),
                            DiagnosticKind::EmptyGenericMapper,
                        ));
                    }
                    for mapper in &generic.generic_mappers {
                        if adt
                            .definition_data_type
                            .generic_keys
                            .contains(&mapper.target)
                        {
                            acc.push(mapper.target.clone());
                        }
                        for source in &mapper.source {
                            self.walk_data_type_identifier(adt, source, acc);
                        }
                    }
                }
                Type::GenericKey(key) => acc.push(key.clone()),
            }
        } else {
            self.null_field("data_type".into(), adt);
        }
    }
}
