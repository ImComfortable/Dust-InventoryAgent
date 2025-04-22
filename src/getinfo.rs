use hostname::get as get_hostname;
use winapi::um::winbase::CREATE_NO_WINDOW;
use winapi::um::winuser::{GetWindowTextW, GetWindowTextLengthW, GetForegroundWindow};
use winapi::um::winnt::LPWSTR;
use winapi::um::sysinfoapi::GetTickCount;
use winapi::um::winuser::GetLastInputInfo;
use winapi::um::winuser::LASTINPUTINFO;

use tokio::time::{Duration, Instant};
use std::process::Command;
use std::os::windows::process::CommandExt;
use std::thread;
use regex::Regex;
use chrono::Local;

use crate::requests::sendpages;


pub fn get_serialnumber() -> String {
    let servicetag = Command::new("powershell")
        .arg("/C")
        .arg("Get-Ciminstance Win32_Bios | Format-List Serialnumber")
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    match servicetag {
        Ok(servicetag) => {
            let snoutput = String::from_utf8_lossy(&servicetag.stdout);
            let snoutput_clened = snoutput.replace("Serialnumber :", "");
            snoutput_clened.trim().to_string()
        }
        Err(_) => "Error ao coletar a service tag".to_string()
    }
}
pub fn get_serialnumbermonitor() -> Option<String> {
    let monitor_serial = Command::new("powershell")
        .arg("-Command")
        .arg(r#"Get-WmiObject -Namespace root\wmi -Class WmiMonitorID | ForEach-Object {
    $serialNumber = if ($_.SerialNumberID) {
        [System.Text.Encoding]::ASCII.GetString($_.SerialNumberID).Trim([char]0)
    } else {
        ''
    }
    if ($serialNumber -and $serialNumber -ne '0') {
        $serialNumber
    }}"#)
        .creation_flags(CREATE_NO_WINDOW)
        .output();
    match monitor_serial {
        Ok(monitorserial) => {
            let outputpw = String::from_utf8_lossy(&monitorserial.stdout);
            Some(outputpw.trim().to_string())
        }
        Err(_) => Some("Error ao coletar o serial number do monitor".to_string())
    }
}
pub fn get_monitor() -> Option<String> {
    let output = Command::new("powershell")
        .args(&[
            "-Command",
            "Get-WmiObject WmiMonitorID -Namespace root\\wmi | ForEach-Object { [System.Text.Encoding]::ASCII.GetString($_.UserFriendlyName) }"
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output();
    match output {
        Ok(output) => {
            let outputstring = String::from_utf8_lossy(&output.stdout);
            let cleaned_output: String = outputstring
                .chars()
                .filter(|c| !c.is_control())
                .collect();
            let final_output = cleaned_output.trim().to_string();
            Some(final_output)
        }
        Err(_) => Some("Error ao coletar o monitor".to_string())
    }
}
pub fn get_processador() -> String {
    let processador = Command::new("powershell")
        .arg("/C")
        .arg("(Get-WmiObject Win32_Processor).Name")
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    match processador {
        Ok(processador) => {
            let output = String::from_utf8_lossy(&processador.stdout);
            output.trim().to_string()
        }
        Err(_) => "Error ao coletar o processador".to_string()
    }
}
pub fn get_namepc() -> String {
    match get_hostname() {
        Ok(hostname) => hostname.to_string_lossy().to_string(),
        Err(_) => "Falha ao pegar o nome do usuario".to_string()
    }
}
pub fn get_model() -> String {
    let model = Command::new("powershell")
        .arg("/C")
        .arg("(Get-CimInstance -ClassName Win32_ComputerSystem).Model")
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    match model {
        Ok(model) => {
            let output = String::from_utf8_lossy(&model.stdout);
            output.trim().to_string()
        }
        Err(_) => "Error ao coletar o modelo do dispositivo".to_string()
    }
}
pub fn get_username() -> String {
    let username = Command::new("powershell")
        .arg("$env:USERNAME")
        .creation_flags(CREATE_NO_WINDOW)
        .output();
    match username {
        Ok(username) => {
            let usernameout = String::from_utf8_lossy(&username.stdout);
            let username_clean = usernameout.trim();

            username_clean.to_string()
        }
        Err(_) => "Error to catch the username".to_string()
    }
}
pub fn get_disks() -> String {
    let disk = Command::new("powershell")
        .arg("/C")
        .arg("Get-PhysicalDisk | Select-Object MediaType, Size")
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    match disk {
        Ok(output) => {
            let disktout = String::from_utf8_lossy(&output.stdout);
            let mut result = String::new();

            for line in disktout.lines() {
                if line.contains("SSD") || line.contains("HDD") || line.contains("Unspecified") {
                    let media_type = if line.contains("SSD") {
                        "SSD"
                    } else if line.contains("HDD") {
                        "HDD"
                    } else {
                        "Pendrive"
                    };
                    let size = extract_size_from_line(line);
                    result.push_str(&format!("{} {}GB\n", media_type, size));
                }
            }

            if result.is_empty() {
                "Nenhum disco encontrado".to_string()
            } else {
                result.trim().to_string()
            }
        }
        Err(_) => "Erro ao executar o comando".to_string(),
    }
}
fn extract_size_from_line(line: &str) -> String {
    let size_regex = Regex::new(r"\d+").unwrap();
    if let Some(captures) = size_regex.captures(line) {
        if let Some(size) = captures.get(0) {
            let size_gb = size.as_str().parse::<u64>().unwrap_or(0) / (1024 * 1024 * 1024);
            return size_gb.to_string();
        }
    }
    "Desconhecido".to_string()
}
pub fn get_total_ram() -> String {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("(Get-CimInstance Win32_PhysicalMemory | Measure-Object -Property Capacity -Sum).Sum")
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let output_string = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let mhz = get_ram_speed();

                if let Ok(total_bytes) = output_string.parse::<u64>() {
                    let total_gb = total_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                    match mhz {
                        Some(speed) => format!("{:.2} GB {}MHz", total_gb, speed),
                        None => format!("{:.2} GB", total_gb)
                    }
                } else {
                    "Erro ao processar a memória total".to_string()
                }
            } else {
                "Erro ao executar o comando PowerShell".to_string()
            }
        }
        Err(_) => "Erro ao executar o comando".to_string(),
    }
}
pub fn get_ram_speed() -> Option<u32> {
    let output = Command::new("powershell")
        .args(&[
            "Get-CimInstance",
            "Win32_PhysicalMemory",
            "|",
            "Select-Object",
            "-First",
            "1",
            "Speed"
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .ok()?;

    let output_str = String::from_utf8(output.stdout).ok()?;
    let re = Regex::new(r"\d+").ok()?;

    re.find(&output_str)?.as_str().parse().ok()
}
pub fn get_onlinetime() {
    let time = Command::new("powershell")
        .arg("/C")
        .arg("$uptime = (Get-CimInstance -ClassName Win32_OperatingSystem).LastBootUpTime
    $days = (New-TimeSpan -Start $uptime -End (Get-Date)).Days
    $days")
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    match time {
        Ok(time) => {
            let output = String::from_utf8_lossy(&time.stdout);
            let outputclean = output.trim();

            match outputclean.parse::<u32>() {
                Ok(days) => {
                    if days > 3 {
                        Command::new("Cmd")
                            .arg("/c")
                            .arg("msg * Por conta do excesso de dias ligado, o computador precisa reiniciar. Por favor, reinicie-o agora ou ele reiniciará sozinho em 30 minutos.")
                            .output()
                            .expect("Falha ao enviar a mensagem");

                        Command::new("Cmd")
                            .arg("/c")
                            .arg("shutdown /r /t 1800")
                            .output()
                            .expect("Falha ao programar o desligamento");

                        thread::sleep(Duration::from_secs(1500));

                        Command::new("Cmd")
                            .arg("/c")
                            .arg("msg * O computador será reiniciado em 5 minutos, como avisado a 25 minutos atrás.")
                            .output()
                            .expect("Falha ao enviar a mensagem de aviso");
                    }
                },
                Err(_) => {
                    println!("Erro ao converter o tempo para número");
                },
            }
        },
        Err(e) => {
            println!("Erro ao executar o comando: {}", e);
        }
    }
}
pub fn get_windows_version() -> String {
    let output = Command::new("powershell")
        .arg("/C")
        .arg(format!("(Get-WmiObject -Class Win32_OperatingSystem).Caption + ' ' + (Get-WmiObject -Class Win32_OperatingSystem).Version"))
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    match output {
        Ok(output) => {
            let ver_output = String::from_utf8_lossy(&output.stdout);
            let version_info = ver_output.trim();
            if !version_info.is_empty() {
                return version_info.to_string();
            }
            "Versão do Windows não encontrada".to_string()
        }
        Err(_) => "Erro ao executar o comando".to_string(),
    }
}
pub fn get_ip_local() -> String {
    let ip = Command::new("Powershell")
        .arg("/C")
        .arg(r#"(ipconfig | findstr IPv4 | select -First 1).Split()[-1]"#)
        .creation_flags(CREATE_NO_WINDOW)
        .output();
        
    match ip {
        Ok(ip) => {
            let ipoutput = String::from_utf8_lossy(&ip.stdout);
            ipoutput.trim().to_string()
        }
        Err(_) => "Error ao coletar o ip local".to_string()
    }    
}
pub fn graphic_card() -> String {
    let gpu = Command::new("powershell")
        .arg("/C")
        .arg("(Get-WmiObject Win32_VideoController).Name")
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    match gpu {
        Ok(gpu) => {
            let output = String::from_utf8_lossy(&gpu.stdout);
            output.trim().to_string()
        }
        Err(_) => "Error ao coletar a placa de video".to_string()
    }
}
pub fn time_now() -> String {
    let agora = Local::now();
    agora.format("%d-%m-%Y às %H horas.").to_string() 
}
pub fn get_windows() -> String {
    let model = Command::new("powershell")
    .arg(r"Get-CimInstance SoftwareLicensingProduct -Filter 'Name like ''Windows%'' ' | where { $_.PartialProductKey } | select LicenseStatus")    
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    match model {
        Ok(model) => {
            let output = String::from_utf8_lossy(&model.stdout);
            let saida = output.trim().to_string();
            if saida.contains("1") {
                "Windows Ativo".to_string()
            } else {
                "Precisa ativar o windows".to_string()
            }
        }
        Err(_) => "Error ao coletar o status do windows do dispositivo".to_string()
    }
}
pub fn get_programs() -> Vec<String> {
    let output = Command::new("powershell")
        .arg("-Command")
        .arg("Get-ItemProperty HKLM:\\Software\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\*, HKLM:\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\* | Where-Object { $_.DisplayName -ne $null -and $_.SystemComponent -ne 1 -and $_.DisplayName -notmatch 'Microsoft' -and $_.DisplayName -notmatch 'Windows' } | Select-Object DisplayName, DisplayVersion | Sort-Object DisplayName | ForEach-Object { \"{0} ({1})\" -f $_.DisplayName, $_.DisplayVersion }")
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .expect("Erro ao executar o comando");

    let output_str = String::from_utf8(output.stdout).expect("Erro ao converter a saída para string");
    output_str.lines().map(|line| line.to_string()).collect()
}
fn get_last_input_time() -> u64 {
    unsafe {
        let mut last_input = LASTINPUTINFO {
            cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
            dwTime: 0,
        };
        if GetLastInputInfo(&mut last_input) != 0 {
            let current_tick = GetTickCount();
            return(current_tick - last_input.dwTime) as u64;
        }

        0
    }
}
fn is_inactive(last_active_time: &Instant, threshold: Duration) -> bool {
    last_active_time.elapsed() > threshold
}
pub async fn getwindows(getpassword: &str) {
    let getpassword = getpassword.to_string();
    let mut last_window: Option<String> = None;
    let mut start_time = Instant::now();
    let mut last_active_time = Instant::now();
    let inactive_threashold = Duration::from_secs(60 * 10);

    loop {
        let current_input_time = get_last_input_time();
        let system_indle_time = Duration::from_millis(current_input_time as u64);

        if system_indle_time > inactive_threashold {
            if !is_inactive(&last_active_time, inactive_threashold) {
                if let Some(_last) = last_window.clone() {
                    if let Err(e) = send_to_mongo(&format!("Inativo"), last_active_time.elapsed(), &getpassword).await {
                        eprintln!("Erro ao atualizar o resumo do MongoDB: {}", e);
                    }
                }
            }
        } else {
            if is_inactive(&last_active_time, inactive_threashold) {
                start_time = Instant::now();
            }
            last_active_time = Instant::now();
        }

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

            let mut title_text = String::from_utf16_lossy(&title[..length as usize]).trim().to_string();
            
            if title_text.contains("Firefox") || title_text.contains("Google Chrome") || 
               title_text.contains("Microsoft Edge") || title_text.contains("Brave") {
                
                let browsers = vec!["Mozilla ", "Chrome ", "Microsoft ", "Brave"];
                for browser in browsers.iter() {
                    title_text = title_text.replace(browser, "").trim().to_string();
                }
                Some(title_text)
            } else {
                Some(title_text)
            }
        }).await.unwrap_or(None);

        if let Some(title) = current_title {
            if last_window.as_ref() != Some(&title) {
                if let Some(last) = last_window.clone() {
                    let elapsed = start_time.elapsed();
                    if let Err(e) = send_to_mongo(&last, elapsed, &getpassword).await {
                        eprintln!("Erro ao atualizar o resumo do MongoDB: {}", e);
                    }
                }
                start_time = Instant::now();
                last_window = Some(title.clone());
            }
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
async fn send_to_mongo(window_title: &str, duration: Duration, password: &String) -> Result<(), Box<dyn std::error::Error>> {
    let current_date = Local::now().format("%d-%m-%Y").to_string();
    let seconds = duration.as_secs_f64();

    let page = window_title.trim().to_string();
    let date = current_date.trim().to_string();
    let seconds = seconds;

    sendpages(page, date, seconds, password).await?;
    

    Ok(())
}