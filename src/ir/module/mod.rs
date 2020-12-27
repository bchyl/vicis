pub mod attributes;
pub mod global_variable;
pub mod linkage;
pub mod name;
pub mod parser;
pub mod preemption_specifier;

pub use parser::parse;

use super::{function::Function, types::Types};
use attributes::Attribute;
use id_arena::Arena;
use rustc_hash::FxHashMap;
use std::fmt;

#[derive(Debug, Clone)]
struct Target {
    triple: String,
    datalayout: String,
}

pub struct Module {
    name: String,
    source_filename: String,
    target: Target,
    functions: Arena<Function>,
    attributes: FxHashMap<u32, Vec<Attribute>>,
    types: Types,
    // TODO: Metadata
}
impl Module {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            source_filename: "".to_string(),
            target: Target::new(),
            functions: Arena::new(),
            attributes: FxHashMap::default(),
            types: Types::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn functions(&self) -> &Arena<Function> {
        &self.functions
    }

    pub fn functions_mut(&mut self) -> &mut Arena<Function> {
        &mut self.functions
    }
}

impl Target {
    pub fn new() -> Self {
        Self {
            triple: "".to_string(),
            datalayout: "".to_string(),
        }
    }
}

impl fmt::Debug for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "source_filename = \"{}\"", self.source_filename)?;
        writeln!(f, "target datalayout = \"{}\"", self.target.datalayout)?;
        writeln!(f, "target triple = \"{}\"", self.target.triple)?;
        writeln!(f)?;
        for (_, func) in &self.functions {
            writeln!(f, "{:?}", func)?;
        }
        for (id, attrs) in &self.attributes {
            write!(f, "attributes #{} = {{ ", id)?;
            for attr in attrs {
                write!(f, "{:?} ", attr)?;
            }
            writeln!(f, "}}")?
        }
        Ok(())
    }
}
