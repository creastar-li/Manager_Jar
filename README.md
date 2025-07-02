# Manager_Jar

<div align="center">

**语言版本**: [🇨🇳 中文](README.md) | [🇺🇸 English](README_EN.md)

![许可证](https://img.shields.io/badge/许可证-MIT-blue.svg)
![平台](https://img.shields.io/badge/平台-Windows-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.87+-orange.svg)
![版本](https://img.shields.io/badge/版本-0.1.0-green.svg)

**专业的 Java 应用程序生命周期管理工具**

一个功能强大、模块化设计的 JAR 包管理器，支持启动、停止、重启、监控和批量操作 Java 应用程序。

[功能特性](#功能特性) • [安装指南](#安装指南) • [快速开始](#快速开始) • [使用文档](#使用文档) • [参与贡献](#参与贡献)

</div>

---

## 功能特性

### 🚀 核心功能
- **进程管理**: 启动、停止、重启 JAR 应用程序
- **状态监控**: 实时查看 JAR 包运行状态和资源监控  
- **日志管理**: 集中化日志查看和管理
- **配置管理**: 灵活的参数配置和预设管理

### 🔥 高级功能
- **批量操作**: 同时管理多个 JAR 应用程序
- **序列管理**: 按预定义顺序启动/停止应用程序组
- **守护进程**: 系统级守护进程，自动重启和监控
- **安全防护**: 进程安全管理和资源保护

### 💻 开发者友好
- **现代化 CLI**: 直观的命令行界面和彩色输出
- **命令补全**: 支持 Bash、Zsh、PowerShell 自动补全
- **完整文档**: 完整的使用指南和 API 文档
- **模块化设计**: 清晰的代码结构，易于扩展和维护

## 安装指南

### 系统要求
- Windows 操作系统
- Java 运行环境 (JRE 8+)
- 一个或多个 JAR 文件

### 方式一: 从源码构建 (推荐)

```powershell
# 克隆仓库
git clone https://github.com/YourUsername/Manager_Jar.git
cd Manager_Jar

# 构建发布版本
cargo build --release

# 可选: 将可执行文件添加到 PATH
copy .\target\release\Manager_Jar.exe C:\Windows\System32\
```

### 方式二: 使用构建脚本

```powershell
# PowerShell 用户
.\build.ps1

# 或者使用 CMD
.\build.bat
```

### 方式三: 开发环境

```powershell
# 安装 Rust (如果未安装)
# 访问 https://rustup.rs/

# 克隆并进入项目目录
git clone https://github.com/YourUsername/Manager_Jar.git
cd Manager_Jar

# 开发模式运行
cargo run -- --help
```

## 快速开始

### 基础操作

```powershell
# 启动 JAR 应用程序
Manager_Jar.exe start myapp.jar

# 带参数启动
Manager_Jar.exe start myapp.jar -Xmx1g -server

# 查看状态
Manager_Jar.exe status myapp.jar

# 停止应用程序
Manager_Jar.exe stop myapp.jar

# 重启应用程序
Manager_Jar.exe restart myapp.jar
```

### 配置管理

```powershell
# 保存配置
Manager_Jar.exe config myapp.jar --args "-Xmx2g -server" --name "生产环境"

# 使用保存的配置快速启动
Manager_Jar.exe quick myapp.jar --config "生产环境"

# 列出所有配置
Manager_Jar.exe configs list
```

### 批量操作

```powershell
# 批量启动多个应用程序
Manager_Jar.exe batch start app1.jar app2.jar app3.jar

# 批量停止
Manager_Jar.exe batch stop app1.jar app2.jar

# 批量状态检查
Manager_Jar.exe batch status app1.jar app2.jar
```

### 序列管理

```powershell
# 创建应用程序序列
Manager_Jar.exe sequence create webapps app1.jar app2.jar app3.jar

# 按序列启动 (app1 -> app2 -> app3)
Manager_Jar.exe sequence start webapps

# 按序列停止 (app3 -> app2 -> app1)
Manager_Jar.exe sequence stop webapps
```

### 日志管理

```powershell
# 查看实时日志
Manager_Jar.exe log myapp.jar --follow

# 查看最近 100 行日志
Manager_Jar.exe log myapp.jar --lines 100

# 查看错误日志
Manager_Jar.exe logs error myapp.jar
```

### 守护进程

```powershell
# 启动系统守护进程
Manager_Jar.exe daemon start

# 添加应用程序到监控并启用自动重启
Manager_Jar.exe daemon add myapp.jar --auto-restart

# 查看守护进程状态
Manager_Jar.exe daemon status
```

## 使用示例

### 示例一: 微服务管理
```powershell
# 创建微服务启动序列
Manager_Jar.exe sequence create microservices config-server.jar eureka-server.jar gateway.jar user-service.jar

# 按序列启动所有服务
Manager_Jar.exe sequence start microservices

# 启动守护进程监控
Manager_Jar.exe daemon start

# 添加关键服务到监控
Manager_Jar.exe daemon add config-server.jar --auto-restart
Manager_Jar.exe daemon add eureka-server.jar --auto-restart
```

### 示例二: 开发环境
```powershell
# 保存调试配置
Manager_Jar.exe config myapp.jar --args "-Xmx1g -Xdebug -Xrunjdwp:transport=dt_socket,server=y,suspend=n,address=5005" --name "调试模式"

# 使用调试模式快速启动
Manager_Jar.exe quick myapp.jar --config "调试模式"

# 查看实时日志
Manager_Jar.exe log myapp.jar --follow
```

### 示例三: 生产环境部署
```powershell
# 保存生产环境配置
Manager_Jar.exe config webapp.jar --args "-Xmx4g -Xms2g -server -XX:+UseG1GC" --name "生产环境"

# 启动应用程序
Manager_Jar.exe quick webapp.jar --config "生产环境"

# 后台启动守护进程
Manager_Jar.exe daemon start --background

# 添加到监控并启用自动重启
Manager_Jar.exe daemon add webapp.jar --auto-restart
```

## 项目结构

```
Manager_Jar/
├── src/
│   ├── main.rs              # 主程序入口
│   ├── build_info.rs        # 构建信息 (自动生成)
│   ├── cli/                 # 命令行界面
│   │   ├── commands.rs      # CLI 命令定义
│   │   └── completions.rs   # 命令补全
│   ├── core/                # 核心功能
│   │   ├── process.rs       # 进程管理
│   │   ├── config.rs        # 配置管理
│   │   └── logging.rs       # 日志系统
│   ├── operations/          # 操作模块
│   │   ├── batch.rs         # 批量操作
│   │   └── sequence.rs      # 序列管理
│   ├── daemon/              # 守护进程
│   │   ├── thread_daemon.rs # 线程守护
│   │   └── system_daemon.rs # 系统守护
│   └── utils/               # 工具模块
│       ├── macros.rs        # 宏定义
│       ├── files.rs         # 文件操作
│       └── display.rs       # 显示工具
├── build.rs                 # 构建脚本
├── Cargo.toml              # 项目配置
├── build.ps1               # PowerShell 构建脚本
├── build.sh                # Bash 构建脚本
├── build.bat               # CMD 构建脚本
├── README.md               # 项目文档
└── USAGE_GUIDE.md          # 详细使用指南
```

## 配置

### 全局配置

```powershell
# 设置全局配置
Manager_Jar.exe global-config set log_level info
Manager_Jar.exe global-config set max_processes 10
Manager_Jar.exe global-config set default_jvm_args "-Xmx1g"

# 查看配置
Manager_Jar.exe global-config show

# 重置配置
Manager_Jar.exe global-config reset
```

### 配置文件位置

- **Windows**: `%APPDATA%\Manager_Jar\`
  - 全局配置: `config\global.toml`
  - JAR 配置: `config\jars\*.toml`
  - 日志文件: `logs\*.log`

### 配置示例

```toml
# global.toml
[settings]
log_level = "info"
max_processes = 10
default_jvm_args = "-Xmx1g -server"
auto_restart = true

[daemon]
check_interval = 30
restart_delay = 5
max_restart_attempts = 3
```

## 使用文档

- 📖 **[详细使用指南](USAGE_GUIDE.md)** - 完整的命令参考和示例
- 🏗️ **[项目结构](#项目结构)** - 架构和模块组织
- ⚙️ **[配置](#配置)** - 配置管理指南
- 🔧 **[开发](#开发)** - 开发设置和贡献指南

## 开发

### 开发环境设置

```powershell
# 1. 安装 Rust
# 访问 https://rustup.rs/ 进行安装

# 2. 克隆项目
git clone https://github.com/YourUsername/Manager_Jar.git
cd Manager_Jar

# 3. 安装依赖并运行测试
cargo test

# 4. 开发模式运行
cargo run -- --help
```

### 可用的开发任务

```powershell
# 构建项目
cargo build

# 运行测试
cargo test

# 检查代码
cargo check

# 格式化代码
cargo fmt

# 清理构建文件
cargo clean

# 构建发布版本
cargo build --release
```

### VS Code 任务

项目包含预配置的 VS Code 任务:

- `Build Manager_Jar` - 构建项目
- `Build Manager_Jar Release` - 构建发布版本
- `Run Tests` - 运行测试
- `Format Code` - 格式化代码
- `Check Code` - 检查代码
- `Run Manager_Jar` - 运行程序

## 命令参考

### 可用命令

```
Manager_Jar.exe [COMMAND]

命令:
  completions    生成命令行补全脚本
  start          启动指定JAR包
  stop           停止指定JAR包
  restart        重启指定JAR包
  status         查看JAR包状态
  list           列出所有可用JAR文件
  log            查看JAR日志
  kill           强制杀死JAR进程
  config         配置JAR参数
  quick          用已保存配置快速启动JAR
  batch          批量操作
  sequence       序列化批量管理JAR组
  logs           日志管理
  configs        配置管理
  global-config  全局配置管理
  daemon         系统级守护进程管理
  version        显示版本信息
  help           显示帮助信息或给定子命令的帮助
```

### 命令补全

```powershell
# 生成 PowerShell 补全
Manager_Jar.exe completions powershell > Manager_Jar_completions.ps1

# 生成 Bash 补全
Manager_Jar.exe completions bash > Manager_Jar_completions.bash

# 生成 Zsh 补全
Manager_Jar.exe completions zsh > Manager_Jar_completions.zsh
```

## 系统要求

### 运行环境
- **Java 运行环境 (JRE) 8+** - 运行 JAR 包所需
- **操作系统**: Windows 10+

### 性能特性
- **内存使用**: < 10MB
- **启动时间**: < 100ms
- **CPU 使用**: 接近零开销
- **平台支持**: x86_64, ARM64

## 参与贡献

欢迎各种形式的贡献！

### 如何贡献

1. **报告 Bug**: 在 [Issues](../../issues) 中描述问题
2. **功能请求**: 在 [Issues](../../issues) 中提出新功能想法
3. **代码贡献**: Fork 项目并提交 Pull Request
4. **文档**: 帮助改进文档和示例

### 贡献指南

1. Fork 本仓库
2. 创建您的功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交您的更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

### 开发标准

- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 为新功能添加测试
- 更新相关文档

## 变更日志

### v0.1.0 (2025-07-02)

#### 🎉 首次发布
- ✨ 完整的 JAR 包管理功能
- 🏗️ 模块化架构设计
- 🎨 现代化 CLI 界面
- 📝 完整的文档和用户指南

#### 🚀 核心功能
- JAR 包启动、停止、重启
- 进程状态监控
- 日志管理系统
- 配置管理

#### 🔥 高级功能
- 批量操作支持
- 序列管理
- 系统守护进程
- 命令行补全

#### 🛠️ 技术特性
- Rust 1.87+ 支持
- 自动构建信息注入
- 跨平台构建脚本
- 零编译警告

## 获取支持

### 获取帮助

- 📚 查看 [USAGE_GUIDE.md](USAGE_GUIDE.md) 获取详细使用指南
- 💬 在 [Issues](../../issues) 中提问
- 📧 联系维护者: creastar@gmail.com

### 文档资源

- 📚 [完整使用指南](USAGE_GUIDE.md)
- 🔧 [开发文档](docs/DEVELOPMENT.md)
- 📝 [变更日志](CHANGELOG.md)
- ❓ [常见问题](USAGE_GUIDE.md#故障排除)

## 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

---

<div align="center">

**⭐ 如果这个项目对您有帮助，请给它一个星标！**

[![GitHub stars](https://img.shields.io/github/stars/YourUsername/Manager_Jar?style=social)](../../stargazers)
[![GitHub forks](https://img.shields.io/github/forks/YourUsername/Manager_Jar?style=social)](../../network/members)

Made with ❤️ by [CreaStar](https://github.com/CreaStar)

**[⬆ 回到顶部](#manager_jar)**

</div>
