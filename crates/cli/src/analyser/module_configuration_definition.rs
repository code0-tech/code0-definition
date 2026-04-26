use crate::analyser::core::{AnalysableModuleConfigurationDefinition, Analyser};
use crate::diagnostics::diagnose::Diagnose;
use crate::diagnostics::kinds::DiagnosticKind;

impl Analyser {
    pub fn analyse_module_configuration_definition(
        &mut self,
        amcd: &AnalysableModuleConfigurationDefinition,
    ) {
        let config = &amcd.module_configuration_definition;
        let name = config.identifier.clone();
        let original = amcd.original_definition.clone();

        if self
            .index
            .has_module_configuration_definition(&name, Some(amcd.id))
        {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::DuplicateModuleConfigurationIdentifier {
                    identifier: name.clone(),
                },
            ));
        }

        for linked in config.linked_data_type_identifiers.clone() {
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

        if config.name.is_empty() {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::UndefinedTranslation {
                    translation_field: "name".into(),
                },
            ));
        }

        if config.description.is_empty() {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::UndefinedTranslation {
                    translation_field: "description".into(),
                },
            ));
        }

        if config.r#type.is_empty() {
            self.reporter.add(Diagnose::new(
                name,
                original,
                DiagnosticKind::NullField {
                    field_name: "type".into(),
                },
            ));
        }
    }
}
