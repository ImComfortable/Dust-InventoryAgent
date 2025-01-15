use hostname::get as get_hostname;
use winapi::um::winbase::CREATE_NO_WINDOW;
use tokio::time::{Duration};
use std::process::{Command};
use std::os::windows::process::CommandExt;
use std::thread;
use regex::Regex;
use chrono::Local;

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
pub fn get_serialnumbermonitor() -> String {
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
            outputpw.trim().to_string()
        }
        Err(_) => "Error ao coletar o serial number do monitor".to_string()
    }
}
pub fn get_monitor() -> String {
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
            final_output
        }
        Err(_) => "Error ao coletar o monitor".to_string()
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
    let username = Command::new("cmd")
        .arg("/C")
        .arg("whoami")
        .creation_flags(CREATE_NO_WINDOW)
        .output();
    match username {
        Ok(username) => {
            let usernameout = String::from_utf8_lossy(&username.stdout);
            let usernameoutclean = usernameout.rsplit('\\').next();
            usernameoutclean.unwrap_or("").trim().to_string()
        }
        Err(_) => "Error to catch the username".to_string()
    }
}
pub fn get_disks() -> String {

    let disk = Command::new("powershell")
        .arg("/C")
        .arg("Get-PhysicalDisk | Select-Object MediaType")
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    match disk {
        Ok(output) => {
            let disktout = String::from_utf8_lossy(&output.stdout);
            for line in disktout.lines() {
                if line.contains("SSD") {
                    let space = get_disk_storage();
                    return format!("SSD {}GB", space); // Usa format! para incluir a variável na string
                } else if line.contains("HDD") {
                    let space = get_disk_storage();
                    return format!("HDD {}GB", space); // Mesmo aqui, formatando a string
                }
            }
            "Desconhecido".to_string()
        }
        Err(_) => "Erro ao executar o comando".to_string(),
    }
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
                            .arg("shutdown /s /t 1800")
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
        .arg(format!("(Get-ComputerInfo).WindowsProductName + ' ' + (Get-ComputerInfo).WindowsCurrentVersion"))
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    match output {
        Ok(output) => {
            let ver_output = String::from_utf8_lossy(&output.stdout);

            // Verifique se a saída contém a versão do Windows e a build
            let version_info = ver_output.trim();
            if !version_info.is_empty() {
                return version_info.to_string();
            }
            "Versão do Windows não encontrada".to_string()
        }
        Err(_) => "Erro ao executar o comando".to_string(),
    }
}
pub fn get_disk_storage() -> String {
    let servicetag = Command::new("powershell")
        .arg("/C")
        .arg("Get-WmiObject -Class Win32_DiskDrive | Select-Object Size")
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    match servicetag {
        Ok(servicetag) => {
            let snoutput = String::from_utf8_lossy(&servicetag.stdout);
            let snoutput_result = snoutput.lines().nth(3).unwrap_or("Linha nao encontrada");
            let snoutput_result_limited = snoutput_result.chars().take(3).collect::<String>();
            snoutput_result_limited.trim().to_string()
        }
        Err(_) => "Error ao coletar o espaço do disco".to_string()
    }
}
pub fn get_ip_local() -> String {
    let ip = Command::new("Powershell")
        .arg("/C")
        .arg(r#"(Get-NetIPAddress -InterfaceAlias "Ethernet" | Where-Object { $_.AddressFamily -eq "IPv4" }).IPAddress"#)
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
pub fn time_now() -> String {
    let agora = Local::now();
    agora.format("%Y-%m-%d %H:%M:%S").to_string() 
}