# nezha-agent-rs

由 Rust 编写的 **轻量级、高性能、低占用** 哪吒面板 Agent

目前仅实现了主页面板的监控功能，如 Task、终端等尚未适配 (或者不会适配，保持轻量级别)

使用方法与官方 Nezha Agent 无异，请使用 `--help` 查看帮助信息:
```
Nezha Agent

Usage: nezha-agent-rs [OPTIONS] --server <SERVER> --password <PASSWORD>

Options:
  -s, --server <SERVER>      Frontend Server Address
  -p, --password <PASSWORD>  Token Setting
      --debug                Enable Debug Log
  -h, --help                 Print help
  -V, --version              Print version
```

基础使用仅需 `./nezha-agent-rs -s [服务器地址:端口] -p [连接密钥]`

请前往本项目 Action 获取 Release 构建文件，不会存放至 Release 页面，请自行下载