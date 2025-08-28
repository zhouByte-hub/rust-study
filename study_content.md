### 进程与Shell

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `duct` | 一个用于运行子进程的库。它简化了管道构建和IO重定向，并能处理跨平台的兼容性问题。 |
| `duct_sh` | `duct` 的子 crate，方便通过 shell 字符串构建命令，用于访问高级 shell 功能。 |
| `shells` | `std::process::Command` 的封装，旨在让 Rust 的 shell 脚本编写更具吸引力。 |
| `walkdir` | 一个跨平台的库，用于高效地递归遍历目录，支持符号链接和目录修剪。 |
| `runas` | 一个简单的 Rust 库，可以以 root 身份执行命令。 |

### 命令行

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `Clap` | 一个易于使用、高效且功能齐全的命令行参数解析器。 |
| `colored` | 用于在终端文本中添加颜色和样式的库，是 NodeJS/NPM 中 `colors` 库的移植。 |

### 错误处理

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `thiserror` | 用于创建专用错误类型的库，适用于需要精确错误信息的库代码。 |
| `anyhow` | 基于 trait 对象的错误类型，简化应用程序中的错误处理，适用于不关心具体错误类型的场景。 |
| `errno` | 提供跨平台访问 `errno` 变量的接口。 |

### 序列化与配置文件

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `serde` | 一个通用的序列化/反序列化框架。 |
| `serde_json` | `Serde` 的一个具体实现，用于将对象序列化成 JSON。 |
| `rust-ini` | 一个用于解析 Ini 配置文件的库。 |
| `toml` | 一个原生的 Rust 编码器和解码器，用于处理 TOML 格式。 |
| `config` | 一个分层的配置系统，支持多种文件格式（ini, json, yaml, toml 等），但不支持写回。 |

### 日志

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `log` | 一个轻量级的日志接口，提供统一的日志 API，需与具体实现搭配使用。 |
| `env_logger` | 通过环境变量配置的日志实现，必须与 `log` 一起使用。 |
| `pretty_env_logger` | 基于 `env_logger`，提供漂亮的彩色日志输出。 |
| `flexi_logger` | 一个灵活的日志记录器，支持写入文件、stderr 等多种输出，并可在运行时动态配置。 |

### 文件与目录

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `include_dir` | 用于将整个目录树嵌入到二进制文件中的宏。 |
| `tempfile` | 一个安全、跨平台的临时文件和目录创建库。 |
| `zip` | 一个支持读写简单 ZIP 文件的 Rust 库。 |
| `rust-embed` | 用于将 CSS、JS 和图片等静态资源嵌入到单个可执行文件中。 |

### 网络与Web

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `url` | 一个基于 URL 标准的 URL 解析和处理库。 |
| `http-types` | 提供共享的 HTTP 操作类型，具有高性能的流式接口。 |
| `ipnet` | 用于处理 IPv4 和 IPv6 CIDR 地址的类型和函数。 |
| `netdev` | 提供跨平台的网络接口 API。 |
| `wol-rs` | (描述为空) |
| `stunclient` | 一个简单的 STUN 客户端，用于解析 UDP 套接字的外部 IP 地址和端口。 |
| `paho-mqtt` | Eclipse Paho MQTT 的 Rust 客户端库。 |

### GUI与图形

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `Egui` | 一个简单、快速且高度便携的立即模式 GUI 库。 |
| `qrcode-generator` | 用于生成 RAW、PNG 和 SVG 格式的二维码。 |

### 系统与平台

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `os_info` | (描述为空) |
| `num_cpus` | 用于获取机器上的 CPU 数量。 |
| `sys-locale` | 一个轻量级库，用于获取系统或应用程序当前设置的区域设置。 |
| `system_shutdown` | 提供跨平台的方式来执行关机、重启或注销操作。 |
| `mac_address` | 提供跨平台的方法来获取网络硬件的 MAC 地址。 |

### Windows特定

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `winapi` | 提供对整个 Windows API 的原始 FFI 绑定。 |
| `windows` | 允许调用任何 Windows API 的 crate，代码由元数据动态生成。 |
| `winreg` | 提供对 MS Windows 注册表 API 的 Rust 绑定。 |
| `windows-service` | 提供管理和实现 Windows 服务的工具。 |
| `tauri-winrt-notification` | WinRT Toast API 的不完整包装器。 |

### 音频处理

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `dasp` | 提供处理音频 PCM DSP 的基本功能。 |
| `rubato` | 一个用于音频数据的异步重采样库。 |
| `samplerate` | 基于 `libsamplerate` 的采样率转换库。 |
| `fon` | Rust 音频类型、重采样、处理和混音库。 |

### 并发与异步

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `tokio` | 一个事件驱动的非阻塞 I/O 平台，用于编写异步应用程序。 |
| `crossbeam-queue` | (描述为空) |
| `ringbuf` | 无锁的单生产者单消费者 FIFO 环形缓冲区。 |

### 数据与编码

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `regex` | (描述为空) |
| `wildmatch` | (描述为空) |
| `sha2` | 用纯 Rust 实现的 SHA-2 哈希函数家族。 |
| `bytes` | 一个处理字节的实用库。 |
| `hex` | 用于将数据编码为十六进制表示或从十六进制解码。 |
| `cidr-utils` | 提供了处理 IPv4 CIDR 和 IPv6 CIDR 的函数。 |

### 开发与测试

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `mockall` | 一个强大的 Rust 模拟对象库，用于单元测试。 |
| `rstest` | (描述为空) |
| `once_cell` | (描述为空) |
| `derive_builder` | (描述为空) |
| `lazy_static` | (描述为空) |
| `libloading` | 基于平台动态库加载功能的绑定，具有改进的内存安全性。 |

### 其他

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `chrono` | (描述为空) |
| `winnow` | (描述为空) |
| `handlebars` | (描述为空) |
| `git2-rs` | (描述为空) |
| `sqlx` | (描述为空) |
| `flutter_rust_bridge` | Flutter/Dart 与 Rust 的绑定生成器。 |
| `rdev` | 在 Windows、Linux 和 MacOS 上监听和发送键盘和鼠标事件。 |
| `shutdown_hooks` | Rust 的关机钩子，是 `atexit` 的一个友好包装器。 |
| `totp-rs` | 用于创建双因素认证令牌的库。 |
| `shared_memory` | 一个用户友好的 crate，允许在进程之间共享内存。 |