use crate::diagnostics::severity::Severity;

#[derive(Debug, Clone)]
pub enum DiagnosticKind {
    DeserializationError { description: String },
    DuplicateDataTypeIdentifier { identifier: String },
    DuplicateFlowTypeIdentifier { identifier: String },
    DuplicateRuntimeFunctionIdentifier { identifier: String },
    DuplicateRuntimeParameterIdentifier { identifier: String },
    UndefinedDataTypeIdentifier { identifier: String },
    EmptyGenericMapper,
    GenericKeyNotInMappingTarget { key: String, target: String },
    NullField { field_name: String },
    ForbiddenVariant,
    UnusedGenericKey { key: String },
    UndefinedGenericKey { key: String },
    UndefinedTranslation { translation_field: String },
    MissingTranslation { translation_field: String },
}

impl DiagnosticKind {
    pub fn severity(&self) -> Severity {
        use DiagnosticKind::*;
        match self {
            DeserializationError { .. }
            | DuplicateDataTypeIdentifier { .. }
            | DuplicateFlowTypeIdentifier { .. }
            | DuplicateRuntimeFunctionIdentifier { .. }
            | DuplicateRuntimeParameterIdentifier { .. }
            | GenericKeyNotInMappingTarget { .. }
            | EmptyGenericMapper
            | UndefinedDataTypeIdentifier { .. }
            | NullField { .. }
            | ForbiddenVariant
            | UnusedGenericKey { .. }
            | UndefinedGenericKey { .. }
            | MissingTranslation { .. } => Severity::Error,
            UndefinedTranslation { .. } => Severity::Warning,
        }
    }
}
