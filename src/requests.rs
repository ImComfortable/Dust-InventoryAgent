use serde::{Serialize, Deserialize};
use crate::get_username;

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
   // pub passwordpost: String,
    pub programs: Vec<String>,
}

#[derive(Serialize)]
struct Payload {
    user: String,
    page: String,
    date: String,
    seconds: f64,
}

pub async fn sendinfos(info: Infos) -> Result<(), ()> {
    let client = reqwest::Client::new();

    match client.post("http://192.168.22.80:3000/dbinfos")
        .json(&info)
        .send()
        .await {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("{:?}", e);
            Err(())
        }
    }
 }

 pub async fn sendpages(page: String, date: String, seconds: f64) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let user = get_username();

    let payload = Payload { user, page, date, seconds };

    match client.post("http://192.168.22.80:3000/atualizar-documentos")
        .json(&payload)
        .send()
        .await {
        Ok(_) => Ok(()),
        Err(_) => Ok(())
    }
}