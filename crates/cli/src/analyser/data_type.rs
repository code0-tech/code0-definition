use crate::analyser::core::{AnalysableDataType, Analyser};
use crate::diagnostics::diagnose::Diagnose;
use crate::diagnostics::kinds::DiagnosticKind;
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
        for linked in dt.linked_data_type_identifiers.clone() {
            if !self.data_type_identifier_exists(linked.as_str(), None) {
                self.reporter.add(Diagnose::new(
                    dt.identifier.clone(),
                    adt.original_definition.clone(),
                    DiagnosticKind::UndefinedDataTypeIdentifier {
                        identifier: linked.clone(),
                    },
                ));
            }
        }

        if dt.alias.is_empty() {
            self.reporter.add(Diagnose::new(
                dt.identifier.clone(),
                adt.original_definition.clone(),
                DiagnosticKind::MissingTranslation {
                    translation_field: "alias".into(),
                },
            ));
        }

        if dt.display_message.is_empty() {
            self.reporter.add(Diagnose::new(
                dt.identifier.clone(),
                adt.original_definition.clone(),
                DiagnosticKind::MissingTranslation {
                    translation_field: "displayMessage".into(),
                },
            ));
        }

        if dt.r#type == "" {
            self.reporter.add(Diagnose::new(
                dt.identifier.clone(),
                adt.original_definition.clone(),
                DiagnosticKind::NullField {
                    field_name: "type".into(),
                },
            ));
        }

        for optional_rule in &dt.rules {
            if let Some(config) = &optional_rule.config {
                match config {
                    Config::NumberRange(_) | Config::Regex(_) => {}
                }
            } else {
                self.null_field("rule".into(), adt);
            }
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
}
