use crate::diagnostics::severity::Severity;

#[derive(Debug, Clone)]
pub enum DiagnosticKind {
    DeserializationError { description: String },
    DuplicateDataTypeIdentifier { identifier: String },
    DuplicateFlowTypeIdentifier { identifier: String },
    DuplicateRuntimeFunctionIdentifier { identifier: String },
    DuplicateRuntimeParameterIdentifier { identifier: String },
    UndefinedDataTypeIdentifier { identifier: String },
    NullField { field_name: String },
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
            | UndefinedDataTypeIdentifier { .. }
            | NullField { .. }
            | MissingTranslation { .. } => Severity::Error,
            UndefinedTranslation { .. } => Severity::Warning,
        }
    }
}
