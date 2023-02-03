use std::fmt::Display;

use crate::{Error, Result};

/// Identifier of an attribute tag below top-level.
///
/// NOTE: Add new sub-attribute names here.
///
/// # Example
///
/// ```no_run
/// #[asset(ident = "value")]
///   ^- top-lvl attribute
///         ^- sub-identifier
/// ```
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Identifier {
    Basepath,
    Filename,
    Extension,
}

impl From<Identifier> for &'static str {
    fn from(value: Identifier) -> Self {
        match value {
            Identifier::Basepath => "basepath",
            Identifier::Filename => "filename",
            Identifier::Extension => "ext",

            #[allow(unreachable_patterns)]
            _ => {
                panic!(
                    "New `Identifier` variants need to be defined here, and in `SubIdent::list()`"
                )
            }
        }
    }
}
impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl Identifier {
    /// Iterator over all `SubIdent` variants.
    pub fn list() -> impl Iterator<Item = Identifier> {
        [Self::Basepath, Self::Filename, Self::Extension].into_iter()
    }
    pub fn str_list() -> Vec<&'static str> {
        Self::list().map(|i| i.into()).collect::<Vec<&str>>()
    }

    pub fn from_string(value: String) -> Result<Self> {
        Ok(Identifier::list()
            .find(|&i| Into::<&str>::into(i) == value)
            .ok_or(Error::Identifier(Some(value)))?)
    }

    pub fn default(&self) -> &'static str {
        match self {
            Identifier::Basepath => "./",
            Identifier::Filename => "file",
            Identifier::Extension => "",
        }
    }
}
