// thread_daemon.rs - 后台守护进程
use std::time::Duration;
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use colored::Colorize;
use crate::core::config::GlobalConfig;
use crate::core::process;

#[allow(dead_code)]
pub struct ManagerDaemon {
    running: Arc<AtomicBool>,
    config: Arc<std::sync::Mutex<GlobalConfig>>,
}

impl ManagerDaemon {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            config: Arc::new(std::sync::Mutex::new(GlobalConfig::load())),
        }
    }

    /// 启动守护进程
    pub fn start(&self) -> Result<(), String> {
        if self.running.load(Ordering::Relaxed) {
            return Err("守护进程已在运行".to_string());
        }

        self.running.store(true, Ordering::Relaxed);
        print_success!("正在启动 Manager_Jar 守护进程...");

        let running = Arc::clone(&self.running);
        let config = Arc::clone(&self.config);

        // 创建守护线程
        thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                if let Ok(cfg) = config.lock() {
                    let check_interval = cfg.process.health_check_interval;
                    drop(cfg);

                    // 执行定期任务
                    Self::perform_periodic_tasks(&config);
                    
                    // 等待下次检查
                    thread::sleep(Duration::from_secs(check_interval as u64));
                } else {
                    thread::sleep(Duration::from_secs(30)); // 默认30秒
                }
            }
        });

        print_success!("守护进程已启动，将在后台继续运行");
        println!("  • 自动健康检查");
        println!("  • 自动日志轮转");
        println!("  • 自动过期日志清理");
        println!("  • 自动僵尸进程清理");

        Ok(())
    }

    /// 停止守护进程
    pub fn stop(&self) {
        if self.running.load(Ordering::Relaxed) {
            self.running.store(false, Ordering::Relaxed);
            print_success!("守护进程已停止");
        } else {
            print_warn!("守护进程未在运行");
        }
    }

    /// 检查守护进程状态
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    /// 执行定期任务
    fn perform_periodic_tasks(config: &Arc<std::sync::Mutex<GlobalConfig>>) {
        if let Ok(cfg) = config.lock() {
            let verbose = cfg.system.verbose;
            
            // 1. 健康检查 - 检查所有注册的 JAR 进程
            if verbose {
                println!("🔍 执行健康检查...");
            }
            Self::health_check(&cfg);

            // 2. 自动日志轮转检查
            if cfg.log.enable_rotation {
                if verbose {
                    println!("🔄 检查日志轮转...");
                }
                Self::check_log_rotation(&cfg);
            }

            // 3. 清理过期日志
            if cfg.log.retention_days > 0 {
                if verbose {
                    println!("🧹 清理过期日志...");
                }
                if let Err(e) = cfg.cleanup_old_logs() {
                    if verbose {
                        println!("清理过期日志失败: {}", e);
                    }
                }
            }

            // 4. 清理僵尸PID文件
            if cfg.system.auto_cleanup_pid {
                if verbose {
                    println!("🧹 清理僵尸PID文件...");
                }
                Self::cleanup_zombie_pids(&cfg);
            }
        }
    }

    /// 健康检查
    fn health_check(config: &GlobalConfig) {
        // 获取所有运行中的 JAR 进程
        let running_jars = process::get_running_jars();
        
        for (jar_name, pid) in &running_jars {
            if !process::is_process_running(*pid) {
                if config.system.verbose {
                    print_warn!("检测到进程异常: {} (PID: {})", jar_name, pid);
                }
                // 清理无效的PID文件
                process::remove_pid(jar_name);
            }
        }
    }

    /// 检查日志轮转
    fn check_log_rotation(config: &GlobalConfig) {
        let running_jars = process::get_running_jars();
        
        for (jar_name, _) in &running_jars {
            let log_file = config.get_log_file_path(jar_name);
            if config.should_rotate_log(&log_file) {
                if let Err(e) = config.rotate_log(jar_name) {
                    if config.system.verbose {
                        print_warn!("日志轮转失败 {}: {}", jar_name, e);
                    }
                }
            }
        }
    }

    /// 清理僵尸PID文件
    fn cleanup_zombie_pids(config: &GlobalConfig) {
        let running_jars = process::get_running_jars();
        let mut cleaned = 0;
        
        for (jar_name, pid) in &running_jars {
            if !process::is_process_running(*pid) {
                process::remove_pid(jar_name);
                cleaned += 1;
            }
        }
        
        if cleaned > 0 && config.system.verbose {
            print_success!("清理了 {} 个僵尸PID文件", cleaned);
        }
    }

    /// 显示守护进程状态
    pub fn status(&self) {
        println!("{}", "=== Manager_Jar 守护进程状态 ===".bright_blue().bold());
        
        let status = if self.is_running() {
            "运行中".bright_green()
        } else {
            "已停止".bright_red()
        };
        
        println!("状态: {}", status);
        
        if self.is_running() {
            if let Ok(config) = self.config.lock() {
                println!("健康检查间隔: {} 秒", config.process.health_check_interval.to_string().yellow());
                println!("自动功能:");
                println!("  • 进程健康检查: {}", "启用".green());
                println!("  • 日志轮转: {}", if config.log.enable_rotation { "启用".green() } else { "禁用".red() });
                println!("  • 过期日志清理: {}", if config.log.retention_days > 0 { "启用".green() } else { "禁用".red() });
                println!("  • 僵尸PID清理: {}", if config.system.auto_cleanup_pid { "启用".green() } else { "禁用".red() });
            }
        }
    }
}

/// 全局守护进程实例
use std::sync::OnceLock;
#[allow(dead_code)]
static DAEMON_INSTANCE: OnceLock<std::sync::Mutex<ManagerDaemon>> = OnceLock::new();

/// 获取守护进程实例
#[allow(dead_code)]
pub fn get_daemon() -> &'static std::sync::Mutex<ManagerDaemon> {
    DAEMON_INSTANCE.get_or_init(|| {
        std::sync::Mutex::new(ManagerDaemon::new())
    })
}

/// 启动守护进程
#[allow(dead_code)]
pub fn start_daemon() -> Result<(), String> {
    if let Ok(daemon) = get_daemon().lock() {
        daemon.start()
    } else {
        Err("无法获取守护进程实例".to_string())
    }
}

/// 停止守护进程
#[allow(dead_code)]
pub fn stop_daemon() {
    if let Ok(daemon) = get_daemon().lock() {
        daemon.stop();
    }
}

/// 显示守护进程状态
#[allow(dead_code)]
pub fn daemon_status() {
    if let Ok(daemon) = get_daemon().lock() {
        daemon.status();
    }
}

/// 检查守护进程是否运行
#[allow(dead_code)]
pub fn is_daemon_running() -> bool {
    if let Ok(daemon) = get_daemon().lock() {
        daemon.is_running()
    } else {
        false
    }
}
