#![cfg(test)]

use crate::Asset;

#[derive(Asset)]
#[asset(basepath = "./icons/")]
enum Icon {
    Select,
    Folder,
    #[asset(filename = "folder-dim.svg")]
    FolderDim,
}

#[test]
fn it_works() {
    assert_eq!(4, 4);
}
