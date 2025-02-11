use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Infos {
    pub nome: String,
    pub usuario : String,
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

    match client.post("UrlToServerRestFull")
        .json(&info)
        .send()
        .await {
        Ok(_) => Ok(()),
        Err(_) => Ok(())
    }
 }
