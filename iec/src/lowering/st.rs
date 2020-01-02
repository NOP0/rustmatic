use crate::{
    lowering::CompilationUnit,
    mir::{
        DuplicateSymbolError, Function, Location, Name, Scope, ScopeRef, Symbol,
    },
    Diagnostics,
};
use codespan::FileId;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use rustmatic_structured_text as syntax;
use shred_derive::SystemData;
use specs::prelude::*;

/// Lower a set of *Structured Text* files into a [`Configuration`].
pub fn lower<I>(
    items: I,
    world: &mut World,
    diags: &mut Diagnostics,
) -> CompilationUnit
where
    I: IntoIterator<Item = (FileId, syntax::File)>,
{
    crate::mir::register(world);

    let global_scope = world.create_entity().with(Scope::root()).build();

    let mut lower = Lower {
        state: world.system_data(),
        global_scope,
        diags,
    };

    for (id, file) in items {
        lower.lower_file(id, &file);
    }

    CompilationUnit { global_scope }
}

#[derive(SystemData)]
struct State<'world> {
    entities: Entities<'world>,
    functions: WriteStorage<'world, Function>,
    locations: WriteStorage<'world, Location>,
    names: WriteStorage<'world, Name>,
    scopes: WriteStorage<'world, Scope>,
    scope_refs: WriteStorage<'world, ScopeRef>,
}

struct Lower<'world, 'diag> {
    state: State<'world>,
    diags: &'diag mut Diagnostics,
    global_scope: Entity,
}

impl<'world, 'diag> Lower<'world, 'diag> {
    fn add_global(
        &mut self,
        name: &str,
        item: Symbol,
        decl_site: Location,
    ) -> Result<(), DuplicateSymbolError> {
        self.try_add_to_scope(self.global_scope, name, item, decl_site)
    }

    fn try_add_to_scope(
        &mut self,
        scope: Entity,
        name: &str,
        item: Symbol,
        decl_site: Location,
    ) -> Result<(), DuplicateSymbolError> {
        let scope = self
            .state
            .scopes
            .get_mut(scope)
            .expect("The global scope always exists");

        let got = scope.add_symbol(name.to_string(), item);

        if let Err(ref e) = got {
            let primary_label = Label::new(
                decl_site.file,
                decl_site.span,
                "Duplicate declared here",
            );
            let diag = Diagnostic::new_error(e.to_string(), primary_label);

            self.diags.push(diag);
        }

        got
    }

    fn lower_file(&mut self, id: FileId, ast: &syntax::File) {
        for function in &ast.functions {
            self.lower_function(id, function);
        }
    }

    fn lower_function(&mut self, id: FileId, function: &syntax::Function) {
        let name = &function.name.value;
        let location = Location {
            file: id,
            span: function.span,
        };

        let ent = self
            .state
            .entities
            .build_entity()
            .with(Name(name.to_string()), &mut self.state.names)
            .with(location.clone(), &mut self.state.locations)
            .with(ScopeRef(self.global_scope), &mut self.state.scope_refs)
            .build();

        let _ = self.add_global(name, Symbol::Function(ent), location);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codespan::Files;
    use codespan_reporting::diagnostic::Severity;

    #[test]
    #[ignore]
    fn lower_valid_file() {
        let src = r#"
        FUNCTION add
            VAR_INPUT
                a: INT;
                b: INT;
            END_VAR
            VAR_OUTPUT
                result: INT;
            END_VAR

            result := a + b;
        END_FUNCTION
        "#;
        let mut files = Files::new();
        let file_id = files.add("main", src);
        let parsed = syntax::parse(src).unwrap();
        let mut world = World::new();
        let mut diagnostics = Diagnostics::new();

        let got = lower(vec![(file_id, parsed)], &mut world, &mut diagnostics);

        assert_eq!(diagnostics.at_least(Severity::Warning).count(), 0);
        let state: State = world.system_data();

        // make sure the function was defined
        let global_scope = state.scopes.get(got.global_scope).unwrap();
        assert_eq!(global_scope.symbol_table.len(), 1);
        assert!(global_scope.symbol_table.contains_key("add"));

        // we expect certain information about the function to exist
        let function_symbol = global_scope.symbol_table.get("add").unwrap();
        let function_ent = function_symbol.entity();
        assert!(state.locations.contains(function_ent), "It has a location");
        assert_eq!(
            state.names.get(function_ent).unwrap(),
            "add",
            "Should have been named"
        );
        let function = state.functions.get(function_ent).unwrap();
        assert_eq!(function.variables.len(), 2, "It accepts two parameters");
        assert_eq!(state.names.get(function.variables[0]).unwrap(), "a");
        assert_eq!(state.names.get(function.variables[1]).unwrap(), "b");
        assert!(function.return_type.is_some(), "It has a return type");
    }
}
