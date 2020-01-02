use codespan_reporting::diagnostic::{Diagnostic, Severity};

#[derive(Debug, Default, Clone)]
pub struct Diagnostics {
    diags: Vec<Diagnostic>,
}

impl Diagnostics {
    pub fn new() -> Self { Diagnostics::default() }

    pub fn diagnostics(&self) -> &[Diagnostic] { &self.diags }

    /// Gets all [`Diagnostic`]s with at least the provided [`Severity`] level.
    pub fn at_least(
        &self,
        severity: Severity,
    ) -> impl Iterator<Item = &Diagnostic> + '_ {
        self.diags
            .iter()
            .filter(move |diag| diag.severity >= severity)
    }

    pub fn push(&mut self, diag: Diagnostic) { self.diags.push(diag); }
}
