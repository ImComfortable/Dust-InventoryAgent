#![windows_subsystem = "windows"]


//hasharquivo B1E7964C5CE9F9E8C6031F4A0097ADCC079C70C75746A0CF8A6E4CBE6FB2F56A

use crate::getinfo::*;
use crate::requests::*;
use tokio::time::{sleep, Duration, Instant};

mod getinfo;
mod requests;

#[tokio::main]
async fn main() {
    let mut last_mongodb_call = Instant::now();
    let mongodb_interval = Duration::from_secs(300);

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
                monitor: get_monitor().expect("Sem monitor"),
                snmonitor: get_serialnumbermonitor().expect("Sem monitor"),
                time: time_now(),
                passwordpost: "PasswordToPost".to_string(),
            };

            get_onlinetime();

            sendinfos(info).await.expect("Erro ao chamar MongoDB");
            last_mongodb_call = Instant::now();
        }
        sleep(Duration::from_secs(5)).await;
    }
}
