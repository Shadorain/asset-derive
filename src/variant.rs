use super::{Attributes, Error, Identifier};
use quote::{
    __private::{Span, TokenStream},
    format_ident, quote,
};
use syn::{Data, DeriveInput, Ident, Result};

#[derive(Debug)]
pub struct Variants(Vec<Variant>);

impl Variants {
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
struct Variant {
    attrs: Attributes,
    name: String,
}

impl Variant {
    pub fn from(var: &'_ syn::Variant) -> Result<Self> {
        Ok(Self {
            attrs: Attributes::from(&var.attrs)?,
            name: var.ident.to_string(),
        })
    }

    pub fn filename(&self) -> String {
        self.attrs
            .get(Identifier::Filename)
            .map(|s| s.to_string())
            .unwrap_or_else(|| self.name.to_lowercase())
    }
    pub fn ext(&self) -> Option<&'_ str> {
        self.attrs.get(Identifier::Extension)
    }
    pub fn path(&self) -> Option<&'_ str> {
        self.attrs.get(Identifier::Basepath)
    }

    pub fn getter(&self) -> Ident {
        format_ident!("get_{}", self.name.to_lowercase())
    }
}
