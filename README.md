<div align="center">

# asset-derive

<a href="https://docs.rs/asset-derive/latest/asset_derive/"> ![Docs](https://img.shields.io/docsrs/asset-derive?color=37d4a7&logo=rust&style=for-the-badge)</a>
<a href="https://crates.io/crates/asset-derive"> ![Crate](https://img.shields.io/crates/v/asset-derive?color=ff4971&style=for-the-badge)</a>
<a href="/LICENSE"> ![License](https://img.shields.io/badge/license-GPL%20v3-blueviolet?style=for-the-badge)</a>
<a href="#todos"> ![TODOs](https://img.shields.io/badge/status-WIP-informational?style=for-the-badge&color=ff69b4) </a>

[Summary](#summary)
•
[Todos](#todos)
•
[Docs](https://docs.rs/asset-derive/latest/asset_derive/)

</div>

<div align="center">

<br>

# Summary

</div>

> Simple Rust asset loading derive macro for Enums, and a resource for learning
proc-macros!

Please feel free to offer any advice or create a pull request.

The original intent of this library was for compile time loading assets
into a binary. This will eventually allow for run-time loading as well,
but as for now that will be a future expansion.

## Example

```rust
use asset_derive::Asset;

#[derive(Asset)]
#[asset(basepath = "./icons/", extension = "svg")]
enum Icon {
    #[asset(extension = "png")]
    Select,
    Folder,
    #[asset(filename = "folder-dim")]
    FolderDim,
}
```

## TODOs

> List of ideas I have at the moment for this project's expansion.
> Please create an issue for a new item to add to this list, using
> `todo` label.

- [ ] Filename prefix
- [ ] Run-time Loading
    - [ ] Static (Once on init)
    - [ ] Dynamic (Fluid loading)
- [X] ~~Compile-time Loading~~
