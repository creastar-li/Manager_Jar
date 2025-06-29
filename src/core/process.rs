// process.rs - 进程管理模块
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use colored::Colorize;
use std::thread;
use std::time::Duration;
use crate::core::config::GlobalConfig;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

const PID_DIR: &str = ".Manager_Jar/data";

// Windows创建标志常量
#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;
#[cfg(target_os = "windows")]
const DETACHED_PROCESS: u32 = 0x00000008;

// 检查JAR是否正在运行
pub fn is_jar_running(jar_file: &str) -> bool {
    get_pid(jar_file).is_some()
}

// 获取JAR的PID
pub fn get_pid(jar_file: &str) -> Option<u32> {
    let pid_file = PathBuf::from(PID_DIR).join(format!("{}.pid", jar_file.replace(".jar", "")));
    if pid_file.exists() {
        if let Ok(content) = fs::read_to_string(&pid_file) {
            if let Ok(pid) = content.trim().parse::<u32>() {
                if is_process_running(pid) {
                    return Some(pid);
                } else {
                    // 进程已死，清理PID文件
                    fs::remove_file(&pid_file).ok();
                }
            }
        }
    }
    None
}

// 保存JAR的PID
pub fn save_pid(jar_file: &str, pid: u32) {
    fs::create_dir_all(PID_DIR).ok();
    let pid_file = PathBuf::from(PID_DIR).join(format!("{}.pid", jar_file.replace(".jar", "")));
    if let Err(e) = fs::write(&pid_file, pid.to_string()) {
        print_warn!("保存PID文件失败: {}", e);
    }
}

// 移除JAR的PID文件
pub fn remove_pid(jar_file: &str) {
    let pid_file = PathBuf::from(PID_DIR).join(format!("{}.pid", jar_file.replace(".jar", "")));
    fs::remove_file(&pid_file).ok();
}

// 检查进程是否存在
pub fn is_process_running(pid: u32) -> bool {
    if cfg!(target_os = "windows") {
        // Windows: 使用 tasklist 检查进程，同时检查是否是 java 进程
        let output = Command::new("tasklist")
            .args(&["/FI", &format!("PID eq {}", pid), "/NH", "/FO", "CSV"])
            .output();
        match output {
            Ok(result) => {
                let content = String::from_utf8_lossy(&result.stdout);
                // 检查输出中是否包含该PID，并且确保不是"INFO: No tasks"
                !content.trim().is_empty() && 
                !content.contains("INFO: No tasks") &&
                content.lines().any(|line| {
                    // CSV格式：进程名,PID,会话名,会话号,内存使用
                    let parts: Vec<&str> = line.split(',').collect();
                    parts.len() >= 2 && parts[1].trim_matches('"').trim() == pid.to_string()
                })
            }
            Err(_) => {
                // 如果 tasklist 失败，尝试备选方法
                let alt_output = Command::new("cmd")
                    .args(&["/C", &format!("tasklist /FI \"PID eq {}\" | findstr {}", pid, pid)])
                    .output();
                match alt_output {
                    Ok(result) => !String::from_utf8_lossy(&result.stdout).trim().is_empty(),
                    Err(_) => false,
                }
            }
        }
    } else {
        // Linux: 使用 kill -0 检查进程是否存在
        Command::new("kill")
            .args(&["-0", &pid.to_string()])
            .output()
            .map(|out| out.status.success())
            .unwrap_or(false)
    }
}

// 启动JAR应用
pub fn start_jar(jar: &str, args: &[String]) -> Result<(), String> {
    // 检查是否已经在运行
    if let Some(existing_pid) = get_pid(jar) {
        if is_process_running(existing_pid) {
            print_warn!("JAR应用已在运行: {} (PID: {})", jar, existing_pid);
            print_success!("连接到现有进程，无需重新启动");
            
            // 显示现有进程信息
            let config = GlobalConfig::load();
            let log_file = config.get_log_file_path(jar);
            if PathBuf::from(&log_file).exists() {
                println!("  日志文件: {}", log_file.cyan());
            }
            println!("  后台运行: {}", "是".bright_green());
            println!("💡 提示: 使用 'stop' 命令停止，或 'status' 查看详情");
            return Ok(());
        } else {
            // 进程已死，清理PID文件
            print_warn!("检测到僵尸PID文件，正在清理...");
            remove_pid(jar);
        }
    }
    
    // 获取配置
    let config = GlobalConfig::load();
    
    // 确保目录存在
    fs::create_dir_all(&config.log.log_dir).map_err(|e| format!("创建日志目录失败: {}", e))?;
    fs::create_dir_all(PID_DIR).map_err(|e| format!("创建PID目录失败: {}", e))?;
    
    let log_file = config.get_log_file_path(jar);
    
    // 检查是否需要轮转日志
    if config.should_rotate_log(&log_file) {
        if let Err(e) = config.rotate_log(jar) {
            print_warn!("日志轮转失败: {}", e);
        }
    }
    
    drop(config); // 释放锁
    
    print_success!("正在启动 {}...", jar.bright_cyan());
    if !args.is_empty() {
        println!("  启动参数: {}", args.join(" ").yellow());
    }
    
    let child = if cfg!(target_os = "windows") {
        // Windows: 使用特殊标志确保进程独立运行
        let mut cmd = Command::new("java");
        cmd.arg("-jar").arg(jar);
        for arg in args { cmd.arg(arg); }
        
        // 重定向输出到日志文件
        if let Ok(file) = fs::File::create(&log_file) {
            cmd.stdout(Stdio::from(file.try_clone().unwrap()));
            cmd.stderr(Stdio::from(file));
        } else {
            cmd.stdout(Stdio::null());
            cmd.stderr(Stdio::null());
        }
        
        // 设置创建标志：无窗口 + 分离进程
        cmd.creation_flags(CREATE_NO_WINDOW | DETACHED_PROCESS);
        
        cmd.spawn()
    } else {
        // Linux: 使用 nohup 确保后台运行
        let mut cmd = Command::new("nohup");
        cmd.arg("java").arg("-jar").arg(jar);
        for arg in args { cmd.arg(arg); }
        
        // 重定向输出到日志文件
        if let Ok(file) = fs::File::create(&log_file) {
            cmd.stdout(Stdio::from(file.try_clone().unwrap()));
            cmd.stderr(Stdio::from(file));
        } else {
            cmd.stdout(Stdio::null());
            cmd.stderr(Stdio::null());
        }
        
        // 设置进程组，避免信号传播
        cmd.stdin(Stdio::null());
        
        cmd.spawn()
    };
    
    match child {
        Ok(mut process) => {
            let pid = process.id();
            save_pid(jar, pid);
            
            // 等待一小段时间确保进程启动成功
            thread::sleep(Duration::from_millis(500));
            
            // 验证进程是否真的在运行
            if is_process_running(pid) {
                // 立即分离进程，不等待退出
                thread::spawn(move || {
                    let _ = process.wait();
                });
                
                print_success!("✅ 启动成功: {} (PID: {})", jar.bright_cyan(), pid.to_string().bright_green());
                print_success!("🚀 进程已分离，可安全关闭终端");
                println!("  日志文件: {}", log_file.cyan());
                println!("  后台运行: {}", "是".bright_green());
                println!("💡 使用 'status {}' 检查运行状态", jar);
                Ok(())
            } else {
                // 进程启动失败
                remove_pid(jar);
                let error_msg = "进程启动后立即退出，可能是JAR文件损坏或缺少依赖".to_string();
                print_error!("{}", error_msg);
                println!("💡 请检查日志文件: {}", log_file.cyan());
                Err(error_msg)
            }
        },
        Err(e) => {
            let error_msg = format!("启动失败: {}", e);
            print_error!("{}", error_msg);
            println!("💡 请确保:");
            println!("  1. Java 已正确安装并在 PATH 中");
            println!("  2. JAR 文件存在且完整");
            println!("  3. 当前用户有足够权限");
            Err(error_msg)
        }
    }
}

// 启动JAR应用（无额外参数版本）
pub fn start_jar_simple(jar: &str) -> Result<(), String> {
    start_jar(jar, &[])
}

// 快速启动JAR应用
pub fn quick_jar(jar: &str) -> Result<(), String> {
    if is_jar_running(jar) {
        restart_jar(jar, &[])
    } else {
        start_jar_simple(jar)
    }
}

// 停止JAR应用
pub fn stop_jar(jar: &str) -> Result<(), String> {
    if let Some(pid) = get_pid(jar) {
        let result = if cfg!(target_os = "windows") {
            Command::new("taskkill").args(&["/PID", &pid.to_string(), "/F"]).output()
        } else {
            Command::new("kill").args(&["-TERM", &pid.to_string()]).output()
        };
        
        match result {
            Ok(out) if out.status.success() => {
                remove_pid(jar);
                print_success!("已停止 {} (PID: {})", jar.bright_cyan(), pid.to_string().bright_green());
                Ok(())
            },
            Ok(out) => {
                let error_msg = format!("停止失败: {}", String::from_utf8_lossy(&out.stderr));
                print_error!("{}", error_msg);
                // 强制清理PID文件
                remove_pid(jar);
                Err(error_msg)
            },
            Err(e) => {
                let error_msg = format!("停止失败: {}", e);
                print_error!("{}", error_msg);
                remove_pid(jar);
                Err(error_msg)
            }
        }
    } else {
        print_warn!("未找到运行中的进程: {}", jar);
        Ok(())
    }
}

// 重启JAR应用
pub fn restart_jar(jar: &str, args: &[String]) -> Result<(), String> {
    print_success!("正在重启 {}...", jar.bright_cyan());
    
    // 先停止
    if is_jar_running(jar) {
        stop_jar(jar)?;
        thread::sleep(Duration::from_secs(2)); // 等待进程完全结束
    }
    
    // 再启动
    start_jar(jar, args)
}

// 显示JAR状态
pub fn show_jar_status(jar: Option<&str>) {
    if let Some(jar) = jar {
        println!("\n{}", format!("=== {} 状态信息 ===", jar).bright_blue().bold());
        
        if let Some(pid) = get_pid(jar) {
            if is_process_running(pid) {
                print_success!("✅ {} 正在运行", jar.bright_cyan());
                println!("  进程ID: {}", pid.to_string().bright_green());
                
                // 显示日志信息
                let config = GlobalConfig::load();
                let log_file = config.get_log_file_path(jar);
                if PathBuf::from(&log_file).exists() {
                    println!("  日志文件: {}", log_file.cyan());
                    if let Ok(metadata) = fs::metadata(&log_file) {
                        println!("  日志大小: {}", format_file_size(metadata.len()).bright_blue());
                        if config.should_rotate_log(&log_file) {
                            println!("  轮转状态: {}", "需要轮转".yellow());
                        }
                    }
                } else {
                    println!("  日志文件: {}", "尚未创建".yellow());
                }
                
                // 显示配置信息
                let config_file = format!(".Manager_Jar/configs/{}.config", jar.replace(".jar", ""));
                if PathBuf::from(&config_file).exists() {
                    if let Ok(config) = fs::read_to_string(&config_file) {
                        if !config.trim().is_empty() {
                            println!("  保存配置: {}", config.trim().yellow());
                        }
                    }
                } else {
                    println!("  保存配置: {}", "无".bright_black());
                }
                
                println!("  运行模式: {}", "后台分离".bright_green());
                println!("  终端安全: {}", "可关闭".bright_green());
                
                // 尝试获取更多进程信息（Windows）
                if cfg!(target_os = "windows") {
                    let output = Command::new("tasklist")
                        .args(&["/FI", &format!("PID eq {}", pid), "/FO", "CSV", "/NH"])
                        .output();
                    if let Ok(result) = output {
                        let content = String::from_utf8_lossy(&result.stdout);
                        if !content.trim().is_empty() && !content.contains("INFO: No tasks") {
                            for line in content.lines() {
                                let parts: Vec<&str> = line.split(',').collect();
                                if parts.len() >= 5 {
                                    let memory = parts[4].trim_matches('"').replace(" K", "");
                                    if let Ok(mem_kb) = memory.replace(",", "").parse::<u64>() {
                                        println!("  内存使用: {}", format_file_size(mem_kb * 1024).bright_blue());
                                    }
                                }
                            }
                        }
                    }
                }
                
            } else {
                print_warn!("⚠️  {} 进程已停止", jar.bright_cyan());
                println!("  最后PID: {} (进程已结束)", pid.to_string().bright_black());
                println!("  状态: {}", "僵尸PID文件".red());
                
                // 清理僵尸PID文件
                remove_pid(jar);
                print_success!("已自动清理僵尸PID文件");
            }
        } else {
            print_warn!("❌ {} 未在运行", jar.bright_cyan());
            println!("  状态: {}", "已停止".red());
            println!("  PID文件: {}", "不存在".bright_black());
        }
        
        println!("\n💡 可用命令:");
        if get_pid(jar).is_some() && is_jar_running(jar) {
            println!("  {} - 停止应用", format!("stop {}", jar).cyan());
            println!("  {} - 重启应用", format!("restart {}", jar).cyan());
            println!("  {} - 查看日志", format!("log {}", jar).cyan());
            println!("  {} - 强制终止", format!("kill {}", jar).cyan());
        } else {
            println!("  {} - 启动应用", format!("start {}", jar).cyan());
            if PathBuf::from(&format!(".Manager_Jar/configs/{}.config", jar.replace(".jar", ""))).exists() {
                println!("  {} - 快速启动", format!("quick {}", jar).cyan());
            }
        }
        
    } else {
        println!("\n{}", "=== JAR 应用状态概览 ===".bright_blue().bold());
        list_running_jars();
    }
}

// 获取所有运行中的JAR列表（返回数据而不是打印）
pub fn get_running_jars() -> Vec<(String, u32)> {
    let mut running_jars = Vec::new();
    let mut zombie_pids = Vec::new();
    
    if let Ok(entries) = fs::read_dir(PID_DIR) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.ends_with(".pid") {
                    let jar_name = format!("{}.jar", name.trim_end_matches(".pid"));
                    if let Some(pid) = get_pid(&jar_name) {
                        if is_process_running(pid) {
                            running_jars.push((jar_name, pid));
                        } else {
                            zombie_pids.push((jar_name, pid));
                        }
                    }
                }
            }
        }
    }
    
    // 静默清理僵尸PID文件（守护模式下不输出）
    for (jar, _) in &zombie_pids {
        remove_pid(jar);
    }
    
    running_jars
}

// 列出所有运行中的JAR
pub fn list_running_jars() {
    let mut running_jars = Vec::new();
    let mut zombie_pids = Vec::new();
    
    if let Ok(entries) = fs::read_dir(PID_DIR) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.ends_with(".pid") {
                    let jar_name = format!("{}.jar", name.trim_end_matches(".pid"));
                    if let Some(pid) = get_pid(&jar_name) {
                        if is_process_running(pid) {
                            running_jars.push((jar_name, pid));
                        } else {
                            zombie_pids.push((jar_name, pid));
                        }
                    }
                }
            }
        }
    }
    
    // 清理僵尸PID文件
    if !zombie_pids.is_empty() {
        print_warn!("发现 {} 个僵尸PID文件，正在清理...", zombie_pids.len());
        for (jar, _) in &zombie_pids {
            remove_pid(jar);
        }
        print_success!("已清理僵尸PID文件");
    }
    
    if running_jars.is_empty() {
        print_warn!("🔍 无正在运行的 JAR 应用");
        
        // 显示可用的JAR文件
        let available_jars = crate::utils::files::list_available_jars();
        if !available_jars.is_empty() {
            println!("\n📁 当前目录可用的JAR文件:");
            for jar in available_jars {
                println!("  {} [{}]", jar.cyan(), "已停止".red());
            }
            println!("\n💡 使用 'start <jar>' 启动应用");
        } else {
            println!("\n💡 当前目录下无可用的JAR文件");
        }
    } else {
        print_success!("🚀 运行中的 JAR 应用 ({} 个):", running_jars.len());
        let config = GlobalConfig::load();
        for (jar, pid) in running_jars {
            let log_file = config.get_log_file_path(&jar);
            let log_size = if let Ok(metadata) = fs::metadata(&log_file) {
                let size_info = format_file_size(metadata.len());
                let rotation_hint = if config.should_rotate_log(&log_file) {
                    " [需要轮转]".yellow().to_string()
                } else {
                    String::new()
                };
                format!(" | 日志: {}{}", size_info, rotation_hint)
            } else {
                String::new()
            };
            
            println!("  {} (PID: {}) [{}{}]", 
                     jar.bright_cyan(), 
                     pid.to_string().bright_green(),
                     "后台运行".bright_blue(),
                     log_size.bright_black());
        }
        println!("\n💡 提示:");
        println!("  • 这些进程在终端关闭后仍会继续运行");
        println!("  • 使用 'status <jar>' 查看详细信息");
        println!("  • 使用 'stop <jar>' 停止指定应用");
    }
}

// 强制杀死JAR进程
pub fn kill_jar(jar: &str) -> Result<(), String> {
    if let Some(pid) = get_pid(jar) {
        let result = if cfg!(target_os = "windows") {
            Command::new("taskkill").args(&["/PID", &pid.to_string(), "/F"]).output()
        } else {
            Command::new("kill").args(&["-9", &pid.to_string()]).output()
        };
        
        match result {
            Ok(_) => {
                remove_pid(jar);
                print_success!("已强制杀死 {} (PID: {})", jar.bright_cyan(), pid.to_string().bright_green());
                Ok(())
            },
            Err(e) => {
                let error_msg = format!("强制杀死失败: {}", e);
                print_error!("{}", error_msg);
                remove_pid(jar); // 清理PID文件
                Err(error_msg)
            }
        }
    } else {
        print_warn!("未找到运行中的进程: {}", jar);
        Ok(())
    }
}

// 获取JAR状态
#[allow(dead_code)]
pub fn get_jar_status(jar: &str) -> Result<JarStatus, String> {
    let jar_file = if jar.ends_with(".jar") { jar } else { &format!("{}.jar", jar) };
    let pid_file = PathBuf::from(PID_DIR).join(format!("{}.pid", jar_file.replace(".jar", "")));
    
    if !pid_file.exists() {
        return Ok(JarStatus {
            is_running: false,
            pid: None,
            uptime: None,
            memory_usage: None,
        });
    }

    if let Ok(pid_str) = fs::read_to_string(&pid_file) {
        if let Ok(pid) = pid_str.trim().parse::<u32>() {
            let is_running = is_process_running(pid);
            return Ok(JarStatus {
                is_running,
                pid: Some(pid),
                uptime: if is_running { Some("运行中".to_string()) } else { None },
                memory_usage: None,
            });
        }
    }

    Ok(JarStatus {
        is_running: false,
        pid: None,
        uptime: None,
        memory_usage: None,
    })
}

// JAR状态结构体
#[derive(Debug)]
#[allow(dead_code)]
pub struct JarStatus {
    pub is_running: bool,
    pub pid: Option<u32>,
    pub uptime: Option<String>,
    pub memory_usage: Option<String>,
}

// 格式化文件大小
fn format_file_size(size: u64) -> String {
    if size > 1024 * 1024 {
        format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
    } else if size > 1024 {
        format!("{:.1} KB", size as f64 / 1024.0)
    } else {
        format!("{} B", size)
    }
}
