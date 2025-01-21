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
        ip: ip,
        disco: disk,
        processador: process,
        ram: rampc,
        monitor: tela,
        snmonitor: smonitor,
        time: time,
    };
    
    let client = Client::new();

    let res = client.post("http://localhost:3000/dbinfos")
        .json(&info)
        .send()
        .await?;

    let status = res.status();  
    let body = res.text().await?;  
    
    if status.is_success() {
        println!("Sucesso ao mandar as infos para a API");
    }  else {
        println!("Erro ao mandar as infos para a API {:?}", status);
        println!("Corpo da resposta de erro {}", &body);
    }

    Ok(())
}