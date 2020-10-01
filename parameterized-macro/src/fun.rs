use syn::export::Formatter;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Attribute, Block, FnArg, Ident, ItemFn, Pat, PatIdent, PatType, Type, Visibility};

pub(crate) struct TestFn {
    fun: ItemFn,
}

impl TestFn {
    pub(crate) fn parameters(&self) -> Vec<(&Ident, &Type)> {
        self.fun
            .sig
            .inputs
            .iter()
            .filter_map(|item| {
                if let FnArg::Typed(PatType { pat, ty, .. }) = item {
                    if let Pat::Ident(PatIdent { ident, .. }) = pat.as_ref() {
                        Some((ident, ty.as_ref()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    pub(crate) fn name(&self) -> &Ident {
        &self.fun.sig.ident
    }

    pub(crate) fn vis(&self) -> &Visibility {
        &self.fun.vis
    }

    pub(crate) fn body(&self) -> &Box<Block> {
        &self.fun.block
    }

    pub(crate) fn attrs(&self) -> &Vec<Attribute> {
        &self.fun.attrs
    }
}

impl std::fmt::Debug for TestFn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TestFn(")?;

        for param in self.parameters() {
            f.write_str(&format!("{:?}, ", param.0))?;
        }

        f.write_str(")")
    }
}

impl Parse for TestFn {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(TestFn {
            fun: input.parse()?,
        })
    }
}
