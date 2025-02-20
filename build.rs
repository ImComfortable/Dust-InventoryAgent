extern crate winres;

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico")
            .set("ProductName", "InventoryAgente")
            .set("FileDescription", "InventoryAgente")
            .set("LegalCopyright", "Copyright (c) 2025");
        res.compile().unwrap();
    }
}
