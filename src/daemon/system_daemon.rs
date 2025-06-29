// system_daemon.rs - 系统级守护进程
use std::process::{Command, Stdio};
use std::fs;
use std::path::PathBuf;
use colored::Colorize;
use crate::core::config::GlobalConfig;
use crate::core::process;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

const DAEMON_PID_FILE: &str = ".Manager_Jar/data/daemon.pid";
const DAEMON_LOG_FILE: &str = ".Manager_Jar/logs/daemon.log";

/// 系统级守护进程管理
pub struct SystemDaemon;

impl SystemDaemon {
    /// 启动系统级守护进程
    pub fn start() -> Result<(), String> {
        // 检查是否已经运行
        if Self::is_running() {
            return Err("守护进程已在运行".to_string());
        }

        // 确保目录存在
        fs::create_dir_all(".Manager_Jar/data").map_err(|e| format!("创建数据目录失败: {}", e))?;
        fs::create_dir_all(".Manager_Jar/logs").map_err(|e| format!("创建日志目录失败: {}", e))?;

        print_success!("正在启动系统级守护进程...");

        // 获取当前程序路径
        let current_exe = std::env::current_exe()
            .map_err(|e| format!("获取程序路径失败: {}", e))?;

        // 启动守护进程
        let child = if cfg!(target_os = "windows") {
            Command::new(&current_exe)
                .args(&["--daemon-mode"])
                .creation_flags(0x08000000 | 0x00000200) // DETACHED_PROCESS | CREATE_NO_WINDOW
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .map_err(|e| format!("启动守护进程失败: {}", e))?
        } else {
            Command::new("nohup")
                .arg(&current_exe)
                .arg("--daemon-mode")
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .map_err(|e| format!("启动守护进程失败: {}", e))?
        };

        // 保存守护进程PID
        let pid = child.id();
        fs::write(DAEMON_PID_FILE, pid.to_string())
            .map_err(|e| format!("保存守护进程PID失败: {}", e))?;

        print_success!("系统级守护进程已启动 (PID: {})", pid);
        println!("  ✅ 进程会在终端关闭后继续运行");
        println!("  ✅ 自动健康检查");
        println!("  ✅ 自动日志轮转");
        println!("  ✅ 自动过期日志清理");
        println!("  ✅ 自动僵尸进程清理");
        println!("  📝 守护进程日志: {}", DAEMON_LOG_FILE.cyan());

        Ok(())
    }

    /// 停止系统级守护进程
    pub fn stop() -> Result<(), String> {
        if let Some(pid) = Self::get_daemon_pid() {
            // 终止守护进程
            if Self::kill_process(pid) {
                // 清理PID文件
                let _ = fs::remove_file(DAEMON_PID_FILE);
                print_success!("系统级守护进程已停止");
                Ok(())
            } else {
                Err(format!("无法停止守护进程 (PID: {})", pid))
            }
        } else {
            Err("守护进程未在运行".to_string())
        }
    }

    /// 检查守护进程是否运行
    pub fn is_running() -> bool {
        if let Some(pid) = Self::get_daemon_pid() {
            Self::is_process_running(pid)
        } else {
            false
        }
    }

    /// 显示守护进程状态
    pub fn status() {
        println!("{}", "=== 系统级守护进程状态 ===".bright_blue().bold());
        
        if let Some(pid) = Self::get_daemon_pid() {
            if Self::is_process_running(pid) {
                println!("状态: {}", "运行中".bright_green());
                println!("进程ID: {}", pid.to_string().yellow());
                println!("日志文件: {}", DAEMON_LOG_FILE.cyan());
                
                // 显示配置信息
                if let Ok(_config_content) = fs::read_to_string(GlobalConfig::config_file_path()) {
                    println!("配置文件: {}", "已加载".green());
                } else {
                    println!("配置文件: {}", "未找到".red());
                }
                
                // 显示最近的守护进程日志
                if PathBuf::from(DAEMON_LOG_FILE).exists() {
                    println!("\n{}:", "最近日志".bright_blue());
                    if let Ok(logs) = Self::get_last_daemon_logs(5) {
                        for log in logs {
                            println!("  {}", log.trim());
                        }
                    }
                }
                
                println!("\n{}:", "自动功能".bright_green());
                println!("  • 进程健康检查: {}", "启用".green());
                println!("  • 日志轮转: {}", "启用".green());
                println!("  • 过期日志清理: {}", "启用".green());
                println!("  • 僵尸PID清理: {}", "启用".green());
                
            } else {
                println!("状态: {}", "已停止".bright_red());
                println!("PID文件存在但进程不存在，正在清理...");
                let _ = fs::remove_file(DAEMON_PID_FILE);
            }
        } else {
            println!("状态: {}", "未启动".bright_red());
            println!("使用 'daemon start' 启动系统级守护进程");
        }
    }

    /// 重启守护进程
    pub fn restart() -> Result<(), String> {
        if Self::is_running() {
            println!("正在停止守护进程...");
            Self::stop()?;
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
        
        println!("正在启动守护进程...");
        Self::start()
    }

    /// 获取守护进程PID
    fn get_daemon_pid() -> Option<u32> {
        fs::read_to_string(DAEMON_PID_FILE)
            .ok()
            .and_then(|content| content.trim().parse::<u32>().ok())
    }

    /// 检查进程是否运行
    fn is_process_running(pid: u32) -> bool {
        if cfg!(target_os = "windows") {
            Command::new("tasklist")
                .args(&["/FI", &format!("PID eq {}", pid)])
                .output()
                .map(|output| {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    output_str.contains(&pid.to_string())
                })
                .unwrap_or(false)
        } else {
            Command::new("ps")
                .args(&["-p", &pid.to_string()])
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
        }
    }

    /// 终止进程
    fn kill_process(pid: u32) -> bool {
        if cfg!(target_os = "windows") {
            Command::new("taskkill")
                .args(&["/PID", &pid.to_string(), "/F"])
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
        } else {
            Command::new("kill")
                .args(&["-9", &pid.to_string()])
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
        }
    }

    /// 获取守护进程最近的日志
    fn get_last_daemon_logs(lines: usize) -> Result<Vec<String>, String> {
        if !PathBuf::from(DAEMON_LOG_FILE).exists() {
            return Ok(vec!["守护进程日志文件不存在".to_string()]);
        }

        let output = if cfg!(target_os = "windows") {
            Command::new("powershell")
                .args(&[
                    "-Command", 
                    &format!("Get-Content '{}' -Tail {} -Encoding UTF8", DAEMON_LOG_FILE, lines)
                ])
                .output()
        } else {
            Command::new("tail")
                .args(&["-n", &lines.to_string(), DAEMON_LOG_FILE])
                .output()
        };

        match output {
            Ok(result) => {
                let content = String::from_utf8_lossy(&result.stdout);
                Ok(content.lines().map(|s| s.to_string()).collect())
            }
            Err(e) => Err(format!("读取守护进程日志失败: {}", e))
        }
    }

    /// 守护进程模式运行
    pub fn run_daemon_mode() {
        // 重定向日志输出到文件
        let mut daemon_log = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(DAEMON_LOG_FILE)
            .expect("无法打开守护进程日志文件");

        use std::io::Write;
        
        writeln!(daemon_log, "[{}] 守护进程启动", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")).ok();
        
        // 主循环
        loop {
            // 加载配置
            let config = GlobalConfig::load();
            
            // 执行定期任务
            Self::perform_maintenance_tasks(&config, &mut daemon_log);
            
            // 等待下次检查
            std::thread::sleep(std::time::Duration::from_secs(config.process.health_check_interval as u64));
        }
    }

    /// 执行维护任务
    fn perform_maintenance_tasks(config: &GlobalConfig, log_file: &mut fs::File) {
        use std::io::Write;
        
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        
        // 1. 健康检查
        let running_jars = process::get_running_jars();
        for (jar_name, pid) in &running_jars {
            if !process::is_process_running(*pid) {
                writeln!(log_file, "[{}] 清理僵尸进程: {} (PID: {})", timestamp, jar_name, pid).ok();
                process::remove_pid(jar_name);
            }
        }

        // 2. 日志轮转
        if config.log.enable_rotation {
            for (jar_name, _) in &running_jars {
                let log_path = config.get_log_file_path(jar_name);
                if config.should_rotate_log(&log_path) {
                    if let Err(e) = config.rotate_log(jar_name) {
                        writeln!(log_file, "[{}] 日志轮转失败 {}: {}", timestamp, jar_name, e).ok();
                    } else {
                        writeln!(log_file, "[{}] 日志轮转成功: {}", timestamp, jar_name).ok();
                    }
                }
            }
        }

        // 3. 清理过期日志
        if config.log.retention_days > 0 {
            if let Ok(cleaned) = config.cleanup_old_logs() {
                if cleaned > 0 {
                    writeln!(log_file, "[{}] 清理了 {} 个过期日志文件", timestamp, cleaned).ok();
                }
            }
        }

        if config.system.verbose {
            writeln!(log_file, "[{}] 维护任务完成", timestamp).ok();
        }
        
        log_file.flush().ok();
    }
}
