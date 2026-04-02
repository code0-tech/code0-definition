use crate::analyser::core::{AnalysableFlowType, Analyser};
use crate::diagnostics::diagnose::Diagnose;
use crate::diagnostics::kinds::DiagnosticKind;

impl Analyser {
    pub fn analyse_flow_type(&mut self, aft: &AnalysableFlowType) {
        let flow = &aft.flow_type;
        let name = flow.identifier.clone();
        let original = aft.original_definition.clone();

        for linked in flow.linked_data_type_identifiers.clone() {
            if !self.data_type_identifier_exists(linked.as_str(), None) {
                self.reporter.add(Diagnose::new(
                    name.clone(),
                    original.clone(),
                    DiagnosticKind::UndefinedDataTypeIdentifier {
                        identifier: linked.clone(),
                    },
                ));
            }
        }

        if flow.display_icon.is_empty() {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::NullField {
                    field_name: "displayIcon".into(),
                },
            ))
        }
        if flow.alias.is_empty() {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::MissingTranslation {
                    translation_field: "alias".into(),
                },
            ));
        }

        if flow.display_message.is_empty() {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::MissingTranslation {
                    translation_field: "displayMessage".into(),
                },
            ));
        }

        if flow.name.is_empty() {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::UndefinedTranslation {
                    translation_field: "name".into(),
                },
            ));
        }
        if flow.description.is_empty() {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::UndefinedTranslation {
                    translation_field: "description".into(),
                },
            ));
        }
        if flow.documentation.is_empty() {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::UndefinedTranslation {
                    translation_field: "documentation".into(),
                },
            ));
        }

        if &flow.signature == "" {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::NullField {
                    field_name: "signature".into(),
                },
            ));
        }

        for setting in &flow.settings {
            if setting.name.is_empty() {
                self.reporter.add(Diagnose::new(
                    setting.identifier.clone(),
                    original.clone(),
                    DiagnosticKind::UndefinedTranslation {
                        translation_field: "flow_setting.name".into(),
                    },
                ));
            }
            if setting.description.is_empty() {
                self.reporter.add(Diagnose::new(
                    setting.identifier.clone(),
                    original.clone(),
                    DiagnosticKind::UndefinedTranslation {
                        translation_field: "flow_setting.description".into(),
                    },
                ));
            }
        }

        if self.index.has_flow_type(&name, Some(aft.id)) {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::DuplicateFlowTypeIdentifier { identifier: name },
            ));
        }
    }
}
