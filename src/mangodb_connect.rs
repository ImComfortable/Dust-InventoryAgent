use mongodb::{options::ClientOptions, Client, bson::doc, bson};
use serde::Serialize;

#[derive(Serialize)]
struct Infos {
    nome: String,
    nomeusuario : String,
    servicetag: String,
    modelo: String,
    versao: String,
    ip: String,
    disco: String,
    processador: String,
    ram: String,
    monitor: String,
    snmonitor: String,
    time: String
}

pub async fn mongodb(
    serial: String,
    namepc: String,
    username: String,
    disk: String,
    rampc: String,
    model: String,
    versao: String,
    ip: String,
    process: String,
    tela: String,
    smonitor: String,
    time: String,
) -> Result<(), mongodb::error::Error> {
    let uri = "mongodb://192.168.1.99:27017";

    // Configurar opções do cliente
    let client_options = ClientOptions::parse(uri).await?;

    // Criar o cliente
    let client = Client::with_options(client_options)?;

    // Acessar o banco de dados e a coleção
    let database = client.database("InfosPC");
    let collection = database.collection("infos");

    // Criar um novo usuário
    let pcinfos = Infos {
        nome: namepc.clone(),
        nomeusuario: username.clone(),
        servicetag: serial.clone(),
        modelo: model.clone(),
        versao: versao.clone(),
        ip: ip.clone(),
        disco: disk.clone(),
        processador: process.clone(),
        ram: rampc.clone(),
        monitor: tela.clone(),
        snmonitor: smonitor.clone(),
        time: time.clone(),
    };

    // Criar o filtro para verificar se a servicetag   já existe
    let filter = doc! { "servicetag": serial.clone() };

    // Verificar se o usuário já existe ou se é "candeias"
    if username.to_lowercase() == "candeias" {
        return Ok(());
    }

    let existing_doc = collection.find_one(filter.clone()).await?;

    if let Some(_) = existing_doc {
        let update = doc! {
            "$set": {
                "nome": namepc,
                "nomeusuario": username,
                "servicetag": serial,
                "modelo": model,
                "versao": versao,
                "ip": ip,
                "disco": disk,
                "processador": process,
                "ram": rampc,
                "monitor": tela,
                "snmonitor": smonitor,
                "time": time
            }
        };
        collection.update_one(filter, update).await?;
    } else {
        // Inserir um novo documento
        let doc = bson::to_document(&pcinfos)?;
        collection.insert_one(doc).await?;
    }

    Ok(())
}