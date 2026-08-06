use crate::analyser::core::{AnalysableFunction, Analyser};
use crate::diagnostics::diagnose::Diagnose;
use crate::diagnostics::kinds::DiagnosticKind;

impl Analyser {
    pub fn analyse_runtime_function(&mut self, af: &AnalysableFunction) {
        let name = af.function.runtime_name.clone();
        let function = &af.function;
        let original = af.original_definition.clone();

        for linked in function.linked_data_type_identifiers.clone() {
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

        if function.display_icon.is_empty() {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::NullField {
                    field_name: "displayIcon".into(),
                },
            ))
        }

        if function.alias.is_empty() {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::MissingTranslation {
                    translation_field: "alias".into(),
                },
            ));
        }

        if function.display_message.is_empty() {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::MissingTranslation {
                    translation_field: "displayMessage".into(),
                },
            ));
        }

        if function.name.is_empty() {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::UndefinedTranslation {
                    translation_field: "name".into(),
                },
            ));
        }
        if function.description.is_empty() {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::UndefinedTranslation {
                    translation_field: "description".into(),
                },
            ));
        }
        if function.documentation.is_empty() {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::UndefinedTranslation {
                    translation_field: "documentation".into(),
                },
            ));
        }

        if function.signature.is_empty() {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::NullField {
                    field_name: "signature".into(),
                },
            ));
        }

        let mut param_names: Vec<String> = vec![];
        for parameter in &function.runtime_parameter_definitions {
            if parameter.name.is_empty() {
                self.reporter.add(Diagnose::new(
                    name.clone(),
                    original.clone(),
                    DiagnosticKind::UndefinedTranslation {
                        translation_field: "name".into(),
                    },
                ));
            }
            if parameter.description.is_empty() {
                self.reporter.add(Diagnose::new(
                    name.clone(),
                    original.clone(),
                    DiagnosticKind::UndefinedTranslation {
                        translation_field: "description".into(),
                    },
                ));
            }
            if parameter.documentation.is_empty() {
                self.reporter.add(Diagnose::new(
                    name.clone(),
                    original.clone(),
                    DiagnosticKind::UndefinedTranslation {
                        translation_field: "documentation".into(),
                    },
                ));
            }

            if param_names.contains(&parameter.runtime_name) {
                self.reporter.add(Diagnose::new(
                    name.clone(),
                    original.clone(),
                    DiagnosticKind::DuplicateRuntimeParameterIdentifier {
                        identifier: parameter.runtime_name.clone(),
                    },
                ));
            }
            param_names.push(parameter.runtime_name.clone());
        }
    }
}
