use crate::analyser::core::{AnalysableRuntimeFlowType, Analyser};
use crate::diagnostics::diagnose::Diagnose;
use crate::diagnostics::kinds::DiagnosticKind;

impl Analyser {
    pub fn analyse_runtime_flow_type(&mut self, arft: &AnalysableRuntimeFlowType) {
        let flow = &arft.runtime_flow_type;
        let name = flow.identifier.clone();
        let original = arft.original_definition.clone();

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

        if flow.signature.is_empty() {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::NullField {
                    field_name: "signature".into(),
                },
            ));
        }

        for setting in &flow.runtime_settings {
            if setting.name.is_empty() {
                self.reporter.add(Diagnose::new(
                    setting.identifier.clone(),
                    original.clone(),
                    DiagnosticKind::UndefinedTranslation {
                        translation_field: "runtime_flow_setting.name".into(),
                    },
                ));
            }
            if setting.description.is_empty() {
                self.reporter.add(Diagnose::new(
                    setting.identifier.clone(),
                    original.clone(),
                    DiagnosticKind::UndefinedTranslation {
                        translation_field: "runtime_flow_setting.description".into(),
                    },
                ));
            }
        }

        if self.index.has_runtime_flow_type(&name, Some(arft.id)) {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::DuplicateRuntimeFlowTypeIdentifier { identifier: name },
            ));
        }
    }
}
