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
    time: String,
    passwordpost: String,
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
        windows: windows,//.clone(),
        ip: ip,
        disco: disk,
        processador: process,
        ram: rampc,
        monitor: tela,
        snmonitor: smonitor,
        time: time,
        passwordpost: "JolyneTheCat1202.07".to_string()
    };
    
    let client = Client::new();
    //println!("{}", &windows);

    let res = client.post("http://192.168.20.8:3000/dbinfos")
        .json(&info)
        .send()
        .await;

    match res {
        Ok(response) => {
            let status = response.status();

            if status.is_success() {
                println!("Sucesso ao enviar as infos para a api");
            } else {
                println!("Erro ao mandar as infos para a api, status {}", status);
            }
        },
        Err(e) => {
            println!("Erro ao enviar a requisição {}", e);
        }
    }   

    Ok(())
}