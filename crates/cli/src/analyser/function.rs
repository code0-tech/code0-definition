use crate::analyser::core::{AnalysableFunction, Analyser};
use crate::diagnostics::diagnose::Diagnose;
use crate::diagnostics::kinds::DiagnosticKind;
use tucana::shared::DataTypeIdentifier;
use tucana::shared::data_type_identifier::Type;

impl Analyser {
    pub fn analyse_runtime_function(&mut self, af: &AnalysableFunction) {
        let name = af.function.runtime_name.clone();
        let function = &af.function;
        let original = af.original_definition.clone();

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

        let mut detected_generic_keys: Vec<String> = vec![];
        if let Some(identifier) = &function.return_type_identifier {
            detected_generic_keys.append(&mut self.walk_function_dti(
                &name,
                &original,
                identifier.clone(),
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

            if let Some(identifier) = &parameter.data_type_identifier {
                detected_generic_keys.append(&mut self.walk_function_dti(
                    &name,
                    &original,
                    identifier.clone(),
                ));
            } else {
                self.reporter.add(Diagnose::new(
                    name.clone(),
                    original.clone(),
                    DiagnosticKind::NullField {
                        field_name: "data_type".into(),
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

        for key in function
            .generic_keys
            .iter()
            .filter(|k| !detected_generic_keys.contains(k))
            .cloned()
        {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::UnusedGenericKey { key },
            ));
        }
        for key in detected_generic_keys
            .into_iter()
            .filter(|k| !function.generic_keys.contains(k))
        {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::UndefinedGenericKey { key },
            ));
        }
    }

    fn walk_function_dti(
        &mut self,
        name: &str,
        original: &crate::parser::Meta,
        identifier: DataTypeIdentifier,
    ) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        if let Some(t) = identifier.r#type {
            match t {
                Type::DataTypeIdentifier(dt) => {
                    if !self.data_type_identifier_exists(&dt, None) {
                        self.reporter.add(Diagnose::new(
                            name.to_string(),
                            original.clone(),
                            DiagnosticKind::UndefinedDataTypeIdentifier { identifier: dt },
                        ));
                    }
                }
                Type::GenericType(gt) => {
                    if !self.data_type_identifier_exists(&gt.data_type_identifier, None) {
                        self.reporter.add(Diagnose::new(
                            name.to_string(),
                            original.clone(),
                            DiagnosticKind::UndefinedDataTypeIdentifier {
                                identifier: gt.data_type_identifier.clone(),
                            },
                        ));
                    }
                    if gt.generic_mappers.is_empty() {
                        self.reporter.add(Diagnose::new(
                            name.to_string(),
                            original.clone(),
                            DiagnosticKind::EmptyGenericMapper,
                        ));
                    }
                    for mapper in &gt.generic_mappers {
                        for source in mapper.source.clone() {
                            result.append(&mut self.walk_function_dti(name, original, source));
                        }
                        if !self.generic_key_in_target(&mapper.target, &gt.data_type_identifier) {
                            self.reporter.add(Diagnose::new(
                                name.to_string(),
                                original.clone(),
                                DiagnosticKind::GenericKeyNotInMappingTarget {
                                    key: mapper.target.clone(),
                                    target: gt.data_type_identifier.clone(),
                                },
                            ));
                        }
                    }
                }
                Type::GenericKey(key) => result.push(key.clone()),
            }
        }
        result
    }
}
