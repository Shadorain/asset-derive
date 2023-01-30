//! # asset-derive
//!
//! WARN: Not fully implemented, work in progress and will be fully usable soon.
//! Documentation not complete either, this will be done shortly after
//! implementation is in place.
//!
//! Simple rust asset handling derive macro for enums, and a proc-macro learning resource!
//!
//! Feel free to offer any advice or create a pull request. This is my first
//! true attempt at writing proc-macros so it may have some rough edges.
//!
//! The original intent of this library was for compile time loading assets into a binary.
//! This will eventually allow for run-time loading as well, but as for now that will be a
//! future expansion.
//!
//! ## Example
//!
//! ```rust
//! use asset_derive::Asset;
//!
//! #[derive(Asset)]
//! #[asset(basepath = "./icons/", extension = "svg")]
//! enum Icon {
//!     #[asset(extension = "png")]
//!     Select,
//!     Folder,
//!     #[asset(filename = "folder-dim")]
//!     FolderDim,
//! }
//! ```

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
