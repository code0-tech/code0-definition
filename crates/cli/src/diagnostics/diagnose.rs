use crate::diagnostics::kinds::DiagnosticKind;
use crate::diagnostics::kinds::DiagnosticKind::*;
use crate::diagnostics::severity::Severity;
use crate::formatter::{error, warning};
use crate::reader::Meta;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Diagnose {
    pub kind: DiagnosticKind,
    pub definition_name: String,
    pub definition: Meta,
}

impl Diagnose {
    pub fn new(definition_name: String, definition: Meta, kind: DiagnosticKind) -> Self {
        Self {
            definition_name,
            definition,
            kind,
        }
    }
    pub fn print(&self) -> String {
        let path = format!("{}:{}:{}", Path::new(&self.definition.path).display(), 1, 1);
        match &self.kind {
            DeserializationError { description } => error(
                format!("A JSON parsing error occurred: `{}`", description),
                &path,
            ),
            DuplicateDataTypeIdentifier { identifier } => error(
                format!(
                    "The data_type `{}` is already defined resulting in a duplicate!",
                    identifier
                ),
                &path,
            ),
            DuplicateFlowTypeIdentifier { identifier } => error(
                format!(
                    "The flow_type `{}` is already defined resulting in a duplicate!",
                    identifier
                ),
                &path,
            ),
            DuplicateRuntimeFunctionIdentifier { identifier } => error(
                format!(
                    "The runtime_function `{}` is already defined resulting in a duplicate!",
                    identifier
                ),
                &path,
            ),
            DuplicateRuntimeParameterIdentifier { identifier } => error(
                format!(
                    "The runtime_parameter `{}` is already defined resulting in a duplicate!",
                    identifier
                ),
                &path,
            ),
            UndefinedDataTypeIdentifier { identifier } => error(
                format!(
                    "`{}` uses an undefined data_type_identifier: `{}`!",
                    self.definition_name, identifier
                ),
                &path,
            ),
            NullField { field_name } => error(
                format!(
                    "`{}` has a field (`{}`) that is null!",
                    self.definition_name, field_name
                ),
                &path,
            ),
            UndefinedTranslation { translation_field } => warning(
                format!(
                    "`{}` has an empty field (`{}`) of translations!",
                    self.definition_name, translation_field
                ),
                &path,
            ),
            MissingTranslation { translation_field } => error(
                format!(
                    "`{}` has an required empty field (`{}`) of translations!",
                    self.definition_name, translation_field
                ),
                &path,
            ),
        }
    }

    pub fn severity(&self) -> Severity {
        self.kind.severity()
    }
}
