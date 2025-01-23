use serde::{Serialize, Deserialize};
use reqwest::Client;
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct Infos {
    nome: String,
    nomeusuario : String,
    servicetag: String,
    modelo: String,
    versao: String,
    windows: String,
    ip: String,
    disco: String,
    processador: String,
    ram: String,
    monitor: String,
    snmonitor: String,
    time: String
}

pub async fn sendinfos(
    serial: String,
    namepc: String,
    username: String,
    disk: String,
    rampc: String,
    model: String,
    versao: String,
    windows: String,
    ip: String,
    process: String,
    tela: String,
    smonitor: String,
    time: String,
) -> Result<(), Box<dyn Error>>{

    let info = Infos{
        nome: namepc,
        nomeusuario: username,
        servicetag: serial,
        modelo: model,
        versao: versao,
        windows: windows.clone(),
        ip: ip,
        disco: disk,
        processador: process,
        ram: rampc,
        monitor: tela,
        snmonitor: smonitor,
        time: time,
    };
    
    let client = Client::new();
    println!("{}", &windows);

    let res = client.post("http://localhost:3000/dbinfos")
        .json(&info)
        .send()
        .await?;

    let status = res.status();  
    //let body = res.text().await?;  
    
    if status.is_success() {
        println!("Sucesso ao mandar as infos para a API");
    } else if status == reqwest::StatusCode::BAD_REQUEST {
        println!("Erro: Já existe um registro com esta servicetag.");
    }
    else {
        println!("Sem alterações")
    }

    Ok(())
}