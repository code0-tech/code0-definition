use crate::diagnostics::diagnose::Diagnose;
use crate::diagnostics::kinds::DiagnosticKind;
use crate::diagnostics::reporter::Reporter;
use crate::parser::ModuleConfiguration;
use crate::{analyser::index_identifier::IdentifierIndex, reader::Meta};
use tucana::shared::{
    DefinitionDataType, FlowType, FunctionDefinition, ModuleConfigurationDefinition,
    RuntimeFlowType, RuntimeFunctionDefinition,
};

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

#[derive(Clone)]
pub struct AnalysableRuntimeFlowType {
    pub original_definition: Meta,
    pub runtime_flow_type: RuntimeFlowType,
    pub id: i16,
}

#[derive(Clone)]
pub struct AnalysableFunctionDefinition {
    pub original_definition: Meta,
    pub function_definition: FunctionDefinition,
    pub id: i16,
}

#[derive(Clone)]
pub struct AnalysableModuleConfigurationDefinition {
    pub original_definition: Meta,
    pub module_configuration_definition: ModuleConfigurationDefinition,
    pub id: i16,
}

#[derive(Clone)]
pub struct AnalysableModuleDefinition {
    pub original_definition: Meta,
    pub module_definition: ModuleConfiguration,
    pub id: i16,
}

pub struct Analyser {
    pub reporter: Reporter,
    pub(crate) index: IdentifierIndex,
    pub data_types: Vec<AnalysableDataType>,
    pub flow_types: Vec<AnalysableFlowType>,
    pub functions: Vec<AnalysableFunction>,
    pub runtime_flow_types: Vec<AnalysableRuntimeFlowType>,
    pub function_definitions: Vec<AnalysableFunctionDefinition>,
    pub module_configuration_definitions: Vec<AnalysableModuleConfigurationDefinition>,
    pub module_definitions: Vec<AnalysableModuleDefinition>,
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
        for rft in self.runtime_flow_types.clone() {
            self.analyse_runtime_flow_type(&rft);
        }
        for f in self.function_definitions.clone() {
            self.analyse_function_definition(&f);
        }
        for config in self.module_configuration_definitions.clone() {
            self.analyse_module_configuration_definition(&config);
        }
        for module in self.module_definitions.clone() {
            self.analyse_module_definition(&module);
        }
        self.reporter.print(will_exit, true, with_warning);
    }

    pub fn data_type_identifier_exists(&self, identifier: &str, except_id: Option<i16>) -> bool {
        self.index.has_data_type(identifier, except_id)
    }

    pub fn null_field(&mut self, name: String, adt: &AnalysableDataType) {
        self.reporter.add(Diagnose::new(
            adt.definition_data_type.identifier.clone(),
            adt.original_definition.clone(),
            DiagnosticKind::NullField { field_name: name },
        ));
    }
}
