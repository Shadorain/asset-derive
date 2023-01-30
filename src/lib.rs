use std::path::PathBuf;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Result, Variant};

mod attr;
mod error;
mod ident;

use attr::{Attribute, Attributes};
use error::Error;
use ident::Identifier;

struct Assets {
    basepath: PathBuf,
    names: Vec<String>,
}

impl Assets {
    /// Should build up the full quote and generated code.
    pub fn build(self) {
        todo!();
    }
}

#[proc_macro_derive(Asset, attributes(asset))]
pub fn derive_asset(input: TokenStream) -> TokenStream {
    impl_asset(&parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|err| err.to_compile_error().into())
}

fn impl_asset(input: &DeriveInput) -> Result<TokenStream> {
    // Extract enum variants
    let variants: Vec<&Variant> = match &input.data {
        Data::Enum(ref data) => data.variants.iter().collect(),
        _ => Err(Error::Data(input))?,
    };
    eprintln!("Variants: {:#?}", variants);

    let top = Attributes::new(&input.attrs)?;
    eprintln!("TOP: {:#?}", top);

    Ok(TokenStream::new())
}
