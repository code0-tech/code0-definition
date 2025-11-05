use crate::analyser::core::{AnalysableFlowType, Analyser};
use crate::diagnostics::diagnose::Diagnose;
use crate::diagnostics::kinds::DiagnosticKind;

impl Analyser {
    pub fn analyse_flow_type(&mut self, aft: &AnalysableFlowType) {
        let flow = &aft.flow_type;
        let name = flow.identifier.clone();
        let original = aft.original_definition.clone();

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

        if let Some(identifier) = &flow.input_type_identifier
            && !self.data_type_identifier_exists(identifier, None) {
                self.reporter.add(Diagnose::new(
                    name.clone(),
                    original.clone(),
                    DiagnosticKind::UndefinedDataTypeIdentifier {
                        identifier: identifier.clone(),
                    },
                ));
            }
        if let Some(identifier) = &flow.return_type_identifier
            && !self.data_type_identifier_exists(identifier, None) {
                self.reporter.add(Diagnose::new(
                    name.clone(),
                    original.clone(),
                    DiagnosticKind::UndefinedDataTypeIdentifier {
                        identifier: identifier.clone(),
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
            if !self.data_type_identifier_exists(&setting.data_type_identifier, None) {
                self.reporter.add(Diagnose::new(
                    name.clone(),
                    original.clone(),
                    DiagnosticKind::UndefinedDataTypeIdentifier {
                        identifier: setting.data_type_identifier.clone(),
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
