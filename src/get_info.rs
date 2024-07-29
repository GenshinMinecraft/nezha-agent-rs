use heim_virt::*;
use std::collections::HashMap;
use log::{error, info, warn};
use sysinfo::*;
use systemstat::Platform;

pub async fn get_mem_info(mut sys: System) -> (u64, u64, u64, u64, u64, u64) {
    sys.refresh_memory();
    let total_mem = sys.total_memory();
    let free_mem = sys.free_memory();
    let used_mem = total_mem - free_mem;

    let total_swap = sys.total_swap();
    let free_swap = sys.free_swap();
    let used_swap = total_swap - free_swap;

    (
        total_mem, free_mem, used_mem, total_swap, free_swap, used_swap,
    )
}

pub async fn get_platform_info() -> (String, String) {
    let dist = whoami::distro();
    let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
    (dist, kernel_version)
}

pub async fn get_cpu_info() -> (Vec<String>, String, String) {
    let mut sys = System::new();
    sys.refresh_cpu();
    let mut all_cpu_name = Vec::new();
    for cpu in sys.cpus() {
        all_cpu_name.push(cpu.brand().to_string());
    }

    let mut cpu_counts: HashMap<String, usize> = HashMap::new();
    for cpu in all_cpu_name {
        *cpu_counts.entry(cpu).or_insert(0) += 1;
    }

    let arch = whoami::arch().to_string();
    let virt = detect().await.unwrap_or_else(|| Virtualization::Unknown);

    let frequency = match cpuinfo::frequence::frequency() {
        Ok(freq) => match freq.max {
            Some(freq) => format!("{:.2}GHz", freq.0 as f64 / 1000000000000f64),
            None => "0.0GHz".to_string(),
        },
        Err(_) => "0.0GHz".to_string(),
    };

    let mut cpu_name: Vec<String> = Vec::new();
    if virt.is_vm() || virt.is_container() {
        for (cpu, count) in cpu_counts {
            cpu_name.push(format!("{} @ {} {} Virtual Core", cpu, frequency, count))
        }
    } else {
        for (cpu, count) in cpu_counts {
            cpu_name.push(format!("{} @ {} {} Physical Core", cpu, frequency, count))
        }
    }

    (cpu_name, arch, virt.as_str().to_string())
}

pub async fn get_disk_info() -> (u64, u64, u64) {
    let disk_list = Disks::new_with_refreshed_list();
    let mut all_disk_space: u64 = 0;
    let mut all_disk_available: u64 = 0;
    for disk in &disk_list {
        all_disk_space += disk.total_space();
        all_disk_available += disk.available_space();
    }
    let all_disk_used = all_disk_space - all_disk_available;
    (all_disk_space, all_disk_available, all_disk_used)
}

pub fn get_boot_time() -> u64 {
    let sys = systemstat::System::new();
    match sys.boot_time() {
        Ok(boot_time) => boot_time.unix_timestamp() as u64,
        Err(_) => 0,
    }
}

pub async fn get_ip_info() -> String {
    let client = match reqwest::Client::builder().user_agent("curl/8.0.0").build() {
        Ok(client) => client,
        Err(_) => return String::new(),
    };
    match client.get("http://ip.sb").send().await {
        Ok(resp) => {
            return match resp.text().await {
                Ok(ip) => {
                    info!("成功获取 IP 地址: {}", ip);
                    ip
                },
                Err(e) => {
                    warn!("未能获取 IP 地址, 将不会返回 IP 地址: {}", e);
                    String::new()
                }
            }
        }
        Err(_) => String::new(),
    }
}

pub async fn get_cpu_usage(cpus: &[Cpu]) -> f64 {
    let mut all_usage: f64 = 0.0;
    for cpu in cpus {
        all_usage += cpu.cpu_usage() as f64;
    }
    return all_usage / cpus.len() as f64;
}

static mut TMP_RX: u64 = 0;
static mut TMP_TX: u64 = 0;

pub async fn get_network_info() -> Result<(u64, u64, u64, u64), Box<dyn std::error::Error>> {
    let sys = systemstat::System::new();
    let nic = sys.networks()?;
    let mut nic_names: Vec<String> = Vec::new();
    for (name, _) in nic {
        nic_names.push(name);
    }
    let mut all_rx: u64 = 0;
    let mut all_tx: u64 = 0;
    for nic_name in nic_names {
        all_rx += sys.network_stats(nic_name.as_str())?.rx_bytes.0;
        all_tx += sys.network_stats(nic_name.as_str())?.tx_bytes.0;
    }
    let tmp_tx = unsafe { TMP_TX };
    let tmp_rx = unsafe { TMP_RX };

    unsafe {
        TMP_TX = all_tx;
        TMP_RX = all_rx;
    }
    Ok((all_rx, all_tx, all_rx - tmp_rx, all_tx - tmp_tx))
}

pub async fn get_uptime_info() -> Result<(u64, f64, f64, f64), Box<dyn std::error::Error>> {
    let sys = systemstat::System::new();
    let load_average = sys.load_average()?;
    let (uptime_one, uptime_five, uptime_fifteen) = (
        load_average.one as f64,
        load_average.five as f64,
        load_average.fifteen as f64,
    );

    let uptime = sys.uptime()?.as_secs();
    Ok((uptime, uptime_one, uptime_five, uptime_fifteen))
}
