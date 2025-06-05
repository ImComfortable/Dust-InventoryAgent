//#![windows_subsystem = "windows"]

use crate::collect_data::*;
use crate::make_requests::*;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tauri::{Manager};
use tokio::time::{sleep, Duration};
use winapi::um::winuser::{GetWindowTextW, GetWindowTextLengthW, GetForegroundWindow};
use winapi::um::winnt::LPWSTR;

mod collect_data;
mod make_requests;

struct AppState {
    inactive_duration: Arc<Mutex<Option<Duration>>>,
    last_window: Arc<Mutex<Option<String>>>,
    start_time: Arc<Mutex<Instant>>,
    last_mongodb_call: Arc<Mutex<Instant>>,
    last_depart_call: Arc<Mutex<Instant>>,
    last_depart_time: Arc<Mutex<Duration>>,
    inactivity_window_created: Arc<Mutex<bool>>,
    inactivity_window_shown_time: Arc<Mutex<Option<Instant>>>,
}

#[tauri::command]
async fn register_inactivity(justificativa: String, app_handle: tauri::AppHandle, state: tauri::State<'_, Arc<AppState>>) -> Result<(), String> {
    //println!("register_inactivity FOI CHAMADO com justificativa: {}", justificativa);
    let duration = {
        let mut dur = state.inactive_duration.lock().unwrap();
        dur.take()
    };

    if let Some(dur) = duration {

        if let Err(e) = send_to_mongo(&justificativa, dur, &get_password()).await {
            eprintln!("Erro ao enviar as informações capturadas pelo front erro: {}", e);
        }
        
        if let Some(win) = app_handle.get_window("inactivity") {
            let _ = win.hide();
            
            let mut window_shown_time = state.inactivity_window_shown_time.lock().unwrap();
            *window_shown_time = None;
        }
    }

    Ok(())
}

#[tauri::command]
fn close_inactivity_window(app_handle: tauri::AppHandle, state: tauri::State<'_, Arc<AppState>>) {
    if let Some(win) = app_handle.get_window("inactivity") {
        let _ = win.hide();
        
        let mut window_shown_time = state.inactivity_window_shown_time.lock().unwrap();
        *window_shown_time = None;
    }
}

async fn monitor_inactivity_window(app_handle: tauri::AppHandle, state: Arc<AppState>) {
    let window_timeout = Duration::from_secs(60*5); // tempo da inatividade da janela
    
    loop {
        let should_close = {
            let window_shown_time = state.inactivity_window_shown_time.lock().unwrap();
            if let Some(time) = *window_shown_time {
                time.elapsed() >= window_timeout
            } else {
                false
            }
        };
        
        if should_close {
            
            if let Some(win) = app_handle.get_window("inactivity") {
                let _ = win.hide();
            }
            {
                let mut window_shown_time = state.inactivity_window_shown_time.lock().unwrap();
                *window_shown_time = None;
            }
            
            if let Err(e) = send_to_mongo(
                "O usuario se recusou a fornecer o motivo.", 
                window_timeout, 
                &get_password()
            ).await {
                eprintln!("Erro ao enviar justificativa automática para o MongoDB: {}", e);
            }
        }
        
        sleep(Duration::from_secs(10)).await;
    }
}

async fn get_time_url(state: Arc<AppState>) -> Duration {
    let url_interval = Duration::from_secs(60 * 30); // 30 minutos
    
    let should_send = {
        let last_call = state.last_depart_call.lock().unwrap();
        Instant::now().duration_since(*last_call) >= url_interval
    };
    
    if should_send {
        let new_value = Duration::from_secs(get_depart_time().await);
        println!("{:?}", new_value);
        
        let mut last_call = state.last_depart_call.lock().unwrap();
        *last_call = Instant::now();
        
        let mut last_time = state.last_depart_time.lock().unwrap();
        *last_time = new_value;
        
        new_value
    } else {
        let last_time = state.last_depart_time.lock().unwrap();
        *last_time
    }
}

async fn monitor_inactivity(app_handle: tauri::AppHandle, state: Arc<AppState>) {
    let mut was_inactive = false;
    let mut inactive_start = Instant::now();

    loop {
        let inactive_threshold = get_time_url(state.clone()).await;
        let idle = Duration::from_millis(get_last_input_time() as u64);
        //println!("Idle time: {} ms", idle.as_millis());

        if idle > inactive_threshold {
            if !was_inactive {
                was_inactive = true;
                inactive_start = Instant::now();
                println!("Usuário inativo detectado");
            }
        } else {
            if was_inactive {
                was_inactive = false;
                let duration = inactive_start.elapsed();

                if duration.as_secs() > 0 {
                    println!("Inatividade encerrada após {} segundos", duration.as_secs());

                    let total_inactivity = inactive_threshold + duration;

                    {
                        let mut dur = state.inactive_duration.lock().unwrap();
                        *dur = Some(total_inactivity);
                    }

                    let window_exists = app_handle.get_window("inactivity").is_some();

                    if !window_exists {
                        match tauri::WindowBuilder::new(
                            &app_handle,
                            "inactivity",
                            tauri::WindowUrl::App("index.html".into())
                        )
                        .title("Registro de Inatividade")
                        .center()
                        .always_on_top(true)
                        .max_inner_size(400.0, 600.0)
                        .min_inner_size(400.0, 600.0)
                        .visible(true)
                        .closable(false)
                        .skip_taskbar(false)
                        .build() {
                            Ok(_) => {
                                println!("Janela de inatividade criada com sucesso");
                                
                                let mut window_shown_time = state.inactivity_window_shown_time.lock().unwrap();
                                *window_shown_time = Some(Instant::now());
                                
                                let mut created = state.inactivity_window_created.lock().unwrap();
                                *created = true;
                            },
                            Err(e) => eprintln!("Erro ao criar janela: {}", e),
                        }
                    } else if let Some(win) = app_handle.get_window("inactivity") {
                        let _ = win.show();
                        let _ = win.set_focus();
                        
                        let mut window_shown_time = state.inactivity_window_shown_time.lock().unwrap();
                        *window_shown_time = Some(Instant::now());
                    }
                }
            }
        }

        sleep(Duration::from_secs(1)).await;
    }
}

async fn monitor_window_activity(state: Arc<AppState>) {
    loop {
        let current_title = tokio::task::spawn_blocking(|| {
            let hwnd = unsafe { GetForegroundWindow() };
            let length = unsafe { GetWindowTextLengthW(hwnd) };

            if length == 0 {
                return None;
            }

            let mut title: Vec<u16> = vec![0; (length + 1) as usize];
            unsafe {
                GetWindowTextW(hwnd, title.as_mut_ptr() as LPWSTR, length + 1);
            }

            let title_text = String::from_utf16_lossy(&title[..length as usize]).trim().to_string();

            if title_text.contains("Firefox") || title_text.contains("Google Chrome") ||
               title_text.contains("Microsoft Edge") || title_text.contains("Brave") {
                let browsers = vec!["Mozilla ", "Chrome ", "Microsoft ", "Brave"];
                let cleaned_title = browsers.iter().fold(title_text.clone(), |acc, browser| {
                    acc.replace(browser, "").trim().to_string()
                });
                Some(cleaned_title)
            } else {
                Some(title_text)
            }
        }).await.unwrap_or(None);

        if let Some(title) = current_title {
            let (should_send, window_title, elapsed) = {
                let mut last_window = state.last_window.lock().unwrap();
                let mut start_time = state.start_time.lock().unwrap();
                
                let should_send = last_window.is_some() && last_window.as_ref() != Some(&title);
                let window_title = last_window.clone();
                let elapsed = start_time.elapsed();
                
                if last_window.as_ref() != Some(&title) {
                    *start_time = Instant::now();
                    *last_window = Some(title);
                }
                
                (should_send, window_title, elapsed)
            };
            
            if should_send {
                if let Some(last) = window_title {
                    if let Err(e) = send_to_mongo(&last, elapsed, &get_password()).await {
                        eprintln!("Erro ao atualizar o resumo do MongoDB: {}", e);
                    }
                }
            }
        }

        sleep(Duration::from_secs(1)).await;
    }
}

async fn periodic_system_info(state: Arc<AppState>) {
    let mongodb_interval = Duration::from_secs(60);

    loop {
        let should_send = {
            let last_call = state.last_mongodb_call.lock().unwrap();
            Instant::now().duration_since(*last_call) >= mongodb_interval
        };
        
        if should_send {
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
                ram: get_ram_details(),
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
            
            {
                let mut last_call = state.last_mongodb_call.lock().unwrap();
                *last_call = Instant::now();
            }
        }
        
        sleep(Duration::from_secs(5)).await;
    }
}

#[tokio::main]
async fn main() {
    let context = tauri::generate_context!();

    let app_state = Arc::new(AppState {
        inactive_duration: Arc::new(Mutex::new(None)),
        last_window: Arc::new(Mutex::new(None)),
        start_time: Arc::new(Mutex::new(Instant::now())),
        last_mongodb_call: Arc::new(Mutex::new(Instant::now())),
        last_depart_call: Arc::new(Mutex::new(Instant::now())),
        last_depart_time: Arc::new(Mutex::new(Duration::from_secs(20*60))),
        inactivity_window_created: Arc::new(Mutex::new(false)),
        inactivity_window_shown_time: Arc::new(Mutex::new(None)),
    });

    tauri::Builder::default()
        .manage(app_state.clone())
        .invoke_handler(tauri::generate_handler![
            register_inactivity, 
            close_inactivity_window
        ])
        .setup( move |app| {
            let app_handle = app.handle();
            let state_clone = app_state.clone();
            
            tauri::async_runtime::spawn(monitor_inactivity(app_handle.clone(), state_clone.clone()));
            tauri::async_runtime::spawn(monitor_window_activity(state_clone.clone()));
            tauri::async_runtime::spawn(periodic_system_info(state_clone.clone()));
            tauri::async_runtime::spawn(monitor_inactivity_window(app_handle.clone(), state_clone));
            
            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}