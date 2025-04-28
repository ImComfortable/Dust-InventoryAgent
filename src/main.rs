//#![windows_subsystem = "windows"]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::getinfo::*;
use crate::requests::*;
use std::error::Error;
use slint::SharedString;
use tokio::time::{sleep, Duration, Instant};

mod getinfo;
mod requests;

slint::include_modules!();

#[tokio::main]
async fn main() {

    tokio::spawn(async {
        getwindows().await;
    });

    let mut last_mongodb_call = Instant::now();
    let mongodb_interval = Duration::from_secs(60*30);

    spawn_audit().await.unwrap_or_else(|e| {
        let error_msg = format!("Erro ao iniciar a interface: {:?}", e);
        log_error(&error_msg);
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

async fn spawn_audit() -> Result<(timespent: String, password: String), Box<dyn std::error::Error>> {
    let ui = LoginPage::new()?;

    let ui_weak = ui.as_weak();
    ui.on_send_report(move |text: SharedString| {
        let text_str = text.as_str();
        
        if text_str.is_empty() {
            println!("Campo de texto vazio!");
        } else {
            send_to_mongo("Inativo - {}", text_str.to_string(), timespent).await.unwrap_or_else(|e| {
                let error_msg = format!("Erro ao enviar informações: {:?}", e);
                log_error(&error_msg);
            });
            if let Some(ui) = ui_weak.upgrade() {
                ui.hide().unwrap();
            }
        }
    });

    let ui_weak_close = ui.as_weak();
    ui.window().on_close_requested(move || {
        send_to_mongo("Inativo - Se recusou a esclarecer o motivo").await.unwrap_or_else(|e| {
            let error_msg = format!("Erro ao enviar informações: {:?}", e);
            log_error(&error_msg);
        });
        if let Some(ui) = ui_weak_close.upgrade() {
            ui.hide().unwrap();
        }
        slint::CloseRequestResponse::HideWindow
    });

    ui.run()?;

    Ok(())
}
