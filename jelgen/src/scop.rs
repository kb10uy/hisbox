//! SCOP - Scriptable Contest Operation

mod datetime;
mod error;
mod library;

use std::path::{Path, PathBuf};

use rhai::{
    Engine, exported_module,
    module_resolvers::{FileModuleResolver, ModuleResolversCollection, StaticModuleResolver},
};

use crate::scop::error::ScopError;

#[derive(Debug)]
pub struct Scop {
    engine: Engine,
}

impl Scop {
    pub fn create(script_path: impl Into<PathBuf>) -> Result<Scop, ScopError> {
        let script_path = script_path.into();
        let Some(script_dir) = script_path.parent() else {
            return Err(ScopError::InvalidPath(script_path));
        };

        let engine = Self::create_engine(script_dir);

        Ok(Scop { engine })
    }

    fn create_engine(script_dir: &Path) -> Engine {
        let mut engine = Engine::new();

        let resolver = {
            let script_path_resolver = FileModuleResolver::new_with_path(script_dir);

            let mut scop_modules_resolver = StaticModuleResolver::new();
            scop_modules_resolver.insert("jarl", exported_module!(library::jarl));

            let mut resolver_collection = ModuleResolversCollection::new();
            resolver_collection.push(script_path_resolver);
            resolver_collection.push(scop_modules_resolver);
            resolver_collection
        };
        engine.set_module_resolver(resolver);

        engine
    }
}
