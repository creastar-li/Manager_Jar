# Manager_Jar

<div align="center">

![Manager_Jar Logo](https://img.shields.io/badge/Manager_Jar-v0.1.0-blue?style=for-the-badge&logo=rust)

**专业的 Java 应用程序生命周期管理工具**

[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-brightgreen)](https://github.com/CreaStar/Manager_Jar)
[![Language](https://img.shields.io/badge/Language-Rust-orange)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-green)](LICENSE)
[![Build Status](https://img.shields.io/badge/Build-Passing-success)](https://github.com/CreaStar/Manager_Jar)

</div>

---

## 🎯 项目简介

**Manager_Jar** 是一个用 Rust 开发的现代化 JAR 包管理工具，专门为简化 Java 应用程序的部署、监控和维护而设计。它提供了直观的命令行界面，支持批量操作、进程监控、日志管理等功能。

### ✨ 核心特性

🔄 **智能进程管理** - 启动、停止、重启、状态监控  
📦 **批量操作** - 同时管理多个 JAR 应用  
🔗 **序列管理** - 按预定义顺序执行批量任务  
🤖 **守护进程** - 24/7 健康监控和自动重启  
📋 **日志管理** - 集中日志收集、查看、清理  
🎨 **美观界面** - 彩色输出和友好的用户体验  
🚀 **高性能** - 基于 Rust，内存安全且执行高效  
🌐 **跨平台** - 完美支持 Windows、Linux、macOS  

---

## 🚀 快速开始

### 安装

#### 方式一：下载预编译版本
从 [Releases](https://github.com/CreaStar/Manager_Jar/releases) 页面下载适合您操作系统的版本。

#### 方式二：从源码构建
```bash
# 克隆仓库
git clone https://github.com/CreaStar/Manager_Jar.git
cd Manager_Jar

# 构建发布版本
cargo build --release
# 或使用构建脚本
./build.sh release     # Linux/macOS
.\build.ps1 release    # Windows PowerShell
```

### 基础使用

```bash
# 查看版本信息
Manager_Jar version

# 列出可用的 JAR 文件
Manager_Jar list

# 启动 JAR 应用
Manager_Jar start myapp.jar

# 查看应用状态
Manager_Jar status

# 停止应用
Manager_Jar stop myapp.jar

# 生成命令补全脚本
Manager_Jar completions bash > ~/.manager_jar_completions
```

---

## 📖 详细文档

完整的使用说明请查看：

📚 **[详细使用指南](USER_GUIDE.md)** - 包含所有功能的详细说明和示例

主要内容包括：
- 🛠️ [安装指南](USER_GUIDE.md#-安装指南)
- 📖 [基础使用](USER_GUIDE.md#-基础使用)  
- 🔧 [高级功能](USER_GUIDE.md#-高级功能)
- 📦 [批量操作](USER_GUIDE.md#-批量操作)
- 🔗 [序列管理](USER_GUIDE.md#-序列管理)
- 🤖 [守护进程](USER_GUIDE.md#-守护进程)
- 📋 [日志管理](USER_GUIDE.md#-日志管理)
- 🔍 [故障排除](USER_GUIDE.md#-故障排除)

---

## 🎮 使用示例

### 单应用管理
```bash
# 启动应用（带 JVM 参数）
Manager_Jar start myapp.jar -Xmx1g -Dspring.profiles.active=prod

# 快速重启
Manager_Jar quick myapp.jar

# 查看实时日志
Manager_Jar log myapp.jar --follow
```

### 批量操作
```bash
# 批量启动多个服务
Manager_Jar batch start database.jar config-server.jar eureka.jar

# 批量停止所有应用
Manager_Jar batch stop *.jar

# 批量重启
Manager_Jar batch restart service*.jar
```

### 序列管理
```bash
# 创建微服务启动序列
Manager_Jar sequence create microservices database.jar config.jar eureka.jar gateway.jar user-service.jar

# 按序列启动所有服务
Manager_Jar sequence start microservices

# 检查序列状态
Manager_Jar sequence status microservices
```

### 守护进程监控
```bash
# 启动守护进程
Manager_Jar daemon start --background

# 查看监控状态
Manager_Jar daemon status

# 添加应用到自动重启监控
Manager_Jar start myapp.jar --monitor
```

---

## 🏗️ 项目架构

```
Manager_Jar/
├── 🎯 cli/          # 命令行接口
├── ⚙️  core/         # 核心功能（配置、进程、日志）
├── 🔄 operations/   # 操作模块（批量、序列）
├── 🤖 daemon/       # 守护进程
├── 🛠️  utils/        # 工具模块
└── 📄 build.rs      # 构建脚本
```

**模块化设计**：清晰的代码结构，易于维护和扩展  
**Rust 生态**：充分利用 Rust 的性能和安全优势  
**现代工具链**：集成 Clap、Colored、Chrono 等优秀库  

---

## 🛠️ 开发

### 构建项目

```bash
# 开发构建
cargo build

# 发布构建
cargo build --release

# 运行测试
cargo test

# 代码检查
cargo check

# 格式化代码
cargo fmt
```

### 使用构建脚本

```bash
# Linux/macOS
./build.sh help

# Windows PowerShell
.\build.ps1 help

# Windows CMD
build.bat help
```

### 项目依赖

- **clap** - 命令行参数解析
- **colored** - 终端色彩输出
- **chrono** - 时间日期处理
- **serde** + **toml** - 配置文件序列化
- **regex** - 正则表达式
- **dirs** - 跨平台目录处理

---

## 📊 系统要求

### 运行环境
- **Java Runtime Environment (JRE) 8+** - 运行 JAR 包所需
- **操作系统**：Windows 10+、Linux、macOS

### 性能特性
- **内存占用**：< 10MB
- **启动时间**：< 100ms
- **CPU 使用**：接近零开销
- **平台支持**：x86_64、ARM64

---

## 🤝 贡献

我们欢迎各种形式的贡献！

### 如何贡献

1. **Fork** 本仓库
2. 创建您的功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交您的更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开一个 **Pull Request**

### 开发指南

- 遵循 Rust 代码风格规范
- 编写单元测试和文档
- 确保所有测试通过
- 更新相关文档

---

## 📞 支持

### 获取帮助

- 📋 **Issues**: [提交问题](https://github.com/CreaStar/Manager_Jar/issues)
- 💬 **讨论**: [GitHub Discussions](https://github.com/CreaStar/Manager_Jar/discussions)
- 📧 **邮箱**: creastar@gmail.com

### 文档资源

- 📚 [完整使用指南](USER_GUIDE.md)
- 🔧 [开发者文档](docs/DEVELOPMENT.md)
- 📝 [变更日志](CHANGELOG.md)
- ❓ [常见问题](USER_GUIDE.md#-常见问题)

---

## 📄 许可证

本项目采用 **MIT** 许可证。详情请查看 [LICENSE](LICENSE) 文件。

---

## 🎉 致谢

感谢所有为项目做出贡献的开发者和用户！

特别感谢：
- **Rust 社区** - 提供优秀的生态系统
- **开源社区** - 各种优秀的库和工具
- **用户反馈** - 帮助我们不断改进

---

<div align="center">

### 🌟 如果这个项目对您有帮助，请给我们一个 Star！

[![GitHub stars](https://img.shields.io/github/stars/CreaStar/Manager_Jar?style=social)](https://github.com/CreaStar/Manager_Jar/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/CreaStar/Manager_Jar?style=social)](https://github.com/CreaStar/Manager_Jar/network/members)

**[⬆ 回到顶部](#manager_jar)**

</div>
