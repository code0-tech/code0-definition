use crate::diagnostics::diagnose::Diagnose;
use crate::diagnostics::severity::Severity;
use std::process::exit;

#[derive(Default)]
pub struct Reporter {
    diagnostics: Vec<Diagnose>,
}

impl Reporter {
    pub fn add(&mut self, d: Diagnose) {
        self.diagnostics.push(d);
    }

    pub fn errors(&self) -> impl Iterator<Item = &Diagnose> {
        self.diagnostics
            .iter()
            .filter(|d| d.severity() == Severity::Error)
    }

    pub fn warnings(&self) -> impl Iterator<Item = &Diagnose> {
        self.diagnostics
            .iter()
            .filter(|d| d.severity() == Severity::Warning)
    }

    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    pub fn print(&self, will_exit: bool, print_errors: bool, print_warning: bool) {
        if print_warning {
            for d in self.warnings() {
                println!("{}", d.print());
            }
        }

        if print_errors {
            for d in self.errors() {
                println!("{}", d.print());
            }
        }

        if self
            .diagnostics
            .iter()
            .any(|d| d.severity() == Severity::Error)
            && will_exit
        {
            exit(1)
        }
    }
}
