//! Translate from individual languages (e.g. *Structured Text*) to our
//! mid-level internal representation.

use codespan::FileId;
use rustmatic_structured_text::File;
use specs::prelude::*;

/// Translate a set of *Structured Text* files into a [`Configuration`].
pub fn translate_structured_text<I>(_items: I, _world: &World) -> Entity
where
    I: IntoIterator<Item = (FileId, File)>,
{
    unimplemented!()
}
