use erebus_parser::{ident::Ident, RootModule};
use std::{collections::BTreeMap, marker::PhantomPinned, pin::Pin, sync::LazyLock};

use crate::{statement::NamedStatement, IdentResolverFn, IntoMir, MirNode};

type ExportsBTree<'a> =
    BTreeMap<Ident, LazyLock<NamedStatement<'a>, Box<dyn FnOnce() -> NamedStatement<'a> + 'a>>>;

#[derive(Debug)]
pub struct Crate<'a> {
    exports: Option<ExportsBTree<'a>>,
    _pin: PhantomPinned,
}

impl<'a> Crate<'a> {
    pub fn new(root_module: RootModule) -> Pin<Box<Self>> {
        let mut self_container = Box::new(Self {
            exports: Option::None,
            _pin: PhantomPinned,
        });

        let self_container_pointer = &raw const self_container;
        self_container.exports = Some(Self::new_exports(root_module, move |ident| {
            Self::raw_pointer_ident_resolver(self_container_pointer, ident)
        }));

        self_container.eval();
        Box::into_pin(self_container)
    }

    fn new_exports(
        root_module: RootModule,
        ident_resolver: impl IdentResolverFn<'a, NamedStatement<'a>> + Clone,
    ) -> ExportsBTree<'a> {
        // Safety:
        // Ident resolver is not called in this function
        root_module
            .iter_statements()
            .map(move |statement| {
                let ident_resolver = ident_resolver.clone();

                (
                    statement.get_ident().clone(),
                    LazyLock::new(Box::new(|| {
                        statement.into_mir(move |ident| ident_resolver(ident))
                    })
                        as Box<dyn FnOnce() -> NamedStatement<'a>>),
                )
            })
            .collect()
    }

    fn eval(&mut self) {
        for export in self
            .exports
            .as_mut()
            .expect("Exports should already be initialized when trying to eval items")
        {
            let _: NamedStatement<'a> = **export.1;
        }
    }

    #[inline(always)]
    fn raw_pointer_ident_resolver(
        self_pt: *const Box<Self>,
        ident: Ident,
    ) -> &'a NamedStatement<'a> {
        let self_node = unsafe {
            self_pt
                .as_ref()
                .expect("Non-Null pointer should be passed to `unsafe_ident_resolver`")
        };

        self_node.base_ident_resolver(ident)
    }
}

impl<'a> MirNode for Crate<'a> {
    type Target = NamedStatement<'a>;

    #[expect(unused_variables)]
    fn named_attr(&self, name: Ident, scope: crate::Scope) -> &Self::Target {
        &self
            .exports
            .as_ref()
            .expect("self.exports should be initialized")[&name]
    }
}
