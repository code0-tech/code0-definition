use crate::formatter::{error, warning};
use std::cmp::PartialEq;
use std::path::Path;
use std::process::exit;
use code0_definition_reader::reader::Meta;

#[derive(Default)]
pub struct Reporter {
    diagnose: Vec<Diagnose>,
}

impl PartialEq for Severity {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Severity::Error => {
                if let Severity::Error = other {
                    return true;
                }
                false
            }
            Severity::Warning => {
                if let Severity::Warning = other {
                    return true;
                }
                false
            }
            Severity::Debug => {
                if let Severity::Debug = other {
                    return true;
                }
                false
            }
        }
    }
}

impl Reporter {
    pub fn add_report(&mut self, diagnose: Diagnose) {
        self.diagnose.push(diagnose);
    }

    pub fn run_report(&self, will_exit: bool) {
        for error in &self.get_errors() {
            println!("{}", error.print());
        }

        for warning in &self.get_warnings() {
            println!("{}", warning.print());
        }

        if !self.get_errors().is_empty() && will_exit {
            exit(1)
        }
    }

    pub fn get_errors(&self) -> Vec<&Diagnose> {
        self.diagnose
            .iter()
            .filter(|p| p.kind.severity() == Severity::Error)
            .collect()
    }

    pub fn get_warnings(&self) -> Vec<&Diagnose> {
        self.diagnose
            .iter()
            .filter(|p| p.kind.severity() == Severity::Warning)
            .collect()
    }

    pub fn get_debug(&self) -> Vec<&Diagnose> {
        self.diagnose
            .iter()
            .filter(|p| p.kind.severity() == Severity::Debug)
            .collect()
    }
}

pub enum Severity {
    Error,
    Warning,
    Debug,
}

pub struct Diagnose {
    kind: DiagnosticKind,
    definition_name: String,
    definition: Meta,
}

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
            | EmptyGenericMapper { .. }
            | UndefinedDataTypeIdentifier { .. }
            | NullField { .. }
            | ForbiddenVariant { .. }
            | UnusedGenericKey { .. }
            | UndefinedGenericKey { .. } => Severity::Error,
            UndefinedTranslation { .. } => Severity::Warning,
        }
    }
}

impl Diagnose {
    pub fn new(
        definition_name: String,
        definition: Meta,
        kind: DiagnosticKind,
    ) -> Self {
        Self {
            definition_name,
            definition,
            kind,
        }
    }

    pub fn print(&self) -> String {
        let path = format!(
            "{}:{}:{}",
            Path::new(&self.definition.path.clone()).display(),
            1,
            1
        );

        use DiagnosticKind::*;
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
                    "`{}` has an empty field  (`{}`) of translations!",
                    self.definition_name, translation_field
                ),
                &path,
            ),
        }
    }
}
