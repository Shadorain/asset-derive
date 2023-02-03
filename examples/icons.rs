//! # Icons Example
//!
//! Uses `Asset` derive macro to load in `fetch`able assets.
//!
//! This example simply just prints out the bytes of the
//! loaded in assets (currently laoded into the actual binary
//! of the example!).
#![allow(dead_code)]
use asset_derive::Asset;

/// Icon houses image and icon assets.
#[derive(Asset)]
#[asset(basepath = "./assets/", ext = "svg")]
enum Icon {
    #[asset(ext = "png")]
    Select,
    Folder,
    #[asset(filename = "folder-dim")]
    FolderDim,
    #[asset(filename = "folder", ext = "jpg")]
    FolderJpg,
    #[asset(ext = "txt")]
    Icon,
}

fn main() {
    println!("Folder: {}", String::from_utf8(Icon::Folder.fetch()).unwrap());
    println!("Icon: {}", String::from_utf8(Icon::Icon.fetch()).unwrap());
}
