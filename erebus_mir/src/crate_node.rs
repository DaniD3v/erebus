use erebus_parser::{ident::Ident, statement::TopLevelStatement, RootModule};
use std::{marker::PhantomPinned, pin::Pin};

use crate::{lazy_named_attr_map::LazyNamedAttrMap, statement::NamedStatement, MirNode};

#[derive(Debug)]
pub struct Crate<'a> {
    exports: Option<LazyNamedAttrMap<'a, NamedStatement<'a>, TopLevelStatement>>,
    _pin: PhantomPinned,
}

impl<'a> Crate<'a> {
    pub fn new(root_module: RootModule) -> Pin<Box<Self>> {
        let mut self_container = Box::new(Self {
            exports: Option::None,
            _pin: PhantomPinned,
        });

        let self_container_pointer = &raw const self_container;
        self_container.exports = Some(unsafe {
            LazyNamedAttrMap::new_from_pointer(
                root_module.iter_statements(),
                self_container_pointer,
            )
        });

        let self_container = Box::into_pin(self_container);
        self_container.exports.as_ref().unwrap().eval();

        self_container
    }
}

impl<'a> MirNode for Crate<'a> {
    type Children = NamedStatement<'a>;

    fn ident_resolver(&self, name: Ident) -> &Self::Children {
        self.exports
            .as_ref()
            .expect("self.exports should be initialized")
            .get(&name)
            .unwrap_or_else(|| panic!("Path at {name:?} could not be resolved at crate root"))
    }
}
