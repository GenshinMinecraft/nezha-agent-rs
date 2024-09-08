use std::process::exit;

mod args;
mod get_info;
mod proto;
mod server_comm;

use args::*;
use log::*;
use server_comm::*;
use simple_logger::init_with_level;
use sysinfo::System;

#[tokio::main]
async fn main() {
    let args: Args = init_args();

    if args.debug {
        init_with_level(Level::Debug).unwrap();
    } else {
        init_with_level(Level::Info).unwrap();
    }

    let mut client = if args.tls {
        match init_tls_client(args.server.as_str()).await {
            Ok(tmp) => {
                info!("成功创立与服务器的连接");
                tmp
            }
            Err(e) => {
                error!("无法连接服务器: {}", e);
                exit(1);
            }
        }
    } else {
        match init_client(args.server.as_str()).await {
            Ok(tmp) => {
                info!("成功创立与服务器的连接");
                tmp
            }
            Err(e) => {
                error!("无法连接服务器: {}", e);
                exit(1);
            }
        }
    };

    let mut sys = System::new();

    loop {
        match build_request_host(args.password.as_str()).await {
            Ok(request) => {
                debug!("Host 请求: {:?}", request);
                match client.report_system_info(request).await {
                    Ok(response) => {
                        debug!("Host 回应: {:?}", response);
                        info!("成功发送本机基本信息");
                    }
                    Err(response) => {
                        debug!("Host 回应: {:?}", response);
                        if response.message() == "客户端认证失败" {
                            error!("连接密钥不正确！");
                            exit(1);
                        } else {
                            error!("无法发送本机基本信息");
                        }
                    }
                }
            }
            Err(_) => {}
        }

        loop {
            sys.refresh_cpu();
            match build_request_state(args.password.as_str(), &sys).await {
                Ok(request) => {
                    debug!("State 请求: {:?}", request);
                    match client.report_system_state(request).await {
                        Ok(response) => {
                            debug!("State 回应: {:?}", response);
                            info!("成功发送本机状态信息");
                        }
                        Err(e) => {
                            error!("无法发送本机状态信息: {}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("无法获取本机状态信息: {}", e);
                }
            };
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}
