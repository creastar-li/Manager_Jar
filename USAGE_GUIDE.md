# 📖 Manager_Jar 使用说明

## 🎯 概述

Manager_Jar 是一个专业的 Java 应用程序生命周期管理工具，提供了完整的 JAR 包管理功能。

## 💻 基本用法

### 命令格式
```
Manager_Jar.exe [COMMAND] [OPTIONS] [ARGUMENTS]
```

## 📋 命令详细说明

### 🚀 基础操作

#### 1. 启动 JAR 应用
```powershell
# 基本启动
Manager_Jar.exe start myapp.jar

# 带参数启动
Manager_Jar.exe start myapp.jar -Xmx1g -server

# 带复杂参数启动
Manager_Jar.exe start myapp.jar -Xmx2g -Xms512m -server -Dspring.profiles.active=prod
```

#### 2. 停止 JAR 应用
```powershell
# 正常停止
Manager_Jar.exe stop myapp.jar

# 强制杀死进程
Manager_Jar.exe kill myapp.jar
```

#### 3. 重启 JAR 应用
```powershell
# 重启应用
Manager_Jar.exe restart myapp.jar
```

#### 4. 查看运行状态
```powershell
# 查看所有JAR状态
Manager_Jar.exe status

# 查看特定JAR状态
Manager_Jar.exe status myapp.jar
```

#### 5. 列出可用JAR文件
```powershell
# 列出当前目录下的JAR文件
Manager_Jar.exe list
```

### 📝 日志管理

#### 1. 查看日志
```powershell
# 查看实时日志
Manager_Jar.exe log myapp.jar

# 查看最近100行日志
Manager_Jar.exe log myapp.jar --lines 100

# 跟踪日志输出
Manager_Jar.exe log myapp.jar --follow
```

#### 2. 日志管理操作
```powershell
# 清理日志
Manager_Jar.exe logs clean myapp.jar

# 查看错误日志
Manager_Jar.exe logs error myapp.jar

# 查看所有日志文件
Manager_Jar.exe logs list
```

### ⚙️ 配置管理

#### 1. 保存配置
```powershell
# 保存启动配置
Manager_Jar.exe config myapp.jar --args "-Xmx2g -server" --name "生产环境"

# 保存开发环境配置
Manager_Jar.exe config myapp.jar --args "-Xmx1g -Xdebug" --name "开发环境"
```

#### 2. 使用配置快速启动
```powershell
# 使用已保存的配置启动
Manager_Jar.exe quick myapp.jar --config "生产环境"

# 使用默认配置启动
Manager_Jar.exe quick myapp.jar
```

#### 3. 配置管理
```powershell
# 列出所有配置
Manager_Jar.exe configs list

# 删除配置
Manager_Jar.exe configs delete myapp.jar "旧配置"

# 查看特定配置
Manager_Jar.exe configs show myapp.jar "生产环境"
```

### 🔄 批量操作

#### 1. 批量启动
```powershell
# 批量启动多个JAR
Manager_Jar.exe batch start app1.jar app2.jar app3.jar

# 批量启动所有JAR
Manager_Jar.exe batch start *.jar
```

#### 2. 批量停止
```powershell
# 批量停止
Manager_Jar.exe batch stop app1.jar app2.jar

# 批量强制停止
Manager_Jar.exe batch kill app1.jar app2.jar
```

#### 3. 批量重启
```powershell
# 批量重启
Manager_Jar.exe batch restart app1.jar app2.jar
```

#### 4. 批量状态查看
```powershell
# 查看多个JAR状态
Manager_Jar.exe batch status app1.jar app2.jar
```

### 📋 序列管理

#### 1. 创建序列
```powershell
# 创建序列
Manager_Jar.exe sequence create microservices config-server.jar eureka-server.jar gateway.jar user-service.jar
```

#### 2. 序列操作
```powershell
# 按序列启动（按顺序启动）
Manager_Jar.exe sequence start microservices

# 按序列停止（反向停止）
Manager_Jar.exe sequence stop microservices

# 序列重启
Manager_Jar.exe sequence restart microservices
```

#### 3. 序列管理
```powershell
# 列出所有序列
Manager_Jar.exe sequence list

# 查看序列详情
Manager_Jar.exe sequence show microservices

# 删除序列
Manager_Jar.exe sequence delete microservices

# 查看序列状态
Manager_Jar.exe sequence status microservices
```

### 🤖 守护进程管理

#### 1. 启动守护进程
```powershell
# 启动系统守护进程
Manager_Jar.exe daemon start

# 后台启动守护进程
Manager_Jar.exe daemon start --background
```

#### 2. 守护进程操作
```powershell
# 查看守护进程状态
Manager_Jar.exe daemon status

# 停止守护进程
Manager_Jar.exe daemon stop

# 重启守护进程
Manager_Jar.exe daemon restart
```

#### 3. 监控管理
```powershell
# 添加JAR到监控列表
Manager_Jar.exe daemon add myapp.jar

# 添加JAR到监控并启用自动重启
Manager_Jar.exe daemon add myapp.jar --auto-restart

# 从监控列表移除JAR
Manager_Jar.exe daemon remove myapp.jar

# 查看监控列表
Manager_Jar.exe daemon list
```

### 🌐 全局配置

#### 1. 设置全局配置
```powershell
# 设置日志级别
Manager_Jar.exe global-config set log_level info

# 设置最大进程数
Manager_Jar.exe global-config set max_processes 10

# 设置默认JVM参数
Manager_Jar.exe global-config set default_jvm_args "-Xmx1g"

# 设置自动重启
Manager_Jar.exe global-config set auto_restart true
```

#### 2. 查看和管理配置
```powershell
# 查看所有全局配置
Manager_Jar.exe global-config show

# 获取特定配置
Manager_Jar.exe global-config get log_level

# 重置所有配置
Manager_Jar.exe global-config reset
```

### 🔧 工具功能

#### 1. 生成命令补全
```powershell
# 生成PowerShell补全脚本
Manager_Jar.exe completions powershell > Manager_Jar_completions.ps1

# 生成Bash补全脚本
Manager_Jar.exe completions bash > Manager_Jar_completions.bash

# 生成Zsh补全脚本
Manager_Jar.exe completions zsh > Manager_Jar_completions.zsh
```

#### 2. 版本信息
```powershell
# 查看详细版本信息
Manager_Jar.exe version

# 查看简短版本
Manager_Jar.exe --version
```

## 💡 使用场景示例

### 场景1: 微服务应用管理
```powershell
# 1. 创建微服务启动序列
Manager_Jar.exe sequence create microservices config-server.jar eureka-server.jar gateway.jar user-service.jar order-service.jar

# 2. 按序列启动所有服务
Manager_Jar.exe sequence start microservices

# 3. 启动守护进程监控
Manager_Jar.exe daemon start

# 4. 添加关键服务到监控
Manager_Jar.exe daemon add config-server.jar --auto-restart
Manager_Jar.exe daemon add eureka-server.jar --auto-restart

# 5. 查看所有服务状态
Manager_Jar.exe sequence status microservices
```

### 场景2: 开发环境管理
```powershell
# 1. 保存开发环境配置
Manager_Jar.exe config myapp.jar --args "-Xmx1g -Xdebug -Xrunjdwp:transport=dt_socket,server=y,suspend=n,address=5005" --name "debug"

# 2. 快速启动调试模式
Manager_Jar.exe quick myapp.jar --config "debug"

# 3. 查看实时日志
Manager_Jar.exe log myapp.jar --follow

# 4. 需要重启时
Manager_Jar.exe restart myapp.jar
```

### 场景3: 生产环境部署
```powershell
# 1. 保存生产环境配置
Manager_Jar.exe config webapp.jar --args "-Xmx4g -Xms2g -server -XX:+UseG1GC" --name "production"

# 2. 启动应用
Manager_Jar.exe quick webapp.jar --config "production"

# 3. 启动守护进程
Manager_Jar.exe daemon start --background

# 4. 添加到监控并启用自动重启
Manager_Jar.exe daemon add webapp.jar --auto-restart

# 5. 定期检查状态
Manager_Jar.exe status webapp.jar
```

## ⚠️ 注意事项

### 环境要求
- **操作系统**: Windows 10+
- **Java环境**: JRE 8+ 或 JDK 8+
- **权限**: 某些操作可能需要管理员权限

### 最佳实践
1. **配置管理**: 为不同环境保存不同的配置预设
2. **监控设置**: 对关键应用启用守护进程监控
3. **日志管理**: 定期清理日志文件避免占用过多磁盘空间
4. **批量操作**: 使用序列管理器管理有依赖关系的应用组
5. **备份配置**: 定期备份配置文件和序列定义

### 故障排除
- 如果JAR无法启动，检查Java环境和JAR文件路径
- 如果端口冲突，使用 `Manager_Jar.exe status` 检查运行中的应用
- 如果日志无法查看，检查日志文件权限
- 如果守护进程无法启动，检查系统权限和防火墙设置

## 📞 获取帮助

### 内置帮助
```powershell
# 查看主帮助
Manager_Jar.exe --help

# 查看特定命令帮助
Manager_Jar.exe [COMMAND] --help
```

### 配置文件位置
- **Windows**: `%APPDATA%\Manager_Jar\`
  - 全局配置: `config\global.toml`
  - JAR配置: `config\jars\*.toml`
  - 序列配置: `config\sequences\*.toml`
  - 日志文件: `logs\*.log`
