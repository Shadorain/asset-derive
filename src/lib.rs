//! <div align="center">
//!
//! # asset-derive
//!
//! <a href="https://docs.rs/asset-derive/latest/asset_derive/"> ![Docs](https://img.shields.io/docsrs/asset-derive?color=37d4a7&logo=rust&style=for-the-badge)</a>
//! <a href="https://crates.io/crates/asset-derive"> ![Crate](https://img.shields.io/crates/v/asset-derive?color=ff4971&style=for-the-badge)</a>
//! <a href="/LICENSE"> ![License](https://img.shields.io/badge/license-GPL%20v3-blueviolet?style=for-the-badge)</a>
//! <a href="#todos"> ![TODOs](https://img.shields.io/badge/status-WIP-informational?style=for-the-badge&color=ff69b4) </a>
//!
//! [Summary](#summary)
//! •
//! [Todos](#todos)
//! •
//! [Docs](https://docs.rs/asset-derive/latest/asset_derive/)
//!
//! </div>
//!
//! <div align="center">
//!
//! <br>
//!
//! # Summary
//!
//! </div>
//!
//! > Simple Rust asset loading derive macro for Enums, and a resource for learning
//! proc-macros!
//!
//! Please feel free to offer any advice or create a pull request.
//!
//!
//! The original intent of this library was for compile time loading assets
//! into a binary. This will eventually allow for run-time loading as well,
//! but as for now that will be a future expansion.
//!
//! ## TODOs
//!
//! > List of ideas I have at the moment for this project's expansion.
//! > Please create an issue for a new item to add to this list, using
//! > `todo` label.
//!
//! - [ ] Filename prefix
//! - [ ] Run-time Loading
//!     - [ ] Static (Once on init)
//!     - [ ] Dynamic (Fluid loading)
//! - [X] ~~Compile-time Loading~~
//!
//! ## Structure
//!
//! Since `asset-derive` is meant to be a procedural macro crate, while also housing
//! a trait implementation as well (to be derived), there is a somewhat complex
//! project structue. This is because of the current annoyance of proc-macro crates
//! having to be defined completely separate to normal crates.
//!
//! The external API shall stay the same fortunately, `asset-derive` will now be stuck
//! as the trait implementation crate which depends on `asset-derive-macro` which
//! houses the actual macro implementation. This is unavoidable for the time being, but
//! I did the best I could to not have the external API change and make it as simple as
//! can be.
//!
//! ### Code Tree
//!
//! ```no_run
//! asset-derive/               <-- Crate to use (trait implementation)
//!     src/
//!     examples/               <-- Houses examples using the trait and macro itself.
//!     asset-derive-macro/     <-- Actual internal derive macro crate. Will be pulled in by main crate.
//!         src/
//! ```
//!
//! ## Example
//!
//! ```no_run
//! use asset_derive::Asset;
//!
//! #[derive(Asset)]
//! #[asset(basepath = "./icons/", ext = "svg")]
//! enum Icon {
//!     #[asset(ext = "png")]
//!     Select,
//!     Folder,
//!     #[asset(filename = "folder-dim")]
//!     FolderDim,
//! }
//!
//! Icon::Select.fetch()
//! Icon::FolderDim.fetch_static()
//! ```
pub use asset_derive_macro::Asset;

/// Trait to be derived by `asset-derive` macro.
pub trait Asset {
    /// Method responsible for fetching requested resource.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// let data = Icon::Select.fetch();
    /// ```
    fn fetch(&self) -> Vec<u8>;

    /// Method responsible for fetching requested static resource.
    ///
    /// NOTE: this returns an `Option<&'static [u8]>` because it will
    /// only be useable for compile-time loaded resources. Dynamic
    /// will return `None`.
    ///
    /// ## Example
    ///
    /// ```no_run
    /// let static_data = Icon::Select.fetch_static().unwrap();
    /// ```
    fn fetch_static(&self) -> &'static [u8];
}
