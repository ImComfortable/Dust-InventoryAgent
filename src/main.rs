//#![windows_subsystem = "windows"]
use crate::getinfo::{get_namepc, get_serialnumber, get_username, get_disks, get_total_ram, get_model, get_processador, get_monitor, get_serialnumbermonitor, get_windows_version, get_onlinetime, get_ip_local,time_now,get_windows};

use crate::requests::{sendinfos};
use tokio::time::{sleep, Duration, Instant};

mod getinfo;
mod requests;

#[tokio::main]
async fn main() {
    let mut last_mongodb_call = Instant::now();

    let mongodb_interval = Duration::from_secs(10);

    loop {
        let now = Instant::now();

        if now.duration_since(last_mongodb_call) >= mongodb_interval {
            get_onlinetime();
            let active = get_windows();
            let serialnumber = get_serialnumber();
            let nomepc = get_namepc();
            let username = get_username();
            let disk = get_disks();
            let ram = get_total_ram();
            let model = get_model();
            let ip = get_ip_local();
            let processador = get_processador();
            let version = get_windows_version();
            let mut monitor = get_monitor();
            let mut smodel = get_serialnumbermonitor();
            let time = time_now();

            if smodel == "" && monitor == "" {
                smodel = "Sem Monitor".to_string();
                monitor = "Sem Monitor".to_string();
            }

            sendinfos(
                serialnumber,
                nomepc.clone(),
                username,
                disk,
                ram,
                model,
                version,
                active,
                ip,
                processador,
                monitor,
                smodel,
                time,
            )
                .await
                .expect("Erro ao chamar MongoDB");

            last_mongodb_call = now;
        }

        sleep(Duration::from_secs(5)).await;
    }
}
