use std::time::Duration;

use log::error;
use sysinfo::System;
use tonic::{transport::{Channel, Endpoint}, Request};

use crate::{
    get_info::*,
    proto::{nezha_service_client::NezhaServiceClient, Host, State},
};

pub async fn init_client(
    server_url: &str,
) -> Result<NezhaServiceClient<Channel>, Box<dyn std::error::Error>> {
    let endpoint = match Endpoint::from_shared("http://".to_string() + &server_url) {
        Ok(tmp) => tmp,
        Err(e) => {
            error!("无法解析服务器地址: {}", e);
            return Err(Box::new(tonic::Status::aborted("无法解析服务器地址")));
        }
    };

    let client = NezhaServiceClient::connect(
        endpoint
            .timeout(Duration::from_secs(5))
            .connect_timeout(Duration::from_secs(5))
            .tcp_keepalive(Some(Duration::from_secs(5)))
            .http2_keep_alive_interval(Duration::from_secs(5))
            .keep_alive_timeout(Duration::from_secs(5))
            .keep_alive_while_idle(true),
    )
    .await?;

    return Ok(client);
}

pub async fn build_request_host(token: &str) -> Result<Request<Host>, Box<dyn std::error::Error>> {
    let sys = System::new();
    let (total_mem, _, _, total_swap, _, _) = get_mem_info(sys).await;
    let (dist, kernel_version) = get_platform_info().await;
    let (cpu_name, arch, virt) = get_cpu_info().await;
    let (all_disk_space, _, _) = get_disk_info().await;
    let ip = get_ip_info().await;
    let mut request = Request::new(Host {
        platform: dist,
        platform_version: kernel_version,
        cpu: cpu_name,
        mem_total: total_mem,
        disk_total: all_disk_space,
        swap_total: total_swap,
        arch,
        virtualization: virt,
        boot_time: get_boot_time(),
        ip,
        country_code: "Dropped".to_string(), // 已经弃用
        version: env!("CARGO_PKG_VERSION").to_string(),
        gpu: vec![], // 没写
    });
    request
        .metadata_mut()
        .insert("client_secret", token.parse()?);

    Ok(request)
}

pub async fn build_request_state(
    token: &str,
    sys: &System,
) -> Result<Request<State>, Box<dyn std::error::Error>> {
    let (_, _, mem_used, _, _, swap_used) = get_mem_info(System::new()).await;
    let (_, _, disk_used) = get_disk_info().await;
    let (all_rx, all_tx, speed_rx, speed_tx) = get_network_info().await?;
    let (uptime, uptime_one, uptime_five, uptime_fifteen) = get_uptime_info().await?;
    let mut request = Request::new(State {
        cpu: get_cpu_usage(sys.cpus()).await,
        mem_used,
        swap_used,
        disk_used,
        net_in_transfer: all_rx,
        net_out_transfer: all_tx,
        net_in_speed: speed_rx,
        net_out_speed: speed_tx,
        uptime,
        load1: uptime_one,
        load5: uptime_five,
        load15: uptime_fifteen,
        tcp_conn_count: 0, // 没写
        udp_conn_count: 0, // 没写
        process_count: 0, // 没写
        temperatures: vec![], // 没写
        gpu: 0.0, // 没写
    });
    request
        .metadata_mut()
        .insert("client_secret", token.parse()?);

    Ok(request)
}
