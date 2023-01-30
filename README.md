# asset-derive

Simple rust asset handling derive macro for enums, and a proc-macro learning resource!

Feel free to offer any advice or create a pull request. This is my first
true attempt at writing proc-macros so it may have some rough edges.

The original intent of this library was for compile time loading assets into a binary.
This will eventually allow for run-time loading as well, but as for now that will be a
future expansion.

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
