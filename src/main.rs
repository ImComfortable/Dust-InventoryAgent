//#![windows_subsystem = "windows"]

use crate::collect_data::*;
use crate::make_requests::*;
use tokio::time::{Duration, Instant};

mod collect_data;
mod make_requests;


#[tokio::main]
async fn main() {
    tokio::spawn(async {
        collect_data::monitor_inactivity().await;
    });

    let mut last_mongodb_call = Instant::now();
    let mongodb_interval = Duration::from_secs(60 * 30);

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
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}