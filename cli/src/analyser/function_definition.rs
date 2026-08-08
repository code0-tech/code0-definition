use crate::analyser::core::{AnalysableFunctionDefinition, Analyser};
use crate::diagnostics::diagnose::Diagnose;
use crate::diagnostics::kinds::DiagnosticKind;

impl Analyser {
    pub fn analyse_function_definition(&mut self, afd: &AnalysableFunctionDefinition) {
        let name = afd.function_definition.runtime_name.clone();
        let function = &afd.function_definition;
        let original = afd.original_definition.clone();

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
        for parameter in &function.parameter_definitions {
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

        if self.index.has_function_definition(&name, Some(afd.id)) {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::DuplicateFunctionIdentifier { identifier: name },
            ));
        }
    }
}
