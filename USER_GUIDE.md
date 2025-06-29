# Manager_Jar 使用说明文档

<div align="center">

![Manager_Jar Logo](https://img.shields.io/badge/Manager_Jar-v0.1.0-blue?style=for-the-badge&logo=rust)

**专业的 Java 应用程序生命周期管理工具**

![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-brightgreen)
![Language](https://img.shields.io/badge/Language-Rust-orange)
![License](https://img.shields.io/badge/License-MIT-green)

</div>

---

## 📑 目录

- [项目简介](#-项目简介)
- [核心特性](#-核心特性)
- [快速开始](#-快速开始)
- [安装指南](#-安装指南)
- [基础使用](#-基础使用)
- [高级功能](#-高级功能)
- [配置管理](#-配置管理)
- [批量操作](#-批量操作)
- [序列管理](#-序列管理)
- [守护进程](#-守护进程)
- [日志管理](#-日志管理)
- [构建脚本](#-构建脚本)
- [命令补全](#-命令补全)
- [故障排除](#-故障排除)
- [常见问题](#-常见问题)
- [开发指南](#-开发指南)

---

## 🎯 项目简介

**Manager_Jar** 是一个用 Rust 开发的现代化 JAR 包管理工具，专门为 Java 应用程序的生命周期管理而设计。它提供了直观的命令行界面，支持批量操作、进程监控、日志管理等功能，让 Java 应用的部署和维护变得更加简单高效。

### 🏗️ 设计理念

- **模块化架构**：清晰的代码结构，易于维护和扩展
- **跨平台支持**：完美运行在 Windows、Linux 和 macOS 上
- **用户友好**：直观的命令行界面和彩色输出
- **高性能**：基于 Rust 构建，内存安全且执行高效
- **功能丰富**：涵盖 JAR 包管理的各个方面

---

## ✨ 核心特性

### 🔄 进程管理
- **智能启动**：自动检测 JAR 文件并启动
- **优雅停止**：安全终止进程，保护数据完整性
- **强制终止**：处理无响应的应用程序
- **快速重启**：一键重启功能
- **状态监控**：实时查看应用运行状态

### 📦 批量操作
- **批量启动**：同时启动多个 JAR 应用
- **批量停止**：批量关闭指定的应用
- **批量重启**：批量重启操作
- **进度显示**：实时显示批量操作进度
- **错误处理**：智能处理批量操作中的异常

### 🔗 序列管理
- **序列创建**：创建预定义的 JAR 启动序列
- **序列执行**：按顺序执行多个 JAR 应用
- **序列管理**：查看、修改、删除序列
- **状态检查**：检查序列中所有应用的状态

### 🤖 守护进程
- **后台监控**：持续监控应用程序健康状态
- **自动重启**：检测到异常时自动重启应用
- **日志轮转**：自动管理和清理日志文件
- **资源监控**：监控应用程序资源使用情况

### 📋 日志管理
- **日志收集**：集中收集应用程序日志
- **日志查看**：实时查看应用程序输出
- **日志清理**：定期清理过期日志文件
- **日志分类**：按应用程序分类管理日志

### 🎨 用户体验
- **彩色输出**：美观的终端界面
- **命令补全**：支持 Bash、Zsh、PowerShell 补全
- **详细帮助**：完整的命令行帮助信息
- **错误提示**：友好的错误信息和建议

---

## 🚀 快速开始

### 前置要求

- **Java Runtime Environment (JRE) 8+**：运行 JAR 包所需
- **操作系统**：Windows 10+、Linux、macOS

### 5分钟快速体验

1. **下载并运行**
   ```bash
   # 下载发布版本
   ./Manager_Jar.exe version  # Windows
   ./Manager_Jar version      # Linux/macOS
   ```

2. **查看可用的 JAR 文件**
   ```bash
   Manager_Jar list
   ```

3. **启动第一个 JAR 应用**
   ```bash
   Manager_Jar start myapp.jar
   ```

4. **查看应用状态**
   ```bash
   Manager_Jar status
   ```

5. **停止应用**
   ```bash
   Manager_Jar stop myapp.jar
   ```

---

## 💾 安装指南

### 方式一：下载预编译版本

1. **从 Releases 页面下载**适合您操作系统的版本
2. **解压文件**到您希望安装的目录
3. **添加到 PATH**（可选，但推荐）

#### Windows 安装
```powershell
# 下载后解压到 C:\Tools\Manager_Jar\
# 添加到系统 PATH 环境变量
$env:PATH += ";C:\Tools\Manager_Jar"
```

#### Linux/macOS 安装
```bash
# 解压到 /usr/local/bin/ 或 ~/bin/
sudo mv Manager_Jar /usr/local/bin/
chmod +x /usr/local/bin/Manager_Jar
```

### 方式二：从源码构建

#### 安装 Rust
```bash
# 安装 Rust 工具链
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

#### 克隆并构建
```bash
# 克隆仓库
git clone https://github.com/your-repo/Manager_Jar.git
cd Manager_Jar

# 构建发布版本
cargo build --release

# 可执行文件位于 target/release/Manager_Jar
```

#### 使用构建脚本
```bash
# Windows (PowerShell)
.\build.ps1 release

# Linux/macOS
./build.sh release

# Windows (CMD)
build.bat release
```

---

## 📖 基础使用

### 命令语法

```bash
Manager_Jar <COMMAND> [OPTIONS] [ARGS]
```

### 核心命令概览

| 命令 | 功能 | 示例 |
|------|------|------|
| `start` | 启动 JAR 应用 | `Manager_Jar start app.jar` |
| `stop` | 停止 JAR 应用 | `Manager_Jar stop app.jar` |
| `restart` | 重启 JAR 应用 | `Manager_Jar restart app.jar` |
| `quick` | 快速启动/重启 | `Manager_Jar quick app.jar` |
| `kill` | 强制终止应用 | `Manager_Jar kill app.jar` |
| `status` | 查看应用状态 | `Manager_Jar status` |
| `list` | 列出可用 JAR 文件 | `Manager_Jar list` |
| `log` | 查看应用日志 | `Manager_Jar log app.jar` |

### 应用程序启动

#### 基础启动
```bash
# 启动单个 JAR 文件
Manager_Jar start myapp.jar

# 带 Java 参数启动
Manager_Jar start myapp.jar -Xmx1g -Dspring.profiles.active=prod

# 启动并指定配置文件
Manager_Jar start myapp.jar --spring.config.location=/path/to/config.yml
```

#### 快速操作
```bash
# 快速启动（如果已运行则重启）
Manager_Jar quick myapp.jar

# 带参数的快速启动
Manager_Jar quick myapp.jar -Xmx2g -server
```

### 应用程序停止

#### 优雅停止
```bash
# 发送 SIGTERM 信号，允许应用程序正常关闭
Manager_Jar stop myapp.jar
```

#### 强制终止
```bash
# 强制杀死进程（慎用）
Manager_Jar kill myapp.jar
```

### 状态查看

#### 查看所有应用状态
```bash
Manager_Jar status
```

输出示例：
```
📊 JAR 应用状态:
  📦 myapp: 运行中 (PID: 12345)
  📦 backend: 已停止
  📦 frontend: 运行中 (PID: 12346)

📈 统计信息:
   运行中: 2 | 已停止: 1 | 总计: 3
```

#### 查看特定应用状态
```bash
Manager_Jar status myapp.jar
```

### 日志管理

#### 查看应用日志
```bash
# 查看最新日志
Manager_Jar log myapp.jar

# 跟踪日志（类似 tail -f）
Manager_Jar log myapp.jar --follow

# 查看指定行数的日志
Manager_Jar log myapp.jar --lines 100
```

#### 清理日志
```bash
# 清理指定应用的日志
Manager_Jar log myapp.jar --clean

# 清理所有日志
Manager_Jar logs clean
```

---

## 🔧 高级功能

### 配置文件管理

Manager_Jar 支持全局配置文件，位于 `~/.Manager_Jar/config.toml`：

```toml
[java]
# 默认 Java 路径
java_path = "java"
# 默认 JVM 参数
default_args = ["-server", "-Xms256m"]

[logging]
# 日志保留天数
log_retention_days = 30
# 日志文件最大大小 (MB)
log_max_size_mb = 100

[daemon]
# 健康检查间隔 (秒)
health_check_interval = 60
# 自动重启
auto_restart = true
```

#### 配置命令
```bash
# 查看当前配置
Manager_Jar config show

# 设置配置项
Manager_Jar config set java.java_path "/usr/lib/jvm/java-11/bin/java"

# 编辑配置文件
Manager_Jar config edit

# 重置为默认配置
Manager_Jar config reset
```

### 应用程序配置

为每个 JAR 应用创建单独的配置文件：

```bash
# 为应用保存启动配置
Manager_Jar config save myapp.jar -Xmx2g -Dspring.profiles.active=prod

# 使用保存的配置启动
Manager_Jar start myapp.jar --use-config

# 编辑应用配置
Manager_Jar config edit myapp.jar

# 删除应用配置
Manager_Jar config delete myapp.jar
```

---

## 📦 批量操作

批量操作允许您同时管理多个 JAR 应用，极大提高工作效率。

### 批量启动

#### 启动多个指定应用
```bash
Manager_Jar batch start app1.jar app2.jar app3.jar
```

#### 启动所有 JAR 文件
```bash
Manager_Jar batch start *.jar
```

#### 带通配符的批量操作
```bash
# 启动所有以 "service" 开头的 JAR 文件
Manager_Jar batch start service*.jar

# 启动所有在 "microservices" 目录下的 JAR 文件
Manager_Jar batch start microservices/*.jar
```

### 批量停止

```bash
# 停止多个应用
Manager_Jar batch stop app1.jar app2.jar

# 停止所有运行中的应用
Manager_Jar batch stop --all
```

### 批量重启

```bash
# 重启多个应用
Manager_Jar batch restart app1.jar app2.jar

# 重启所有应用
Manager_Jar batch restart *.jar
```

### 批量强制终止

```bash
# 强制终止无响应的应用
Manager_Jar batch kill app1.jar app2.jar
```

### 批量操作选项

```bash
# 设置操作间隔（毫秒）
Manager_Jar batch start app1.jar app2.jar --interval 1000

# 并行执行（默认是串行）
Manager_Jar batch start app1.jar app2.jar --parallel

# 失败时继续执行
Manager_Jar batch start app1.jar app2.jar --continue-on-error
```

### 批量操作输出示例

```
=== 批量操作 start (3 个文件)

[启动] 1/3 myapp.jar
✅ 启动成功: myapp.jar

[启动] 2/3 backend.jar
✅ 启动成功: backend.jar

[启动] 3/3 frontend.jar
❌ 启动失败: frontend.jar - 端口已被占用

批量启动完成 成功: 2 | 失败: 1
```

---

## 🔗 序列管理

序列功能允许您创建预定义的 JAR 启动顺序，适用于微服务架构等需要按特定顺序启动服务的场景。

### 创建序列

#### 创建基础序列
```bash
# 创建名为 "microservices" 的序列
Manager_Jar sequence create microservices database.jar config-server.jar eureka.jar gateway.jar user-service.jar order-service.jar
```

#### 从文件创建序列
```bash
# 从文本文件创建序列
Manager_Jar sequence create prod-env --from-file services.txt
```

`services.txt` 内容示例：
```
# 基础服务
database.jar
redis.jar
config-server.jar

# 注册中心
eureka.jar

# 网关
gateway.jar

# 业务服务
user-service.jar
order-service.jar
payment-service.jar
```

### 序列操作

#### 列出所有序列
```bash
Manager_Jar sequence list
```

输出示例：
```
可用序列:
  • microservices (6 个 JAR)
  • prod-env (8 个 JAR)
  • test-env (4 个 JAR)
```

#### 查看序列内容
```bash
Manager_Jar sequence show microservices
```

输出示例：
```
序列: microservices
包含: 6 个JAR
  1. database.jar
  2. config-server.jar
  3. eureka.jar
  4. gateway.jar
  5. user-service.jar
  6. order-service.jar
```

#### 执行序列操作
```bash
# 按序列启动所有服务
Manager_Jar sequence start microservices

# 按序列停止所有服务（逆序）
Manager_Jar sequence stop microservices

# 按序列重启所有服务
Manager_Jar sequence restart microservices
```

#### 检查序列状态
```bash
Manager_Jar sequence status microservices
```

输出示例：
```
序列状态: microservices
  • database.jar - 运行中
  • config-server.jar - 运行中
  • eureka.jar - 运行中
  • gateway.jar - 已停止
  • user-service.jar - 已停止
  • order-service.jar - 已停止

总计: 运行: 3 | 停止: 3
```

### 序列管理

#### 编辑序列
```bash
# 编辑序列文件
Manager_Jar sequence edit microservices

# 添加服务到序列
Manager_Jar sequence add microservices notification-service.jar

# 从序列中移除服务
Manager_Jar sequence remove microservices old-service.jar
```

#### 删除序列
```bash
Manager_Jar sequence delete microservices
```

### 序列启动选项

```bash
# 设置服务启动间隔
Manager_Jar sequence start microservices --interval 5000

# 失败时停止执行
Manager_Jar sequence start microservices --fail-fast

# 显示详细进度
Manager_Jar sequence start microservices --verbose
```

---

## 🤖 守护进程

守护进程功能提供 24/7 的应用程序监控和自动化管理。

### 启动守护进程

#### 基础启动
```bash
# 启动守护进程
Manager_Jar daemon start

# 后台启动守护进程
Manager_Jar daemon start --background
```

#### 配置守护进程
```bash
# 设置健康检查间隔（秒）
Manager_Jar daemon start --check-interval 30

# 启用自动重启
Manager_Jar daemon start --auto-restart

# 设置日志轮转
Manager_Jar daemon start --log-rotation daily
```

### 守护进程状态

#### 查看守护进程状态
```bash
Manager_Jar daemon status
```

输出示例：
```
🤖 守护进程状态:
   状态: 运行中
   PID: 98765
   运行时间: 2天 3小时 45分钟
   监控应用: 5 个
   
📊 监控统计:
   健康检查: 1,234 次
   自动重启: 3 次
   日志轮转: 12 次
   
🔧 配置信息:
   检查间隔: 60 秒
   自动重启: 启用
   日志保留: 30 天
```

### 守护进程管理

#### 停止守护进程
```bash
Manager_Jar daemon stop
```

#### 重启守护进程
```bash
Manager_Jar daemon restart
```

#### 重新加载配置
```bash
Manager_Jar daemon reload
```

### 守护进程功能

#### 健康检查
- 定期检查应用程序是否响应
- 检测内存泄漏和资源异常
- 监控应用程序端口状态

#### 自动重启
- 应用程序崩溃时自动重启
- 可配置最大重启次数
- 支持重启延迟设置

#### 日志管理
- 自动轮转应用程序日志
- 清理过期日志文件
- 压缩历史日志

#### 资源监控
- 监控 CPU 和内存使用率
- 检测磁盘空间不足
- 报告异常资源使用

---

## 📋 日志管理

完善的日志管理功能，帮助您更好地调试和监控应用程序。

### 查看日志

#### 实时日志查看
```bash
# 查看应用程序的实时日志
Manager_Jar log myapp.jar

# 跟踪日志输出（类似 tail -f）
Manager_Jar log myapp.jar --follow

# 查看错误日志
Manager_Jar log myapp.jar --level error
```

#### 历史日志查看
```bash
# 查看最近 100 行日志
Manager_Jar log myapp.jar --lines 100

# 查看指定时间范围的日志
Manager_Jar log myapp.jar --since "2024-01-01 00:00:00"
Manager_Jar log myapp.jar --until "2024-01-02 00:00:00"

# 搜索日志内容
Manager_Jar log myapp.jar --grep "ERROR"
```

### 日志文件管理

#### 列出日志文件
```bash
# 列出所有应用的日志文件
Manager_Jar logs list

# 列出特定应用的日志文件
Manager_Jar logs list myapp.jar
```

#### 清理日志
```bash
# 清理指定应用的日志
Manager_Jar logs clean myapp.jar

# 清理所有应用的日志
Manager_Jar logs clean --all

# 清理超过指定天数的日志
Manager_Jar logs clean --older-than 7days
```

### 日志配置

#### 设置日志级别
```bash
# 设置全局日志级别
Manager_Jar config set logging.level INFO

# 设置特定应用的日志级别
Manager_Jar config set logging.applications.myapp.level DEBUG
```

#### 配置日志轮转
```bash
# 设置日志文件最大大小
Manager_Jar config set logging.max_size "100MB"

# 设置日志保留天数
Manager_Jar config set logging.retention_days 30

# 启用日志压缩
Manager_Jar config set logging.compress true
```

---

## 🛠️ 构建脚本

项目提供了多种构建脚本，支持不同的开发和部署需求。

### PowerShell 脚本 (Windows)

```powershell
# 基础构建
.\build.ps1

# 构建发布版本
.\build.ps1 release

# 清理构建文件
.\build.ps1 clean

# 运行测试
.\build.ps1 test

# 检查代码
.\build.ps1 check

# 格式化代码
.\build.ps1 format

# 显示帮助
.\build.ps1 help
```

### Bash 脚本 (Linux/macOS)

```bash
# 基础构建
./build.sh

# 构建发布版本
./build.sh release

# 清理构建文件
./build.sh clean

# 运行测试
./build.sh test

# 检查代码
./build.sh check

# 格式化代码
./build.sh format

# 显示帮助
./build.sh help
```

### 批处理脚本 (Windows CMD)

```cmd
REM 基础构建
build.bat

REM 构建发布版本
build.bat release

REM 清理构建文件
build.bat clean

REM 检查代码
build.bat check

REM 显示帮助
build.bat help
```

### 使用 Cargo 直接构建

```bash
# 调试构建
cargo build

# 发布构建
cargo build --release

# 运行测试
cargo test

# 检查代码
cargo check

# 格式化代码
cargo fmt

# 生成文档
cargo doc --open
```

---

## 🎯 命令补全

Manager_Jar 支持为多种 Shell 生成自动补全脚本，提升命令行使用体验。

### 生成补全脚本

#### Bash 补全
```bash
# 生成 Bash 补全脚本
Manager_Jar completions bash > ~/.manager_jar_completions

# 添加到 ~/.bashrc
echo "source ~/.manager_jar_completions" >> ~/.bashrc
source ~/.bashrc
```

#### Zsh 补全
```bash
# 生成 Zsh 补全脚本
Manager_Jar completions zsh > ~/.manager_jar_completions.zsh

# 添加到 ~/.zshrc
echo "source ~/.manager_jar_completions.zsh" >> ~/.zshrc
source ~/.zshrc
```

#### PowerShell 补全
```powershell
# 生成 PowerShell 补全脚本
Manager_Jar completions powershell > Manager_Jar_completions.ps1

# 添加到 PowerShell Profile
Add-Content $PROFILE ". $(PWD)\Manager_Jar_completions.ps1"
```

### 补全功能特性

- **命令补全**：自动补全所有可用命令
- **文件补全**：自动补全 JAR 文件名
- **参数补全**：补全命令参数和选项
- **智能建议**：根据上下文提供相关建议

---

## 🔍 故障排除

### 常见问题及解决方案

#### 1. JAR 文件无法启动

**问题**：`Manager_Jar start myapp.jar` 失败

**可能原因**：
- Java 环境未安装或配置错误
- JAR 文件损坏或不存在
- 端口被占用
- 权限不足

**解决步骤**：
```bash
# 1. 检查 Java 环境
java -version

# 2. 检查 JAR 文件是否存在
ls -la myapp.jar

# 3. 测试 JAR 文件能否直接运行
java -jar myapp.jar

# 4. 检查端口占用
netstat -an | grep :8080

# 5. 查看详细错误信息
Manager_Jar start myapp.jar --verbose
```

#### 2. 应用程序无法停止

**问题**：`Manager_Jar stop myapp.jar` 无效果

**解决方案**：
```bash
# 1. 查看应用状态
Manager_Jar status myapp.jar

# 2. 强制终止
Manager_Jar kill myapp.jar

# 3. 检查进程是否仍在运行
ps aux | grep myapp.jar

# 4. 手动清理 PID 文件
rm ~/.Manager_Jar/pids/myapp.pid
```

#### 3. 配置文件问题

**问题**：配置无法加载或保存

**解决方案**：
```bash
# 1. 检查配置目录权限
ls -la ~/.Manager_Jar/

# 2. 重建配置目录
rm -rf ~/.Manager_Jar/configs/
Manager_Jar config reset

# 3. 验证配置文件格式
Manager_Jar config show
```

#### 4. 守护进程问题

**问题**：守护进程无法启动或异常停止

**解决方案**：
```bash
# 1. 检查守护进程状态
Manager_Jar daemon status

# 2. 查看守护进程日志
Manager_Jar log daemon --lines 50

# 3. 强制停止并重启
Manager_Jar daemon stop
Manager_Jar daemon start --background

# 4. 重置守护进程配置
Manager_Jar config reset daemon
```

### 调试模式

#### 启用详细输出
```bash
# 启用调试模式
export MANAGER_JAR_DEBUG=1
Manager_Jar start myapp.jar

# 或使用 --verbose 参数
Manager_Jar start myapp.jar --verbose
```

#### 查看系统信息
```bash
# 显示系统和版本信息
Manager_Jar version

# 显示配置信息
Manager_Jar config show

# 显示环境变量
Manager_Jar env
```

---

## ❓ 常见问题

### Q1: Manager_Jar 支持哪些 Java 版本？

**A**: Manager_Jar 支持 Java 8 及更高版本。推荐使用 Java 11 或 Java 17 LTS 版本。

### Q2: 如何在服务器上运行 Manager_Jar？

**A**: 推荐使用守护进程模式：
```bash
# 启动守护进程
Manager_Jar daemon start --background

# 添加应用到监控
Manager_Jar start myapp.jar --daemon-monitor
```

### Q3: 如何迁移现有的 JAR 应用管理？

**A**: 您可以：
1. 创建序列文件包含所有现有应用
2. 使用批量操作导入现有配置
3. 逐步替换现有的启动脚本

### Q4: Manager_Jar 的性能开销如何？

**A**: Manager_Jar 使用 Rust 编写，内存占用极小（通常 < 10MB），CPU 使用率几乎为零。守护进程模式下的监控开销也非常轻量。

### Q5: 如何在 Docker 容器中使用？

**A**: 创建 Dockerfile：
```dockerfile
FROM openjdk:11-jre-slim
COPY Manager_Jar /usr/local/bin/
COPY *.jar /app/
WORKDIR /app
CMD ["Manager_Jar", "daemon", "start", "--background"]
```

### Q6: 如何备份和恢复配置？

**A**: 配置文件位于 `~/.Manager_Jar/` 目录：
```bash
# 备份配置
tar -czf manager_jar_config.tar.gz ~/.Manager_Jar/

# 恢复配置
tar -xzf manager_jar_config.tar.gz -C ~/
```

### Q7: Manager_Jar 是否支持集群管理？

**A**: 当前版本主要针对单机管理。集群管理功能在未来版本的规划中。

---

## 👥 开发指南

### 项目结构

```
Manager_Jar/
├── src/
│   ├── main.rs           # 主入口
│   ├── cli/              # 命令行接口
│   │   ├── commands.rs   # 命令定义
│   │   └── completions.rs# 补全功能
│   ├── core/             # 核心功能
│   │   ├── config.rs     # 配置管理
│   │   ├── logging.rs    # 日志系统
│   │   └── process.rs    # 进程管理
│   ├── operations/       # 操作模块
│   │   ├── batch.rs      # 批量操作
│   │   └── sequence.rs   # 序列管理
│   ├── daemon/           # 守护进程
│   │   ├── thread_daemon.rs
│   │   └── system_daemon.rs
│   └── utils/            # 工具模块
│       ├── display.rs    # 显示工具
│       ├── files.rs      # 文件工具
│       └── macros.rs     # 宏定义
├── build.rs              # 构建脚本
├── Cargo.toml           # 项目配置
└── README.md            # 项目说明
```

### 贡献指南

1. **Fork 项目**并创建功能分支
2. **编写代码**并确保通过所有测试
3. **更新文档**包括 README 和内联注释
4. **提交 Pull Request**

### 开发环境设置

```bash
# 克隆项目
git clone https://github.com/your-repo/Manager_Jar.git
cd Manager_Jar

# 安装依赖
cargo build

# 运行测试
cargo test

# 格式化代码
cargo fmt

# 检查代码质量
cargo clippy
```

### 代码风格

- 使用 `rustfmt` 进行代码格式化
- 遵循 Rust 官方代码风格指南
- 为公共 API 编写文档注释
- 编写单元测试和集成测试

---

## 📞 支持与联系

### 获取帮助

- **GitHub Issues**: [提交问题](https://github.com/your-repo/Manager_Jar/issues)
- **讨论区**: [GitHub Discussions](https://github.com/your-repo/Manager_Jar/discussions)
- **邮箱**: creastar@gmail.com

### 社区资源

- **官方文档**: [Manager_Jar Docs](https://manager-jar.docs.com)
- **示例项目**: [Examples Repository](https://github.com/your-repo/Manager_Jar-examples)
- **视频教程**: [YouTube Channel](https://youtube.com/manager-jar)

---

## 📄 许可证

本项目采用 MIT 许可证。详情请查看 [LICENSE](LICENSE) 文件。

---

## 🎉 致谢

感谢所有为 Manager_Jar 项目做出贡献的开发者和用户！

特别感谢：
- Rust 社区提供的优秀生态系统
- Clap 库提供的命令行解析功能
- Colored 库提供的终端色彩支持

---

<div align="center">

**如果这个项目对您有帮助，请给我们一个 ⭐ Star！**

![Stars](https://img.shields.io/github/stars/your-repo/Manager_Jar?style=social)
![Forks](https://img.shields.io/github/forks/your-repo/Manager_Jar?style=social)

</div>
