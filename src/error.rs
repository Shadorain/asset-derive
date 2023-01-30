use syn::{spanned::Spanned, DeriveInput, MetaNameValue};

use crate::{Attributes, Identifier};

pub enum Error<'a> {
    Attributes(&'a Attributes),
    Attribute(&'a MetaNameValue),
    Identifier(Option<String>),
    Data(&'a DeriveInput),
}

impl<'a> From<Error<'a>> for syn::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::Attributes(attr) => syn::Error::new(
                attr.span(),
                format!(
                    "expected `{}(..., ...)`, got {:#?}",
                    Attributes::base(),
                    attr
                ),
            ),
            Error::Attribute(meta) => syn::Error::new(
                meta.span(),
                format!("expected `sub_attribute = \" ... \"`, got {:#?}", meta),
            ),
            // This one is quite a struggle... definitely something to refactor...
            Error::Identifier(ref s) => syn::Error::new(
                quote::__private::Span::call_site(),
                format!(
                    "expected `[{}]`, got {}",
                    Identifier::str_list().join(", "),
                    s.as_ref().unwrap_or(&"None".to_string())
                ),
            ),
            Error::Data(input) => {
                syn::Error::new_spanned(input, format!("expected Enum, got {:#?}", input.data))
            }
        }
    }
}
