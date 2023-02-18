use syn::{
    __private::Span, punctuated::Punctuated, spanned::Spanned, MetaNameValue, Result, Token,
};

use crate::{Error, Identifier};

/// Top-Level Attributes.
///
/// * Stores a collection of it's sub-attributes.
///
/// # Example
///
/// ```ignore
/// #[asset( ... , ... , ... )]
///   ^- top-lvl attribute
///         ^- sub-attributes
/// ```
#[derive(Debug)]
pub struct Attributes(Vec<Attribute>);

/// Allows `Attributes` to hold it's own span.
impl Spanned for Attributes {
    fn span(&self) -> Span {
        Span::call_site()
    }
}

impl Attributes {
    /// Name of the top-level attribute.
    const BASE_ATTR: &str = "asset";

    /// Returns top-level attribute name.
    pub fn base() -> &'static str {
        Self::BASE_ATTR
    }

    /// Creates a new `Attributes` type from a collection of `syn::Attribute`s.
    /// Pass any attributes in and they will be filtered accordingly.
    ///
    /// * `attrs`: List of `syn::Attribute`s to parse.
    pub fn from(attrs: &'_ [syn::Attribute]) -> Result<Self> {
        Ok(Self(
            attrs
                .iter()
                .filter(|a| a.path.is_ident(Attributes::base()))
                .flat_map(|attr| {
                    attr.parse_args_with(Punctuated::<MetaNameValue, Token![,]>::parse_terminated)
                        .unwrap()
                })
                .map(|a| Attribute::from(&a))
                .collect::<Result<Vec<Attribute>>>()?,
        ))
    }

    /// Searches for the specified `Identifier`.
    /// Returns `None` if `ident` is not found.
    ///
    /// * `ident`: `Identifier` to locate.
    pub fn get(&self, ident: Identifier) -> Option<&'_ str> {
        Some(&self.0.iter().find(|a| a.ident == ident)?.value)
    }

    /// Searches for the specified `Identifier`.
    /// Returns default fallback value if `ident` not found.
    ///
    /// * `ident`: `Identifier` to locate.
    pub fn get_or(&self, ident: Identifier) -> &'_ str {
        self.get(ident).unwrap_or(ident.default())
    }
}

/// Sub-Level Attribute.
///
/// * `ident`: Identifies the sub-attribute type.
/// * `value`: Value to set for the sub-attribute.
///
/// # Example
///
/// ```ignore
/// #[asset(ident = "value")]
///   ^- top-lvl attribute
///         ^- sub-attribute
/// ```
#[derive(Debug)]
pub struct Attribute {
    pub ident: Identifier,
    pub value: String,
}

impl Attribute {
    /// Parses an `Attribute` from passed attribute meta data.
    ///
    /// * `meta`: `MetaNameValue` to be parsed.
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
                _ => return Err(Error::Attribute(meta).into()),
            },
        })
    }
}
