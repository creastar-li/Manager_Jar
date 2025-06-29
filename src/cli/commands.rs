// cli/commands.rs - 命令定义和参数解析

use clap::{Parser, Subcommand};

/// 主命令行接口结构
#[derive(Parser, Debug)]
#[command(name = "Manager_Jar", version = "1.0.0", about = "JAR包管理工具 - 专业的Java应用程序生命周期管理")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// 主命令枚举
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 生成命令行补全脚本
    Completions { 
        /// Shell类型 (bash, zsh, powershell)
        shell: String 
    },
    /// 启动指定JAR包
    Start { 
        /// JAR文件名
        jar: String, 
        /// Java启动参数
        args: Vec<String> 
    },
    /// 停止指定JAR包
    Stop { 
        /// JAR文件名
        jar: String 
    },
    /// 重启指定JAR包
    Restart { 
        /// JAR文件名
        jar: String, 
        /// Java启动参数
        args: Vec<String> 
    },
    /// 查看JAR包状态
    Status { 
        /// JAR文件名 (可选，显示所有)
        jar: Option<String> 
    },
    /// 列出所有可用JAR文件
    List,
    /// 查看JAR日志
    Log { 
        /// JAR文件名
        jar: String, 
        /// 显示行数
        #[arg(default_value = "20")] 
        lines: u32 
    },
    /// 强制杀死JAR进程
    Kill { 
        /// JAR文件名
        jar: String 
    },
    /// 配置JAR参数
    Config { 
        /// JAR文件名
        jar: String, 
        /// 启动参数
        args: Vec<String> 
    },
    /// 用已保存配置快速启动JAR
    Quick { 
        /// JAR文件名
        jar: String 
    },
    /// 批量操作
    Batch { 
        /// 操作类型 (start, stop, restart, kill)
        op: String, 
        /// JAR文件列表
        jars: Vec<String> 
    },
    /// 序列化批量管理JAR组
    Sequence { 
        /// 操作类型 (create, start, stop, restart, list, show, delete)
        op: String, 
        /// 参数列表
        args: Vec<String> 
    },
    /// 日志管理
    Logs { 
        #[command(subcommand)]
        action: LogsAction 
    },
    /// 配置管理
    Configs { 
        #[command(subcommand)]
        action: ConfigsAction 
    },
    /// 全局配置管理
    GlobalConfig { 
        #[command(subcommand)]
        action: GlobalConfigAction 
    },
    /// 系统级守护进程管理
    Daemon { 
        #[command(subcommand)]
        action: DaemonAction 
    },
    /// 显示版本信息
    Version,
}

/// 日志管理子命令
#[derive(clap::Subcommand, Debug)]
pub enum LogsAction {
    /// 列出所有日志文件
    List,
    /// 清理所有日志文件
    Clean,
}

/// 配置管理子命令
#[derive(clap::Subcommand, Debug)]
pub enum ConfigsAction {
    /// 列出所有配置
    List,
    /// 查看指定JAR的配置
    Show { 
        /// JAR文件名
        jar: String 
    },
    /// 删除指定JAR的配置
    Delete { 
        /// JAR文件名
        jar: String 
    },
}

/// 全局配置管理子命令
#[derive(clap::Subcommand, Debug)]
pub enum GlobalConfigAction {
    /// 显示当前全局配置
    Show,
    /// 编辑全局配置文件
    Edit,
    /// 重置为默认配置
    Reset,
    /// 重新加载配置
    Reload,
    /// 清理过期日志
    CleanLogs,
    /// 设置日志目录
    SetLogDir { 
        /// 日志目录路径
        path: String 
    },
    /// 设置日志保存天数
    SetRetentionDays { 
        /// 保存天数
        days: u32 
    },
    /// 设置日志文件最大大小(MB)
    SetMaxLogSize { 
        /// 文件大小(MB)
        size_mb: u32 
    },
    /// 启用/禁用日志轮转
    SetLogRotation { 
        /// 是否启用
        enable: bool 
    },
}

/// 守护进程管理子命令
#[derive(clap::Subcommand, Debug)]
pub enum DaemonAction {
    /// 启动系统级守护进程
    Start,
    /// 停止系统级守护进程
    Stop,
    /// 查看守护进程状态
    Status,
    /// 重启守护进程
    Restart,
}
