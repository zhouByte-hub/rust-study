# Rust 学习项目

这是一个综合性的 Rust 学习项目，涵盖了 Rust 生态系统中的各种常用库和最佳实践。项目通过实际的代码示例展示了 Rust 在不同领域的应用。

## 项目结构

```
rust-study/
├── src/                    # 主要源代码目录
│   ├── database/          # 数据库相关操作
│   ├── error/            # 错误处理示例
│   ├── file/             # 文件和目录操作
│   ├── logs/             # 日志系统
│   ├── other/            # 其他实用工具
│   ├── serialize/        # 序列化和配置文件
│   ├── shell/            # 命令行和进程管理
│   ├── system/           # 系统相关操作
│   ├── web/              # Web 开发相关
│   └── windows/          # Windows 特定功能
├── rust_basic/           # Rust 基础学习
├── rust_async/           # 异步编程学习
├── rust_net/             # 网络编程学习
├── rust_actix/           # Actix Web 框架学习
├── rust_wgpu/            # WGPU 图形编程学习
├── rust_macro/           # Rust 宏编程学习
├── rust_mcp/             # 多线程/进程通信学习
├── tauri-test/           # Tauri 桌面应用开发学习
├── study_content.md      # 学习内容文档
└── Cargo.toml           # 项目配置文件
```

## 功能模块

### 数据库操作 (database/)

- **MySQL**: 使用 sea-orm 进行 MySQL 数据库操作，包括增删改查、条件查询、分组统计等
- **PostgreSQL**: PostgreSQL 数据库操作示例
- **Redis**: Redis 缓存操作，包括字符串、哈希、列表等数据结构的使用

### 错误处理 (error/)

- **thiserror**: 用于创建专用错误类型的库，适用于需要精确错误信息的库代码
- **anyhow**: 基于 trait 对象的错误类型，简化应用程序中的错误处理

### 文件与目录操作 (file/)

- **include_dir**: 用于将整个目录树嵌入到二进制文件中的宏
- **tempfile**: 安全、跨平台的临时文件和目录创建库
- **walkdir**: 高效地递归遍历目录，支持符号链接和目录修剪
- **zip**: 支持 ZIP 文件的读写操作
- **tar**: TAR 文件压缩和解压缩

### 日志系统 (logs/)

- **log**: 轻量级的日志接口，提供统一的日志 API
- **env_logger**: 通过环境变量配置的日志实现
- **pretty_env_logger**: 基于 env_logger，提供漂亮的彩色日志输出
- **flexi_logger**: 灵活的日志记录器，支持写入文件、stderr 等多种输出
- **simple-log**: 简单易用的日志库

### 序列化与配置文件 (serialize/)

- **serde**: 通用的序列化/反序列化框架
- **serde_json**: JSON 格式的序列化和反序列化
- **toml**: TOML 格式的配置文件处理
- **rust-ini**: INI 格式的配置文件解析
- **config**: 分层的配置系统，支持多种文件格式

### 命令行与进程管理 (shell/)

- **clap**: 易于使用、高效且功能齐全的命令行参数解析器
- **colored**: 在终端文本中添加颜色和样式
- **duct**: 用于运行子进程的库，简化管道构建和IO重定向
- **duct_sh**: duct 的子 crate，方便通过 shell 字符串构建命令

### 系统操作 (system/)

- **os_info**: 获取操作系统信息
- **num_cpus**: 获取机器上的 CPU 数量
- **sys-locale**: 获取系统或应用程序当前设置的区域设置
- **system_shutdown**: 跨平台执行关机、重启或注销操作
- **mac_address**: 获取网络硬件的 MAC 地址
- **bytes**: 处理字节的实用库

### Web 开发 (web/)

- **lettre**: 功能强大、异步友好的邮件发送库，支持 SMTP 协议
- **jsonwebtoken**: JSON Web Token (JWT) 库，支持多种签名算法
- **qrcode-generator**: 生成 QR 码矩阵和图像，支持 PNG 和 SVG 格式
- **handlebars**: 强大的模板引擎库，实现了 Handlebars.js 的大部分功能，支持字符串模板和文件模板
- **crossbeam-queue**: 无锁队列实现
- **ringbuf**: 无锁的单生产者单消费者 FIFO 环形缓冲区
- **mime_guess**: 根据文件扩展名猜测 MIME 类型

### 其他实用工具 (other/)

- **regex**: 强大的正则表达式库，支持复杂的字符串匹配和操作
- **wildmatch**: 简单的通配符匹配库
- **hex**: 数据与十六进制表示之间的编码和解码
- **sha2**: 纯 Rust 实现的 SHA-2 哈希函数家族
- **once_cell**: 单次初始化的单元格
- **derive_builder**: 通过派生宏构建构建器模式
- **winnow**: Rust 解析器组合器库，用于构建高效、灵活的解析器，支持文本和二进制数据解析

### Windows 特定功能 (windows/)

- **winapi**: 对整个 Windows API 的原始 FFI 绑定
- **windows**: 允许调用任何 Windows API 的 crate

### 图形编程 (rust_wgpu/)

- **winit**: 跨平台窗口管理库，用于创建窗口和处理事件
- **wgpu**: 现代化的跨平台图形 API 抽象层，支持 Vulkan、Metal、D3D12 等

### 宏编程 (rust_macro/)

- **声明式宏**: 使用 macro_rules! 定义的宏示例
- **派生宏**: 自定义派生宏实现，如 getter 自动生成

### 多线程/进程通信 (rust_mcp/)

- **客户端-服务器架构**: 客户端和服务器通信模式示例

### Tauri 桌面应用 (tauri-test/)

- **WebView 集成**: 使用 Tauri 构建轻量级桌面应用
- **前端交互**: 与 Vue.js 等前端框架的集成示例
- **跨平台**: 支持 Windows、macOS 和 Linux 的桌面应用开发

## 依赖库详解

### 进程与Shell

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `duct` | 一个用于运行子进程的库。它简化了管道构建和IO重定向，并能处理跨平台的兼容性问题。 |
| `duct_sh` | `duct` 的子 crate，方便通过 shell 字符串构建命令，用于访问高级 shell 功能。 |
| `walkdir` | 一个跨平台的库，用于高效地递归遍历目录，支持符号链接和目录修剪。 |

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

### 序列化与配置文件

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `serde` | 一个通用的序列化/反序列化框架。 |
| `serde_json` | `Serde` 的一个具体实现，用于将对象序列化成 JSON。 |
| `rust-ini` | 一个用于解析 Ini 配置文件的库。 |
| `toml` | 一个原生的 Rust 编码器和解码器，用于处理 TOML 格式。 |
| `config` | 一个分层的配置系统，支持多种文件格式（ini, json, yaml, toml, ron, json5 等），提供统一的配置读取接口，但不支持写回。 |

### 日志

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `log` | 一个轻量级的日志接口，提供统一的日志 API，需与具体实现搭配使用。 |
| `env_logger` | 通过环境变量配置的日志实现，必须与 `log` 一起使用。 |
| `pretty_env_logger` | 基于 `env_logger`，提供漂亮的彩色日志输出。 |
| `flexi_logger` | 一个灵活的日志记录器，支持写入文件、stderr 等多种输出，并可在运行时动态配置。 |
| `simple-log` | 一个简单易用的日志库，提供基本的日志记录功能。 |

### 文件与目录

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `include_dir` | 用于将整个目录树嵌入到二进制文件中的宏。 |
| `tempfile` | 一个安全、跨平台的临时文件和目录创建库。 |
| `zip` | 一个支持读写简单 ZIP 文件的 Rust 库。 |
| `tar` | 一个用于处理 TAR 文件格式的 Rust 库。 |
| `rust-embed` | 用于将静态资源（如 HTML、CSS、JS、图片等）嵌入到单个可执行文件中，支持目录结构和文件访问，适合构建独立的 Web 应用程序。 |

### 网络与Web

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `lettre` | Rust 生态中一个功能强大、异步友好的邮件发送库。它允许你的 Rust 程序通过多种方式（最常见的是 SMTP）发送电子邮件。 |
| `jsonwebtoken` | Rust 生态中一个非常流行且成熟的 JSON Web Token (JWT) 库。它允许你在 Rust 程序中方便地创建、验证和解析 JWT。 |
| `qrcode-generator` | 在 RAW、PNG 和 SVG 格式中生成 QR 码矩阵和图像的 Rust 库。 |
| `mime_guess` | 根据文件扩展名猜测 MIME 类型的库。 |
| `handlebars` | Rust 的模板引擎库，实现了 Handlebars.js 的大部分功能，支持模板定义、数据填充和文本输出生成。 |

### 系统与平台

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `os_info` | 用于获取操作系统信息的 Rust 库。 |
| `num_cpus` | 用于获取机器上的 CPU 数量的库。 |
| `sys-locale` | 一个轻量级库，用于获取系统或应用程序当前设置的区域设置。 |
| `system_shutdown` | 提供跨平台的方式来执行关机、重启或注销操作。 |
| `mac_address` | 提供跨平台的方法来获取网络硬件的 MAC 地址。 |
| `bytes` | 一个处理字节的实用库。 |

### 数据库

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `sea-orm` | 一个关系型 ORM 框架，支持 MySQL、PostgreSQL、SQLite 等多种数据库。 |
| `redis` | Redis 客户端库，支持异步操作和连接池。 |

### 并发与异步

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `tokio` | 一个事件驱动的非阻塞 I/O 平台，用于编写异步应用程序。 |
| `tokio-stream` | 为 Tokio 异步运行时提供 Stream trait 的实现，支持异步流处理。 |
| `crossbeam-queue` | 无锁队列实现，提供高性能的并发操作。 |
| `ringbuf` | 无锁的单生产者单消费者 FIFO 环形缓冲区。 |

### 数据与编码

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `regex` | Rust 编程语言中一个非常流行的用于处理正则表达式的库。它提供了强大的正则表达式功能，允许开发人员在 Rust 中进行复杂的字符串匹配和操作。 |
| `wildmatch` | 简单的通配符匹配库，支持类似 shell 的通配符模式。 |
| `sha2` | 用纯 Rust 实现的 SHA-2 哈希函数家族。 |
| `hex` | 用于将数据编码为十六进制表示或从十六进制解码。 |
| `winnow` | Rust 解析器组合器库，提供零成本抽象，支持声明式和命令式解析风格，适用于解析字符串、二进制数据等各种场景。 |

### 开发与测试

| 库 (Library) | 描述 (Description) |
| :--- | :--- |
| `mockall` | 一个强大的 Rust 模拟对象库，用于单元测试。 |
| `httpmock` | 用于 HTTP 服务的模拟和测试库，可以创建模拟 HTTP 服务器来测试客户端代码。 |
| `once_cell` | 提供单次初始化的单元格类型，用于延迟初始化。 |
| `derive_builder` | 通过派生宏自动生成构建器模式的代码。 |

## 使用方法

### 运行测试

项目中的每个模块都包含了相应的测试用例，可以通过以下命令运行：

```bash
# 运行所有测试
cargo test

# 运行特定模块的测试
cargo test database
cargo test web::email
```

### 查看示例代码

每个模块的源代码都包含了详细的使用示例和注释，可以直接查看对应的 `.rs` 文件来学习如何使用各个库。

## 项目特点

1. **全面性**: 涵盖了 Rust 生态系统中的主要库和工具
2. **实用性**: 每个模块都包含实际可运行的代码示例
3. **结构化**: 清晰的模块划分，便于学习和参考
4. **文档完善**: 详细的代码注释和使用说明
5. **异步支持**: 大部分示例都支持异步编程模式
