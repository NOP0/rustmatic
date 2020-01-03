use crate::{
    lowering::CompilationUnit,
    mir::{
        DuplicateSymbolError, Function, HasType, Location, Name, Scope,
        ScopeRef, Symbol, Type,
    },
    utils::HasLocation,
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

    let global_scope = create_global_scope(world);

    let mut lower = Lower {
        state: world.system_data(),
        global_scope,
        diags,
    };

    for (id, file) in items {
        log::debug!("Lowering file with {:?}", id);
        lower.lower_file(id, &file);
    }

    CompilationUnit { global_scope }
}

/// Create the global [`Scope`] and populate it with various builtin types.
fn create_global_scope(world: &mut World) -> Entity {
    let mut scope = Scope::empty();

    let int = world
        .create_entity()
        .with(Name(String::from("INT")))
        .with(Type::Integer {
            signed: true,
            bit_width: 32,
        })
        .build();
    scope
        .add_symbol("INT".to_string(), Symbol::Type(int))
        .unwrap();

    log::debug!("Created the global scope: {:#?}", scope);

    world.create_entity().with(scope).build()
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
        let current_scope = self.global_scope;
        let name = &function.name.value;
        let location = function.loc(id);
        log::debug!("Lowering function \"{}\" at {:?}", name, location);

        let signature = match self.resolve_function_signature(
            id,
            function,
            current_scope,
        ) {
            Ok(sig) => sig,
            Err(diags) => {
                self.diags.extend(diags);
                return;
            },
        };

        let ent = self
            .state
            .entities
            .build_entity()
            .with(Name(name.to_string()), &mut self.state.names)
            .with(location.clone(), &mut self.state.locations)
            .with(ScopeRef(current_scope), &mut self.state.scope_refs)
            .with(signature, &mut self.state.functions)
            .build();

        self.add_global(name, Symbol::Function(ent), location);
    }

    fn resolve_function_signature(
        &mut self,
        id: FileId,
        function: &syntax::Function,
        current_scope: Entity,
    ) -> Result<Function, Vec<Diagnostic>> {
        let variables = self.resolve_var_blocks(id, &function.var_blocks);

        let return_ident = &function.return_type;
        let return_type = self.get_type_by_name(
            &return_ident.value,
            current_scope,
            return_ident.loc(id),
        );

        // we need to do this funny dance because we want *all* function
        // signature errors to be emitted, not just the first like you'd get
        // using "?"
        match (variables, return_type) {
            (
                Ok(Variables {
                    local_variables,
                    parameters,
                }),
                Ok(return_type),
            ) => Ok(Function {
                local_variables,
                parameters,
                return_type,
            }),
            (Err(var_err), Ok(_)) => Err(var_err),
            (Ok(_), Err(ret_err)) => Err(vec![ret_err]),
            (Err(mut var_errors), Err(ret_err)) => {
                var_errors.push(ret_err);
                Err(var_errors)
            },
        }
    }

    fn resolve_var_blocks(
        &mut self,
        file_id: FileId,
        blocks: &[syntax::VarBlock],
    ) -> Result<Variables, Vec<Diagnostic>> {
        let mut local_variables = Vec::new();
        let mut return_types = Vec::new();
        let mut parameters = Vec::new();
        let mut diags = Vec::new();

        for block in blocks {
            let dest = match block.kind {
                syntax::VarBlockKind::Input => &mut parameters,
                syntax::VarBlockKind::Output => &mut return_types,
                syntax::VarBlockKind::Normal => &mut local_variables,
                _ => unimplemented!(),
            };

            for decl in &block.declarations {
                match self.resolve_var(file_id, decl) {
                    Ok(var) => dest.push(var),
                    Err(diag) => diags.push(diag),
                }
            }
        }

        if diags.is_empty() {
            Ok(Variables {
                local_variables,
                parameters,
            })
        } else {
            Err(diags)
        }
    }

    fn resolve_var(
        &mut self,
        file_id: FileId,
        decl: &syntax::VariableDeclaration,
    ) -> Result<Entity, Diagnostic> {
        let name = Name(decl.name.value.to_string());
        let location = Location {
            file: file_id,
            span: decl.name.span,
        };

        let ty = self.get_type_by_name(
            &decl.declared_type.value,
            self.global_scope,
            location,
        )?;

        let ent = self
            .state
            .entities
            .build_entity()
            .with(name, &mut self.state.names)
            .with(location, &mut self.state.locations)
            .with(HasType { ty }, &mut self.state.has_type)
            .build();

        Ok(ent)
    }
}

struct Variables {
    local_variables: Vec<Entity>,
    parameters: Vec<Entity>,
}

/// Helpers.
impl<'world, 'diag> Lower<'world, 'diag> {
    fn get_type_by_name(
        &self,
        name: &str,
        current_scope: Entity,
        usage_site: Location,
    ) -> Result<Entity, Diagnostic> {
        match self.lookup_symbol(name, current_scope) {
            Some(Symbol::Type(t)) => Ok(t),
            Some(other) => Err(self.incorrect_symbol_type(
                name,
                usage_site,
                Symbol::TYPE,
                other,
            )),
            None => unimplemented!(),
        }
    }

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
        self.diags.push(self.duplicate_symbol_error(e, decl_site));
    }

    /// Try to get a hint with the original item's declaration.
    fn originally_declared_here(&self, item: Entity) -> Option<Label> {
        self.state.locations.get(item).map(|location| {
            Label::new(location.file, location.span, "Originally declared here")
        })
    }

    fn duplicate_symbol_error(
        &self,
        e: &DuplicateSymbolError,
        decl_site: Location,
    ) -> Diagnostic {
        let primary_label = Label::new(
            decl_site.file,
            decl_site.span,
            "Duplicate declared here",
        );
        let mut diag = Diagnostic::new_error(e.to_string(), primary_label);

        if let Some(label) = self.originally_declared_here(e.original.entity())
        {
            diag.secondary_labels.push(label);
        }

        diag
    }

    fn incorrect_symbol_type(
        &self,
        name: &str,
        usage_site: Location,
        expected: &'static str,
        actual: Symbol,
    ) -> Diagnostic {
        let primary_label =
            Label::new(usage_site.file, usage_site.span, "Used here");

        let mut diag = Diagnostic::new_error(
            format!(
                "Tried to use \"{}\" as a {} but it is a {}",
                name,
                expected,
                actual.description(),
            ),
            primary_label,
        );

        if let Some(label) = self.originally_declared_here(actual.entity()) {
            diag.secondary_labels.push(label);
        }

        diag
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codespan::Files;

    #[test]
    fn lower_valid_file() {
        let src = r#"
        FUNCTION add: INT
            VAR_INPUT
                a: INT;
                b: INT;
            END_VAR

            add := a + b;
        END_FUNCTION;
        "#;
        let mut files = Files::new();
        let file_id = files.add("main", src);
        let parsed = syntax::parse(src).unwrap();
        let mut world = World::new();
        let mut diagnostics = Diagnostics::new();

        let got = lower(vec![(file_id, parsed)], &mut world, &mut diagnostics);

        assert!(diagnostics.diagnostics().is_empty());
        let state: State = world.system_data();

        // make sure the function was defined
        let global_scope = state.scopes.get(got.global_scope).unwrap();
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
        assert_eq!(function.parameters.len(), 2, "It accepts two parameters");
        let int = global_scope.lookup("INT").unwrap().entity();
        let a = function.parameters[0];
        assert_eq!(state.names.get(a).unwrap(), "a");
        assert!(state.locations.contains(a), "a has a location");
        assert_eq!(
            state.has_type.get(a).unwrap(),
            &HasType { ty: int },
            "a is an int"
        );
        let b = function.parameters[1];
        assert_eq!(state.names.get(b).unwrap(), "b");
        assert!(state.locations.contains(b), "b has a location");
        assert_eq!(
            state.has_type.get(b).unwrap(),
            &HasType { ty: int },
            "b is an int"
        );
        assert_eq!(function.return_type, int, "It returns an INT");
    }
}
