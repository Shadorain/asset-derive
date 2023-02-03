use super::{Attributes, Error, Identifier};
use quote::{
    __private::{Span, TokenStream},
    format_ident, quote,
};
use syn::{Data, DeriveInput, Ident, Result};

/// Variants wrapper.
///
/// * Stores a collection of all parsed enum variants.
#[derive(Debug)]
pub struct Variants(Vec<Variant>);

impl Variants {
    /// Creates a list of variants from a macro input.
    ///
    /// * `input`: Parsed macro input.
    pub fn from(input: &'_ DeriveInput) -> Result<Self> {
        Ok(Self(match input.data {
            Data::Enum(ref data) => data
                .variants
                .iter()
                .map(Variant::from)
                .collect::<Result<Vec<Variant>>>()?,
            _ => Err(Error::Data(input))?,
        }))
    }

    /// Builds a list of getter methods for each variant.
    ///
    /// * `attrs`: Base-level attribute list.
    ///
    /// ## Example
    ///
    /// ```
    /// fn get_select() -> &'static [u8] {
    ///     include_bytes!("./assets/select.png")
    /// }
    /// ```
    pub fn build_getters(&self, attrs: &'_ Attributes) -> Vec<TokenStream> {
        let ext = attrs.get_or(Identifier::Extension);
        let basepath = attrs.get_or(Identifier::Basepath);
        self.0
            .iter()
            .map(|var| {
                let getter = var.getter();
                let full_path = format!(
                    "{}{}.{}",
                    var.path().unwrap_or(basepath),
                    var.filename(),
                    var.ext().unwrap_or(ext)
                );
                quote! {
                    fn #getter() -> &'static [u8] {
                        include_bytes!(#full_path)
                    }
                }
            })
            .collect()
    }

    /// Builds a list of match arms for each variant.
    ///
    /// ## Example
    ///
    /// ```
    /// Variant1 => Self::VARIANT1
    /// }
    /// ```
    pub fn build_arms(&self) -> Vec<TokenStream> {
        self.0
            .iter()
            .map(|var| {
                let variant = Ident::new(&var.name, Span::call_site());
                let getter = var.getter();
                quote! {
                    Self::#variant => Self::#getter()
                }
            })
            .collect()
    }
}

#[derive(Debug)]
/// Parsed Variant type (from `syn::Variant`).
///
/// * `attrs`: Variant's attribute list.
/// * `name`: Identifier of the enum variant.
///
/// ## Example
///
/// ```
/// #[asset(filename = "folder", ext = "jpg")]
/// FolderJpg,
/// ```
struct Variant {
    attrs: Attributes,
    name: String,
}

impl Variant {
    /// Creates a `Variant` from a `syn::Variant`.
    ///
    /// * `var`: variant to be parsed.
    pub fn from(var: &'_ syn::Variant) -> Result<Self> {
        Ok(Self {
            attrs: Attributes::from(&var.attrs)?,
            name: var.ident.to_string(),
        })
    }

    /// Returns an overriden attribute defined filename
    /// or the enum identifier lowercased.
    pub fn filename(&self) -> String {
        self.attrs
            .get(Identifier::Filename)
            .map(|s| s.to_string())
            .unwrap_or_else(|| self.name.to_lowercase())
    }

    /// Returns extension from attribute if exists.
    pub fn ext(&self) -> Option<&'_ str> {
        self.attrs.get(Identifier::Extension)
    }

    /// Returns basepath from attribute if exists.
    pub fn path(&self) -> Option<&'_ str> {
        self.attrs.get(Identifier::Basepath)
    }

    /// Builds a getter method identifier.
    pub fn getter(&self) -> Ident {
        format_ident!("get_{}", self.name.to_lowercase())
    }
}
