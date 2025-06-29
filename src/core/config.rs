// core/config.rs - 配置管理模块（整合所有配置功能）

// ============= 基础配置管理 =============
use std::fs;
use std::path::PathBuf;
use colored::Colorize;

// ============= 全局配置管理 =============
use serde::{Deserialize, Serialize};

pub const GLOBAL_CONFIG_FILE: &str = ".Manager_Jar/configs/global_config.toml";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GlobalConfig {
    /// 日志配置
    pub log: LogConfig,
    /// 进程配置
    pub process: ProcessConfig,
    /// 系统配置
    pub system: SystemConfig,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogConfig {
    /// 日志保存目录
    pub log_dir: String,
    /// 日志保存天数
    pub retention_days: u32,
    /// 单个日志文件最大大小 (MB)
    pub max_file_size_mb: u32,
    /// 是否启用日志轮转
    pub enable_rotation: bool,
    /// 日志时间戳格式
    pub timestamp_format: String,
    /// 是否启用日志压缩 (针对旧日志)
    pub enable_compression: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProcessConfig {
    /// 默认 Java 参数
    pub default_java_args: Vec<String>,
    /// 进程检查间隔（秒）
    pub health_check_interval: u32,
    /// 启动超时时间（秒）
    pub startup_timeout: u32,
    /// 停止超时时间（秒）
    pub shutdown_timeout: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SystemConfig {
    /// 彩色输出
    pub enable_color: bool,
    /// 详细输出
    pub verbose: bool,
    /// 自动清理PID文件
    pub auto_cleanup_pid: bool,
    /// 最大并发操作数
    pub max_concurrent_operations: u32,
}

// ============= 原有的JAR配置管理功能 =============
const CONFIG_DIR: &str = ".Manager_Jar/configs";

// 保存JAR配置参数
pub fn save_jar_config(jar: &str, args: &[String]) -> Result<(), String> {
    fs::create_dir_all(CONFIG_DIR).map_err(|e| format!("创建配置目录失败: {}", e))?;
    
    let config_file = PathBuf::from(CONFIG_DIR).join(format!("{}.config", jar.replace(".jar", "")));
    let config_args = args.join(" ");
    
    fs::write(&config_file, &config_args).map_err(|e| format!("保存配置失败: {}", e))?;
    
    print_success!("配置已保存: {}", jar.bright_cyan());
    if !config_args.is_empty() {
        println!("  参数: {}", config_args.yellow());
    } else {
        println!("  参数: {}", "无".bright_black());
    }
    println!("  文件: {}", config_file.display().to_string().cyan());
    
    Ok(())
}

// 使用已保存配置快速启动JAR
pub fn quick_start_jar(jar: &str) -> Result<(), String> {
    let config_file = PathBuf::from(CONFIG_DIR).join(format!("{}.config", jar.replace(".jar", "")));
    
    let config_args = if config_file.exists() {
        match fs::read_to_string(&config_file) {
            Ok(content) => content,
            Err(e) => {
                print_warn!("读取配置文件失败: {}", e);
                String::new()
            }
        }
    } else {
        print_warn!("配置文件不存在: {}", config_file.display());
        print_warn!("使用默认参数启动 {}", jar);
        String::new()
    };
    
    let args: Vec<String> = if config_args.trim().is_empty() {
        Vec::new()
    } else {
        config_args.split_whitespace().map(|s| s.to_string()).collect()
    };
    
    print_success!("快速启动 {} ...", jar.bright_cyan());
    if !args.is_empty() {
        println!("  使用配置: {}", args.join(" ").yellow());
    }
    
    crate::core::process::start_jar(jar, &args)
}

// 显示JAR配置
pub fn show_config(jar: &str) -> Result<(), String> {
    let config_file = PathBuf::from(CONFIG_DIR).join(format!("{}.config", jar.replace(".jar", "")));
    
    if config_file.exists() {
        match fs::read_to_string(&config_file) {
            Ok(content) => {
                print_success!("配置文件: {}", config_file.display().to_string().cyan());
                if content.trim().is_empty() {
                    println!("  参数: {}", "无".bright_black());
                } else {
                    println!("  参数: {}", content.trim().yellow());
                }
                Ok(())
            },
            Err(e) => {
                let error_msg = format!("读取配置文件失败: {}", e);
                print_error!("{}", error_msg);
                Err(error_msg)
            }
        }
    } else {
        print_warn!("配置文件不存在: {}", jar);
        Ok(())
    }
}

// 删除JAR配置
pub fn delete_config(jar: &str) -> Result<(), String> {
    let config_file = PathBuf::from(CONFIG_DIR).join(format!("{}.config", jar.replace(".jar", "")));
    
    if config_file.exists() {
        fs::remove_file(&config_file).map_err(|e| format!("删除配置失败: {}", e))?;
        print_success!("已删除配置: {}", jar.bright_cyan());
        Ok(())
    } else {
        print_warn!("配置文件不存在: {}", jar);
        Ok(())
    }
}

// 列出所有配置文件
pub fn list_configs() {
    if let Ok(entries) = fs::read_dir(CONFIG_DIR) {
        let mut configs = Vec::new();
        
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("config") {
                if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                    let jar_name = format!("{}.jar", name);
                    if let Ok(content) = fs::read_to_string(&path) {
                        configs.push((jar_name, content));
                    }
                }
            }
        }
        
        if configs.is_empty() {
            print_warn!("无已保存的配置");
        } else {
            print_success!("已保存的配置 ({} 个):", configs.len());
            configs.sort_by(|a, b| a.0.cmp(&b.0));
            for (jar, config) in configs {
                if config.trim().is_empty() {
                    println!("  {} : {}", jar.cyan(), "无参数".bright_black());
                } else {
                    println!("  {} : {}", jar.cyan(), config.trim().yellow());
                }
            }
        }
    } else {
        print_warn!("无法访问配置目录: {}", CONFIG_DIR);
    }
}

// ============= 全局配置实现 =============
impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            log: LogConfig {
                log_dir: ".Manager_Jar/logs".to_string(),
                retention_days: 15,
                max_file_size_mb: 10,
                enable_rotation: true,
                timestamp_format: "%Y-%m-%d %H:%M:%S".to_string(),
                enable_compression: false,
            },
            process: ProcessConfig {
                default_java_args: vec!["-Xmx512m".to_string()],
                health_check_interval: 30,
                startup_timeout: 60,
                shutdown_timeout: 30,
            },
            system: SystemConfig {
                enable_color: true,
                verbose: false,
                auto_cleanup_pid: true,
                max_concurrent_operations: 5,
            },
        }
    }
}

impl GlobalConfig {
    /// 加载配置文件
    pub fn load() -> Self {
        if let Ok(content) = fs::read_to_string(GLOBAL_CONFIG_FILE) {
            match toml::from_str(&content) {
                Ok(config) => config,
                Err(e) => {
                    print_warn!("配置文件解析失败，使用默认配置: {}", e);
                    Self::default()
                }
            }
        } else {
            // 配置文件不存在，创建默认配置
            let default_config = Self::default();
            if let Err(e) = default_config.save() {
                print_warn!("创建默认配置文件失败: {}", e);
            }
            default_config
        }
    }

    /// 保存配置文件
    pub fn save(&self) -> Result<(), String> {
        // 确保目录存在
        if let Some(parent) = PathBuf::from(GLOBAL_CONFIG_FILE).parent() {
            fs::create_dir_all(parent).map_err(|e| format!("创建配置目录失败: {}", e))?;
        }

        // 生成带注释的配置内容
        let toml_content = self.generate_config_with_comments();

        fs::write(GLOBAL_CONFIG_FILE, toml_content)
            .map_err(|e| format!("写入配置文件失败: {}", e))?;

        Ok(())
    }

    /// 生成带详细注释的配置文件内容
    fn generate_config_with_comments(&self) -> String {
        let default_args_str = self.process.default_java_args
            .iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<_>>()
            .join(", ");

        format!(r#"# Manager_Jar 全局配置文件
# 此文件控制 Manager_Jar 的各种行为和设置
# 修改此文件后，使用 'global-config reload' 命令重新加载配置

# ========================================
# 日志配置 - 控制日志的存储和管理
# ========================================
[log]
# 日志文件存储目录
log_dir = "{}"

# 日志文件保存天数 (超过此天数的日志文件将被清理)
retention_days = {}

# 单个日志文件最大大小 (MB)
max_file_size_mb = {}

# 是否启用日志轮转功能
enable_rotation = {}

# 日志时间戳格式 (strftime格式)
timestamp_format = "{}"

# 是否启用日志压缩 (实验性功能)
enable_compression = {}

# ========================================
# 进程配置 - 控制 JAR 进程的启动和管理
# ========================================
[process]
# 默认 Java 启动参数
default_java_args = [{}]

# 健康检查间隔 (秒)
health_check_interval = {}

# JAR 启动超时时间 (秒)
startup_timeout = {}

# JAR 停止超时时间 (秒)
shutdown_timeout = {}

# ========================================
# 系统配置 - 控制工具本身的行为
# ========================================
[system]
# 是否启用彩色输出
enable_color = {}

# 是否启用详细输出
verbose = {}

# 是否自动清理PID文件
auto_cleanup_pid = {}

# 最大并发操作数
max_concurrent_operations = {}
"#,
            self.log.log_dir,
            self.log.retention_days,
            self.log.max_file_size_mb,
            self.log.enable_rotation,
            self.log.timestamp_format,
            self.log.enable_compression,
            default_args_str,
            self.process.health_check_interval,
            self.process.startup_timeout,
            self.process.shutdown_timeout,
            self.system.enable_color,
            self.system.verbose,
            self.system.auto_cleanup_pid,
            self.system.max_concurrent_operations,
        )
    }

    /// 获取日志文件路径
    pub fn get_log_file_path(&self, jar_name: &str) -> String {
        let log_name = format!("{}.log", jar_name.replace(".jar", ""));
        format!("{}/{}", self.log.log_dir, log_name)
    }

    /// 检查是否需要轮转日志
    pub fn should_rotate_log(&self, log_path: &str) -> bool {
        if !self.log.enable_rotation {
            return false;
        }
        
        if let Ok(metadata) = fs::metadata(log_path) {
            let size_mb = metadata.len() / (1024 * 1024);
            size_mb >= self.log.max_file_size_mb as u64
        } else {
            false
        }
    }

    /// 轮转日志文件
    pub fn rotate_log(&self, jar_name: &str) -> Result<(), String> {
        let log_path = self.get_log_file_path(jar_name);
        
        if !PathBuf::from(&log_path).exists() {
            return Ok(());
        }

        // 生成轮转后的文件名
        let rotated_path = format!("{}.1", log_path);
        
        // 移动当前日志文件
        fs::rename(&log_path, &rotated_path)
            .map_err(|e| format!("轮转日志文件失败: {}", e))?;

        print_success!("日志文件已轮转: {} -> {}", 
                      PathBuf::from(&log_path).file_name().unwrap().to_string_lossy(),
                      PathBuf::from(&rotated_path).file_name().unwrap().to_string_lossy());

        Ok(())
    }

    /// 清理过期日志文件
    pub fn cleanup_old_logs(&self) -> Result<u32, String> {
        if self.log.retention_days == 0 {
            return Ok(0); // 永不清理
        }

        let log_dir = PathBuf::from(&self.log.log_dir);
        if !log_dir.exists() {
            return Ok(0);
        }

        let cutoff_time = std::time::SystemTime::now() 
            - std::time::Duration::from_secs(self.log.retention_days as u64 * 24 * 3600);

        let mut cleaned_count = 0;

        if let Ok(entries) = fs::read_dir(&log_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "log") {
                    if let Ok(metadata) = entry.metadata() {
                        if let Ok(modified) = metadata.modified() {
                            if modified < cutoff_time {
                                if fs::remove_file(&path).is_ok() {
                                    cleaned_count += 1;
                                    print_info!("清理过期日志: {}", path.file_name().unwrap().to_string_lossy());
                                }
                            }
                        }
                    }
                }
            }
        }

        if cleaned_count > 0 {
            print_success!("清理了 {} 个过期日志文件", cleaned_count);
        }

        Ok(cleaned_count)
    }

    /// 获取配置文件路径
    pub fn config_file_path() -> String {
        GLOBAL_CONFIG_FILE.to_string()
    }

    /// 显示配置信息
    pub fn display(&self) {
        println!("{}", "=== 全局配置信息 ===".bright_blue().bold());
        
        println!("\n{}:", "日志配置".bright_green());
        println!("  日志目录: {}", self.log.log_dir.cyan());
        println!("  保存天数: {} 天", self.log.retention_days.to_string().yellow());
        println!("  文件大小: {} MB", self.log.max_file_size_mb.to_string().yellow());
        println!("  日志轮转: {}", if self.log.enable_rotation { "启用".green() } else { "禁用".red() });
        println!("  时间格式: {}", self.log.timestamp_format.cyan());
        println!("  日志压缩: {}", if self.log.enable_compression { "启用".green() } else { "禁用".red() });

        println!("\n{}:", "进程配置".bright_green());
        if self.process.default_java_args.is_empty() {
            println!("  默认参数: {}", "无".bright_black());
        } else {
            println!("  默认参数: {}", self.process.default_java_args.join(" ").yellow());
        }
        println!("  健康检查: 每 {} 秒", self.process.health_check_interval.to_string().yellow());
        println!("  启动超时: {} 秒", self.process.startup_timeout.to_string().yellow());
        println!("  停止超时: {} 秒", self.process.shutdown_timeout.to_string().yellow());

        println!("\n{}:", "系统配置".bright_green());
        println!("  彩色输出: {}", if self.system.enable_color { "启用".green() } else { "禁用".red() });
        println!("  详细输出: {}", if self.system.verbose { "启用".green() } else { "禁用".red() });
        println!("  自动清理: {}", if self.system.auto_cleanup_pid { "启用".green() } else { "禁用".red() });
        println!("  并发数量: {}", self.system.max_concurrent_operations.to_string().yellow());
        
        println!("\n配置文件: {}", GLOBAL_CONFIG_FILE.cyan());
    }

    /// 编辑配置文件
    #[allow(dead_code)]
    pub fn edit() -> Result<(), String> {
        let editor = if cfg!(target_os = "windows") {
            "notepad".to_string()
        } else {
            std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string())
        };

        let status = std::process::Command::new(&editor)
            .arg(GLOBAL_CONFIG_FILE)
            .status()
            .map_err(|e| format!("启动编辑器失败: {}", e))?;

        if status.success() {
            println!("{}", "配置文件编辑完成".green());
            println!("{}", "使用 'global-config reload' 重新加载配置".cyan());
        } else {
            println!("{}", "编辑器退出异常".red());
        }

        Ok(())
    }
}

/// 全局配置实例 (线程安全)
#[allow(dead_code)]
static GLOBAL_CONFIG: std::sync::OnceLock<std::sync::Arc<std::sync::Mutex<GlobalConfig>>> = std::sync::OnceLock::new();

/// 获取全局配置实例
#[allow(dead_code)]
pub fn get_global_config() -> &'static std::sync::Arc<std::sync::Mutex<GlobalConfig>> {
    GLOBAL_CONFIG.get_or_init(|| {
        std::sync::Arc::new(std::sync::Mutex::new(GlobalConfig::load()))
    })
}
