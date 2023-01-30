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

fn main() {}
