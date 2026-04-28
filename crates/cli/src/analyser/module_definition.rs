use crate::analyser::core::{AnalysableModuleDefinition, Analyser};
use crate::diagnostics::diagnose::Diagnose;
use crate::diagnostics::kinds::DiagnosticKind;

impl Analyser {
    pub fn analyse_module_definition(&mut self, amd: &AnalysableModuleDefinition) {
        let module = &amd.module_definition;
        let name = if module.identifier.is_empty() {
            "<module>".to_string()
        } else {
            module.identifier.clone()
        };
        let original = amd.original_definition.clone();

        if module.identifier.is_empty() {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::NullField {
                    field_name: "identifier".into(),
                },
            ));
        }
        if module.name.is_empty() {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::UndefinedTranslation {
                    translation_field: "name".into(),
                },
            ));
        }
        if module.description.is_empty() {
            self.reporter.add(Diagnose::new(
                name.clone(),
                original.clone(),
                DiagnosticKind::UndefinedTranslation {
                    translation_field: "description".into(),
                },
            ));
        }
        if module.icon.is_empty() {
            self.reporter.add(Diagnose::new(
                name,
                original,
                DiagnosticKind::NullField {
                    field_name: "icon".into(),
                },
            ));
        }
    }
}
