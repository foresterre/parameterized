use std::fmt::Formatter;
use syn::braced;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;

/// An ordered list of attribute arguments, which consists of (id, param-args) pairs.
#[derive(Clone)]
pub struct ParameterizedList {
    pub args: Punctuated<ParameterList, Token![,]>,
}

impl Parse for ParameterizedList {
    /// This part parses
    /// It uses IdentifiedArgList.parse() for each inner argument.
    ///
    /// ['IdentifiedArgList.parse ']: struct.IdentifiedArgList
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(ParameterizedList {
            args: Punctuated::parse_terminated(input)?,
        })
    }
}

/// A single (id, param-args) pair which consists of:
///   - id: identifier for the list
///   - param_args: ordered list arguments formatted using curly-braced list syntax, i.e. "{ 3, 4, 5 }"
///
/// For example:
/// `parameter_name = { 3, 4, 5}`
#[derive(Clone)]
pub struct ParameterList {
    pub id: syn::Ident,
    _assignment: Token![=],
    _braces: syn::token::Brace,
    pub param_args: Punctuated<syn::Expr, Token![,]>,
}

impl std::fmt::Debug for ParameterList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("ParameterList(id = {:?})", self.id))
    }
}

impl Parse for ParameterList {
    // parts:
    //
    // v = { a, b, c }
    // $ident $Token![=] ${ $expr, ... }
    fn parse(input: ParseStream) -> Result<Self> {
        let content;

        Ok(ParameterList {
            id: input.parse()?,
            _assignment: input.parse()?,
            _braces: braced!(content in input),
            param_args: Punctuated::parse_terminated(&content)?,
        })
    }
}
