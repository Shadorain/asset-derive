use syn::{
    __private::Span, punctuated::Punctuated, spanned::Spanned, MetaNameValue, Result,
    Token,
};

use crate::{Error, Identifier};

/// Top-Level Attribute.
///
/// * Stores a collection of it's sub-attributes.
///
/// # Example
///
/// ```no_run
/// #[asset( ... , ... , ... )]
///   ^- top-lvl attribute
///         ^- sub-attributes
/// ```
#[derive(Debug)]
pub struct Attribute(Vec<SubAttribute>);

impl Spanned for Attribute {
    fn span(&self) -> Span {
        Span::call_site()
    }
}

impl Attribute {
    const BASE_ATTR: &str = "asset";

    pub fn base() -> &'static str {
        Self::BASE_ATTR
    }

    /// Creates a new `TopAttribute` from a collection of `syn::Attribute`s
    /// Pass any attributes in and they will be filtered accordingly.
    ///
    /// * `attrs`: List of Attributes to parse.
    pub fn new(attrs: &'_ [syn::Attribute]) -> Result<Self> {
        Ok(Self(
            attrs
                .iter()
                .filter(|a| a.path.is_ident(Attribute::base()))
                .flat_map(|attr| {
                    attr.parse_args_with(Punctuated::<MetaNameValue, Token![,]>::parse_terminated)
                        .unwrap()
                })
                .map(|a| SubAttribute::from(&a))
                .collect::<Result<Vec<SubAttribute>>>()?,
        ))
    }
}

/// Sub-Level Attribute.
///
/// * `ident`: Identifies the sub-attribute type.
/// * `value`: Value to set for the sub-attribute.
/// * `span`: Location of this rust block.
///
/// # Example
///
/// ```no_run
/// #[asset(ident = "value")]
///   ^- top-lvl attribute
///         ^- sub-attribute
/// ```
#[derive(Debug)]
pub struct SubAttribute {
    pub ident: Identifier,
    value: String,
}

impl SubAttribute {
    pub fn from(meta: &'_ MetaNameValue) -> Result<Self> {
        // let meta = meta.clone();
        Ok(Self {
            ident: Identifier::from_string(
                meta.path
                    .get_ident()
                    .ok_or(Error::Identifier(None))?
                    .to_string(),
            )?,
            value: match meta.lit {
                syn::Lit::Str(ref s) => s.value(),
                _ => return Err(Error::SubAttribute(meta).into()),
            },
        })
    }
}
