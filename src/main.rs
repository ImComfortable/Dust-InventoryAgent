#![windows_subsystem = "windows"]

use crate::getinfo::*;
use crate::requests::*;
use tokio::time::{sleep, Duration, Instant};
use std::fs::File;
use std::io::Read;
use serde_json::Value;


mod getinfo;
mod requests;

#[tokio::main]
async fn main() {

    tokio::spawn(async {
        getwindows(&get_password()).await;
    });

    let mut last_mongodb_call = Instant::now();
    let mongodb_interval = Duration::from_secs(60*30);
    
    loop {
        if Instant::now().duration_since(last_mongodb_call) >= mongodb_interval {
            let info = Infos {
                nome: get_namepc(),
                usuario: get_username(),
                servicetag: get_serialnumber(),
                modelo: get_model(),
                versao: get_windows_version(),
                windows: get_windows(),
                ip: get_ip_local(),
                disco: get_disks(),
                processador: get_processador(),
                graphiccard: graphic_card(),
                ram: get_total_ram(),
                monitor: get_monitor().unwrap_or_else(|| "Monitor não encontrado".to_string()),
                snmonitor: get_serialnumbermonitor().unwrap_or_else(|| "Monitor não encontrado".to_string()),
                time: time_now(),
                apiauth: get_password(),
                programs: get_programs(),
            };

            get_onlinetime();

            if let Err(e) = sendinfos(info).await {
                let error_msg = format!("Erro ao enviar informações: {:?}", e);
                log_error(&error_msg);
            }
            last_mongodb_call = Instant::now();
        }
        sleep(Duration::from_secs(5)).await;
    }
}

fn get_password() -> String {
    let mut file = File::open("config.json").expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    let json: Value = serde_json::from_str(&contents).expect("Unable to parse JSON");
    let password = json["password"].as_str().unwrap_or("default_password").to_string();
    password
}