//! # asset-derive
//!
//! Simple Rust asset loading derive macro for Enums, and a resource for
//! learning proc-macros!
//!
//! Please feel free to offer any advice or create a pull request.
//!
//! The original intent of this library was for compile time loading assets
//! into a binary. This will eventually allow for run-time loading as well,
//! but as for now that will be a future expansion.
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
//!
//! ## TODOs
//!
//! > A list of ideas I have at the moment for this project's expansion.
//! > Please create an issue for a new item to add to this list, using
//! > `todo` label.
//!
//! - [ ] Filename prefix
//! - [ ] Run-time Loading
//!     - [ ] Static (Once on init)
//!     - [ ] Dynamic (Fluid loading)
//! - [X] ~~Compile-time Loading~~

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Generics, Ident, Result};

mod attr;
mod error;
mod ident;
mod variant;

use attr::Attributes;
use error::Error;
use ident::Identifier;
use variant::Variants;

/// Assets type.
///
/// Houses the entire parsed macro contents.
///
/// * `name`: Main enum identifier.
/// * `generics`: Any generic information.
/// * `attrs`: Base level attributes.
/// * `variants`: All variants.
#[derive(Debug)]
struct Assets<'a> {
    name: &'a Ident,
    generics: &'a Generics,
    attrs: Attributes,
    variants: Variants,
}

impl<'a> Assets<'a> {
    /// Creates an `Assets` type from parsed macro input.
    ///
    /// * `input`: Parsed macro input.
    pub fn from(input: &'a DeriveInput) -> Result<Self> {
        Ok(Self {
            name: &input.ident,
            generics: &input.generics,
            attrs: Attributes::from(&input.attrs)?,
            variants: Variants::from(input)?,
        })
    }

    /// Builds the entire macro output.
    pub fn build(self) -> TokenStream {
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let name = self.name;
        let getters = self.variants.build_getters(&self.attrs);
        let arms = self.variants.build_arms();
        quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                #(#getters)*
                pub fn fetch(&self) -> Vec<u8> {
                    match self {
                        #(#arms),*
                    }.to_vec()
                }
            }
        }
        .into()
    }
}

/// Entry function of the derive macro.
#[proc_macro_derive(Asset, attributes(asset))]
pub fn derive_asset(input: TokenStream) -> TokenStream {
    impl_asset(&parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(|err| err.to_compile_error().into())
}

/// Lower implementation of `Asset`.
///
/// * `input`: Parsed macro input.
fn impl_asset(input: &DeriveInput) -> Result<TokenStream> {
    Ok(Assets::from(input)?.build())
}
