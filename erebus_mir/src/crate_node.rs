#![expect(unused_imports, unused_variables)]

use ariadne::Report;
use erebus_parser::{ident::Ident, statement::TopLevelStatement, Path, RootModule};
use std::{marker::PhantomPinned, pin::Pin, rc::Rc};

use crate::{
    lazy_named_attr_map::LazyNamedAttrMap, statement::NamedStatement, ErrorNode, ErrorNodeOr,
    MirNode,
};

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

    /// Expects a path that was prefixed by `crate::`
    fn path_resolve_attrs(&self, path: Path) {}
}

impl<'a> MirNode for Crate<'a> {
    type Children = NamedStatement<'a>;

    fn path_resolver(&self, mut path: Path) -> ErrorNodeOr<'_, &Self::Children> {
        let path_lead = path
            .remove_leading_segment()
            .expect("Path shouldn't be empty");

        match path_lead.value() {
            "crate" => self.path_resolve_attrs(path),
        };

        todo!()

        // match self
        //     .exports
        //     .as_ref()
        //     .expect("self.exports should be initialized")
        //     .get(&path)
        // {
        //     Some(resolved_statement) => resolved_statement.as_ref().map_err(Rc::clone),

        //     None => Err(Rc::new(ErrorNode::new(
        //         Report::build(ariadne::ReportKind::Error, path.span)
        //             .with_message(format!("The identifier '{path}' could not be found"))
        //             .finish(),
        //     ))),
        // }
    }
}
