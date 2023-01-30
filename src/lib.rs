use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Result};

mod attr;
mod error;
mod ident;

use attr::Attributes;
use error::Error;
use ident::Identifier;

#[derive(Debug)]
#[allow(dead_code)]
struct Assets {
    attrs: Attributes,
    variants: Vec<Variant>,
}

impl Assets {
    pub fn new(input: &DeriveInput) -> Result<Self> {
        Ok(Self {
            attrs: Attributes::new(&input.attrs)?,
            variants: match &input.data {
                Data::Enum(ref data) => data
                    .variants
                    .iter()
                    .map(Variant::new)
                    .collect::<Result<Vec<Variant>>>()?,
                _ => Err(Error::Data(input))?,
            },
        })
    }

    /// Should build up the full quote and generated code.
    #[allow(dead_code)]
    pub fn build(self) {
        todo!();
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Variant {
    attrs: Attributes,
    name: String,
}

impl Variant {
    pub fn new(var: &'_ syn::Variant) -> Result<Self> {
        Ok(Self {
            attrs: Attributes::new(&var.attrs)?,
            name: var.ident.to_string(),
        })
    }
}

#[proc_macro_derive(Asset, attributes(asset))]
pub fn derive_asset(input: TokenStream) -> TokenStream {
    impl_asset(&parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|err| err.to_compile_error().into())
}

fn impl_asset(input: &DeriveInput) -> Result<TokenStream> {
    let assets = Assets::new(input);
    eprintln!("Assets: {:#?}", assets);

    Ok(TokenStream::new())
}
