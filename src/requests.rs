use serde::{Serialize, Deserialize};
use crate::get_username;
use std::io::Write;
use std::fs::{File, OpenOptions};
use std::error::Error;
use std::fmt;
use std::env;

#[derive(Debug)]
pub struct ApiError {
    pub message: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "API Error: {}", self.message)
    }
}

impl Error for ApiError {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Infos {
    pub nome: String,
    pub usuario: String,
    pub servicetag: String,
    pub modelo: String,
    pub versao: String,
    pub windows: String,
    pub ip: String,
    pub disco: String,
    pub processador: String,
    pub graphiccard: String,
    pub ram: String,
    pub monitor: String,
    pub snmonitor: String,
    pub time: String,
    pub apiauth: String,
    pub programs: Vec<String>,
}

#[derive(Serialize, Debug)]
struct Payload {
    user: String,
    page: String,
    date: String,
    seconds: f64,
    apiauth: String,
}

pub async fn sendinfos(info: Infos) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    match client.post("?/dbinfos")
        .json(&info)
        .send()
        .await {
        Ok(_) => Ok(()),
        Err(e) => {
            let error_msg = format!("Erro ao enviar informações: {:?}", e);
            log_error(&error_msg);
            Ok(())
        }
    }
}

pub async fn sendpages(page: String, date: String, seconds: f64) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let user = get_username();

    let payload = Payload { user, page, date, seconds, apiauth: "JolyneTheCat120207.18".to_string() };

    match client.post("?/atualizar-documentos")
        .json(&payload)
        .send()
        .await {
        Ok(_) => Ok(()),
        Err(e) => {
            let error_msg = format!("Erro ao enviar informações de página: {:?}", e);
            log_error(&error_msg);
            Ok(())
        }
    }
}

pub fn log_error(msg: &str) {
    let appdata_path = env::var("APPDATA").unwrap_or_else(|_| {
        eprintln!("Erro ao obter o caminho do AppData. Usando o diretório atual.");
        ".".to_string()
    });

    let log_file_path = format!("{}/agentLogs.txt", appdata_path);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
        .unwrap_or_else(|_| {
            File::create(&log_file_path).expect("Erro ao criar arquivo de log")
        });

    if let Err(e) = writeln!(file, "{}", msg) {
        eprintln!("Erro ao escrever no arquivo de log: {}", e);
    }
}