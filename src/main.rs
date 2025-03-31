#![windows_subsystem = "windows"]

use crate::getinfo::*;
use crate::requests::*;
use tokio::time::{sleep, Duration, Instant};

mod getinfo;
mod requests;


#[tokio::main]
async fn main() {
    let mut last_mongodb_call = Instant::now();
    let mongodb_interval = Duration::from_secs(10);

    tokio::spawn(async {
        getwindows().await;
    });

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
                ram: get_total_ram(),
                monitor: get_monitor().unwrap_or_else(|| "Monitor não encontrado".to_string()),
                snmonitor: get_serialnumbermonitor().unwrap_or_else(|| "Monitor não encontrado".to_string()),
                time: time_now(),
                apiauth: "JolyneTheCat120207.18".to_string(),
                programs: get_programs(),
            };

            get_onlinetime();

            sendinfos(info).await.expect("Erro ao chamar MongoDB");
            last_mongodb_call = Instant::now();
        }
        sleep(Duration::from_secs(5)).await;
    }
}
