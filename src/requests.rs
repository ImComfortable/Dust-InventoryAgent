use serde::{Serialize, Deserialize};
use reqwest::Client;

#[derive(Serialize, Deserialize, Debug)]
pub struct Infos {
    pub nome: String,
    pub nomeusuario : String,
    pub servicetag: String,
    pub modelo: String,
    pub versao: String,
    pub windows: String,
    pub ip: String,
    pub disco: String,
    pub processador: String,
    pub ram: String,
    pub monitor: String,
    pub snmonitor: String,
    pub time: String,
    pub passwordpost: String,
}

pub async fn sendinfos(info: Infos) -> Result<(), ()> {
    let client = reqwest::Client::new();
    match client.post("http://192.168.1.99:3000/dbinfos")
        .json(&info)
        .send()
        .await {
        Ok(res) => {
            if res.status().is_success() {
                println!("Sucesso ao enviar as infos para a api");
            } else {
                println!("Erro ao mandar as infos para a api, status {}", res.status());
            }
            Ok(())
        }
        Err(_) => {
            println!("Falha na conexão com a API");
            Ok(())
        }
    }
 }