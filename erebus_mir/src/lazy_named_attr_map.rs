#![expect(dead_code)]

use std::{
    collections::BTreeMap,
    iter::once,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    sync::LazyLock,
};

use erebus_parser::ident::Ident;

use crate::{IdentResolverFn, IntoMir, MirNode};

type RawNamedAttrMap<'a, Emit> = BTreeMap<Ident, LazyLock<Emit, Box<dyn FnOnce() -> Emit + 'a>>>;

#[derive(Debug)]
pub(crate) struct LazyNamedAttrMap<'a, Emit: 'a, AstType: IntoMir<'a, Target = Emit> + 'a> {
    named_attrs: RawNamedAttrMap<'a, Emit>,
    _phantom_ast: PhantomData<AstType>,
}

impl<'a, Emit: 'a, AstType: IntoMir<'a, Target = Emit> + Clone>
    LazyNamedAttrMap<'a, Emit, AstType>
{
    fn new_from_closure(
        ast_items: impl Iterator<Item = AstType>,
        ident_resolver: impl IdentResolverFn<'a, AstType::IdentResolverOutput> + Clone,
    ) -> Self {
        Self {
            named_attrs: {
                ast_items
                    .flat_map(move |statement| {
                        let mut idents: Vec<_> = statement.get_idents().cloned().collect();

                        // Avoiding an expensive `.clone()` call
                        if idents.len() == 1 {
                            Box::new(once(((idents.remove(0)), statement)))
                                as Box<dyn Iterator<Item = (Ident, AstType)>>
                        }
                        // e.g. use crate::{A, B};
                        else {
                            Box::new(
                                idents
                                    .into_iter()
                                    .map(move |ident| (ident, statement.clone())),
                            )
                        }
                    })
                    .map(|(ident, statement)| {
                        let ident_resolver = ident_resolver.clone();

                        (
                            ident.clone(),
                            LazyLock::new(Box::new(|| statement.into_mir(ident_resolver))
                                as Box<dyn FnOnce() -> Emit>),
                        )
                    })
                    .collect()
            },

            _phantom_ast: PhantomData,
        }
    }

    pub fn new<T: MirNode<Children = AstType::IdentResolverOutput>>(
        ast_items: impl Iterator<Item = AstType>,
        parent: &'a T,
    ) -> Self {
        Self::new_from_closure(ast_items, |ident| parent.ident_resolver(ident))
    }

    /// Safety:
    /// It needs to be guaranteed that `parent` is not
    /// modified while this pointer is accessed.
    pub unsafe fn new_from_pointer<T: MirNode<Children = AstType::IdentResolverOutput> + 'a>(
        ast_items: impl Iterator<Item = AstType>,
        parent: *const T,
    ) -> Self {
        // `*const T` can only turn into `&T` when the ident_resolver is called.
        // This relaxes the safety guarantees.

        Self::new_from_closure(ast_items, move |ident| {
            let parent_ref = parent
                .as_ref()
                .expect("Non-Null pointer should be passed to `unsafe_ident_resolver`");

            parent_ref.ident_resolver(ident)
        })
    }

    pub fn eval(&self) {
        self.named_attrs.iter().for_each(|item| {
            let _: Emit = **item.1;
        });
    }
}

impl<'a, Emit: 'a, AstType: IntoMir<'a, Target = Emit>> Deref
    for LazyNamedAttrMap<'a, Emit, AstType>
{
    type Target = RawNamedAttrMap<'a, Emit>;

    fn deref(&self) -> &Self::Target {
        &self.named_attrs
    }
}

impl<'a, Emit: 'a, AstType: IntoMir<'a, Target = Emit>> DerefMut
    for LazyNamedAttrMap<'a, Emit, AstType>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.named_attrs
    }
}
