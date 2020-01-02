use crate::{
    lowering::CompilationUnit,
    mir::{
        DuplicateSymbolError, Function, HasType, Location, Name, Scope,
        ScopeRef, Symbol,
    },
    Diagnostics,
};
use codespan::FileId;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use rustmatic_structured_text as syntax;
use shred_derive::SystemData;
use specs::prelude::*;

/// Lower a set of *Structured Text* files into a [`CompilationUnit`].
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

/// Helper struct containing the various components we'll need during the
/// lowering process.
#[derive(SystemData)]
struct State<'world> {
    entities: Entities<'world>,
    functions: WriteStorage<'world, Function>,
    locations: WriteStorage<'world, Location>,
    names: WriteStorage<'world, Name>,
    scopes: WriteStorage<'world, Scope>,
    scope_refs: WriteStorage<'world, ScopeRef>,
    has_type: WriteStorage<'world, HasType>,
}

/// Temporary state used while lowering.
struct Lower<'world, 'diag> {
    state: State<'world>,
    diags: &'diag mut Diagnostics,
    global_scope: Entity,
}

/// Lowering functions.
impl<'world, 'diag> Lower<'world, 'diag> {
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
        let function = self.resolve_var_blocks(id, &function.var_blocks);

        let ent = self
            .state
            .entities
            .build_entity()
            .with(Name(name.to_string()), &mut self.state.names)
            .with(location.clone(), &mut self.state.locations)
            .with(ScopeRef(self.global_scope), &mut self.state.scope_refs)
            .with(function, &mut self.state.functions)
            .build();

        self.add_global(name, Symbol::Function(ent), location);
    }

    fn resolve_var_blocks(
        &mut self,
        file_id: FileId,
        blocks: &[syntax::VarBlock],
    ) -> Function {
        let mut local_variables = Vec::new();
        let mut return_types = Vec::new();
        let mut parameters = Vec::new();

        for block in blocks {
            let dest = match block.kind {
                syntax::VarBlockKind::Input => &mut parameters,
                syntax::VarBlockKind::Output => &mut return_types,
                syntax::VarBlockKind::Normal => &mut local_variables,
                _ => unimplemented!(),
            };

            for decl in &block.declarations {
                let name = Name(decl.name.value.to_string());
                let location = Location {
                    file: file_id,
                    span: decl.name.span,
                };

                let ty = match self
                    .lookup_symbol(&decl.declared_type.value, self.global_scope)
                {
                    Some(Symbol::Type(ty)) => HasType { ty },
                    Some(_other) => unimplemented!(),
                    None => unimplemented!(),
                };

                let ent = self
                    .state
                    .entities
                    .build_entity()
                    .with(name, &mut self.state.names)
                    .with(location, &mut self.state.locations)
                    .with(ty, &mut self.state.has_type)
                    .build();

                dest.push(ent);
            }
        }

        Function {
            local_variables,
            return_types,
            parameters,
        }
    }
}

/// Helpers.
impl<'world, 'diag> Lower<'world, 'diag> {
    fn lookup_symbol(
        &self,
        name: &str,
        current_scope: Entity,
    ) -> Option<Symbol> {
        let mut current_scope = Some(current_scope);

        while let Some(scope) = current_scope {
            let scope = self
                .state
                .scopes
                .get(scope)
                .expect("This entity should be a scope");

            if let Some(got) = scope.lookup(name) {
                return Some(got);
            }

            current_scope = scope.parent;
        }

        None
    }

    fn add_global(&mut self, name: &str, item: Symbol, decl_site: Location) {
        self.add_to_scope(self.global_scope, name, item, decl_site)
    }

    fn add_to_scope(
        &mut self,
        scope: Entity,
        name: &str,
        item: Symbol,
        decl_site: Location,
    ) {
        let scope = self
            .state
            .scopes
            .get_mut(scope)
            .expect("The global scope always exists");

        if let Err(e) = scope.add_symbol(name.to_string(), item) {
            self.emit_duplicate_symbol_error(&e, decl_site);
        }
    }
}

/// Diagnostics.
impl<'world, 'diag> Lower<'world, 'diag> {
    fn emit_duplicate_symbol_error(
        &mut self,
        e: &DuplicateSymbolError,
        decl_site: Location,
    ) {
        let primary_label = Label::new(
            decl_site.file,
            decl_site.span,
            "Duplicate declared here",
        );
        let mut diag = Diagnostic::new_error(e.to_string(), primary_label);

        // try to emit a hint with the original item's declaration
        if let Some(original_location) =
            self.state.locations.get(e.original.entity())
        {
            diag.secondary_labels.push(Label::new(
                original_location.file,
                original_location.span,
                "Original declared here",
            ));
        }

        self.diags.push(diag);
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Variables {
    input_parameters: Vec<Entity>,
    output_parameters: Vec<Entity>,
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

        // check the function signature
        let function = state.functions.get(function_ent).unwrap();
        assert_eq!(
            function.local_variables.len(),
            2,
            "It accepts two parameters"
        );
        let a = function.local_variables[0];
        assert_eq!(state.names.get(a).unwrap(), "a");
        assert!(state.locations.contains(a));
        let b = function.local_variables[1];
        assert_eq!(state.names.get(b).unwrap(), "b");
        assert!(state.locations.contains(b));
        assert_eq!(function.return_types.len(), 1, "It has one return value");
    }
}
