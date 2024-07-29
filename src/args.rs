use clap::Parser;

/// Cloudflare IP Speedtest Backend
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Frontend Server Address
    #[arg(short, long)]
    pub server: String,

    /// Token Setting
    #[arg(short, long)]
    pub password: String,

    /// Enable Debug Log
    #[arg(long, default_value_t = false)]
    pub debug: bool,
}

pub fn init_args() -> Args {
    // 使用Args::parse方法从命令行参数中构建Args对象。
    let args: Args = Args::parse();
    // 返回构建好的Args对象。
    return args;
}
