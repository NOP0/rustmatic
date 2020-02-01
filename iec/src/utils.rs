use crate::mir::Location;
use codespan::FileId;
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

impl Extend<Diagnostic> for Diagnostics {
    fn extend<I: IntoIterator<Item = Diagnostic>>(&mut self, items: I) {
        self.diags.extend(items);
    }
}

/// Something which has a location in the source code.
pub trait HasLocation {
    fn loc(&self, file_id: FileId) -> Location;
}

macro_rules! impl_has_location {
    ($( $type:ty ),* $(,)?) => {
        $(
            impl HasLocation for $type {
                fn loc(&self, file_id: FileId) -> Location {
                    Location {
                        file: file_id,
                        span: self.span,
                    }
                }
            }
        )*
    };
}

impl_has_location! {
    rustmatic_structured_text::Identifier,
    rustmatic_structured_text::Function,
    rustmatic_structured_text::FunctionBlock,
    rustmatic_structured_text::Program,
}
