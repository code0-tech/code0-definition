use crate::diagnostics::kinds::DiagnosticKind;
use crate::diagnostics::kinds::DiagnosticKind::*;
use crate::diagnostics::severity::Severity;
use crate::formatter::{error, warning};
use crate::parser::Meta;
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
            EmptyGenericMapper => error(
                format!(
                    "`{}` defined a generic_type but its mapper are empty!`",
                    self.definition_name
                ),
                &path,
            ),
            DeserializationError { description } => error(
                format!("A JSON paring error occurred: `{}`", description),
                &path,
            ),
            GenericKeyNotInMappingTarget { key, target } => error(
                format!(
                    "`{}` is mapping the key: {} onto the target: {}. But the target did not define this generic_key!",
                    self.definition_name, key, target
                ),
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
            ForbiddenVariant => error(
                format!(
                    "The data_type variant of `{}` is 0 and thus incorrect!",
                    self.definition_name
                ),
                &path,
            ),
            UnusedGenericKey { key } => error(
                format!(
                    "`{}` defined a generic_key (`{}`) that is never used!",
                    self.definition_name, key
                ),
                &path,
            ),
            UndefinedGenericKey { key } => error(
                format!(
                    "`{}` uses a generic_key (`{}`) that's not defined!",
                    self.definition_name, key
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
