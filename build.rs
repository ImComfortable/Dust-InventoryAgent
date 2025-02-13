extern crate winres;

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico") // Caminho para o arquivo .ico
            .set("ProductName", "InventoryAgente") // Nome do aplicativo
            .set("FileDescription", "InventoryAgente") // Descrição do arquivo
            .set("LegalCopyright", "Copyright (c) 2023"); // Direitos autorais
        res.compile().unwrap();
    }
}