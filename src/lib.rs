mod test;

use std::{fmt::Display, path::PathBuf};

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, spanned::Spanned, Attribute, Data, DeriveInput, Error, Meta, Result, Variant,
};

struct Assets {
    basepath: PathBuf,
    names: Vec<String>,
}

#[derive(Copy, Clone)]
enum AttrType {
    Base,
    Basepath,
    Filename,
}

impl From<AttrType> for &'static str {
    fn from(value: AttrType) -> Self {
        match value {
            AttrType::Base => AttrType::BASE_ATTR,
            AttrType::Basepath => "basepath",
            AttrType::Filename => "filename",
        }
    }
}
impl Display for AttrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl AttrType {
    const BASE_ATTR: &str = "asset";

    fn list() -> impl Iterator<Item = AttrType> {
        [Self::Base, Self::Basepath, Self::Filename].into_iter()
    }

    fn parse(&self, meta: Meta) -> Result<()> {
        match self {
            AttrType::Base => match meta {
                Meta::List(l) => todo!(),
                n => Err(Error::new(
                    n.span(),
                    format!("expected `{}(\"path_literal\")`, got {:#?}", self, n),
                )),
            },
            _ => match meta {
                Meta::NameValue(v) => todo!(),
                n => Err(Error::new(
                    n.span(),
                    format!("expected `{} = \"path_literal\"`, got {:#?}", self, n),
                )),
            },
        }
    }

    /// Retrieves an attribute from known identifier.
    /// Returns `None` if doesn't exist.
    ///
    /// * `attrs`: List of attributes to search.
    /// * `ident`: Attribute identifier to search.
    fn get_attribute<'a>(&'a self, attrs: &'a [Attribute]) -> Option<&'a Attribute> {
        self.get_attributes(attrs).next()
    }

    fn get_attributes<'a>(&'a self, attrs: &'a [Attribute]) -> impl Iterator<Item = &'a Attribute> {
        attrs.iter().filter(|a| {
            a.parse_meta()
                .map_or(false, |b| b.path().is_ident(Into::<&str>::into(*self)))
        })
    }
}

#[proc_macro_derive(Asset, attributes(asset, basepath, filename))]
pub fn derive_asset(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    eprintln!("{:#?}", input);

    // let gen = impl_asset(&ast);
    impl_asset(&input)
        .unwrap_or_else(|err| err.to_compile_error().into())
        .into()
    //
    // // Iterate variants, extracting attributes
    // let attrs: Vec<_> = variants.iter().map(|variant| {
    //     let attrs = variant.attrs.iter()
    //         // Check attribute named "asset(...)"
    //         .find_map(|attr| match attr.path.is_ident("asset") {
    //             true => Some(&attr.tokens),
    //             false => None,
    //         })
    //         .expect("expected attribute macro #[asset(...)] on each variant, found none");
    //
    //     eprintln!("attr: {:#?}", attrs.to_string());
    // }).collect();
}

fn impl_asset(ast: &DeriveInput) -> Result<TokenStream> {
    // Extract enum variants
    let variants: Vec<&Variant> = match &ast.data {
        Data::Enum(ref data) => data.variants.iter().collect(),
        other => panic!("expected Enum, got {:#?}", other),
    };

    Ok(TokenStream::new())
}
