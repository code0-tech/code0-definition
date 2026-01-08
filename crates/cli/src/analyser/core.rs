use crate::analyser::index_identifier::IdentifierIndex;
use crate::diagnostics::diagnose::Diagnose;
use crate::diagnostics::kinds::DiagnosticKind;
use crate::diagnostics::reporter::Reporter;
use crate::parser::Meta;
use tucana::shared::{DefinitionDataType, FlowType, RuntimeFunctionDefinition};

#[derive(Clone)]
pub struct AnalysableDataType {
    pub original_definition: Meta,
    pub definition_data_type: DefinitionDataType,
    pub id: i16,
}

#[derive(Clone)]
pub struct AnalysableFlowType {
    pub original_definition: Meta,
    pub flow_type: FlowType,
    pub id: i16,
}

#[derive(Clone)]
pub struct AnalysableFunction {
    pub original_definition: Meta,
    pub function: RuntimeFunctionDefinition,
    pub id: i16,
}

pub struct Analyser {
    pub reporter: Reporter,
    pub(crate) index: IdentifierIndex,
    pub data_types: Vec<AnalysableDataType>,
    pub flow_types: Vec<AnalysableFlowType>,
    pub functions: Vec<AnalysableFunction>,
}

impl Analyser {
    pub fn new(path: &str) -> Self {
        super::loader::load_from_path(path)
    }

    pub fn report(&mut self, will_exit: bool, with_warning: bool) {
        // Run analysis passes
        for dt in self.data_types.clone() {
            self.analyse_data_type(&dt);
        }
        for ft in self.flow_types.clone() {
            self.analyse_flow_type(&ft);
        }
        for f in self.functions.clone() {
            self.analyse_runtime_function(&f);
        }
        self.reporter.print(will_exit, true, with_warning);
    }

    pub fn data_type_identifier_exists(&self, identifier: &str, except_id: Option<i16>) -> bool {
        self.index.has_data_type(identifier, except_id)
    }

    pub fn generic_key_in_target(&self, key: &str, target: &str) -> bool {
        let norm_target = target.to_ascii_lowercase();
        self.data_types.iter().any(|dt| {
            dt.definition_data_type
                .identifier
                .eq_ignore_ascii_case(&norm_target)
                && dt
                    .definition_data_type
                    .generic_keys
                    .contains(&key.to_string())
        })
    }

    pub fn null_field(&mut self, name: String, adt: &AnalysableDataType) {
        self.reporter.add(Diagnose::new(
            adt.definition_data_type.identifier.clone(),
            adt.original_definition.clone(),
            DiagnosticKind::NullField { field_name: name },
        ));
    }
}
