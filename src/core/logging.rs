// logging.rs - 日志管理模块
use std::process::Command;
use std::path::Path;
use std::fs;
use colored::Colorize;
use crate::core::config::GlobalConfig;

// 显示JAR日志 - 直接实现，供 main.rs 调用
pub fn show_jar_log(jar: &str, lines: u32) {
    let config = GlobalConfig::load();
    let log_file = config.get_log_file_path(jar);
    let log_path = Path::new(&log_file);
    
    // 检查是否需要轮转日志
    if config.should_rotate_log(&log_file) {
        drop(config); // 释放锁
        if let Err(e) = GlobalConfig::load().rotate_log(jar) {
            print_warn!("日志轮转失败: {}", e);
        }
    } else {
        drop(config); // 释放锁
    }
    
    if !log_path.exists() {
        print_warn!("日志文件不存在: {}", log_file);
        println!("提示: JAR 应用启动后会自动创建日志文件");
        return;
    }
    
    // 检查文件大小
    if let Ok(metadata) = fs::metadata(&log_file) {
        let size = metadata.len();
        if size == 0 {
            print_warn!("日志文件为空: {}", log_file);
            return;
        }
        println!("📄 日志文件: {} (大小: {} 字节)", log_file.cyan(), size.to_string().bright_green());
    }
    
    println!("📖 显示最后 {} 行日志:", lines.to_string().bright_green());
    println!("{}", "─".repeat(80).bright_blue());
    
    let output = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args(&[
                "-Command", 
                &format!("Get-Content '{}' -Tail {} -Encoding UTF8", log_file, lines)
            ])
            .output()
    } else {
        Command::new("tail")
            .args(&["-n", &lines.to_string(), &log_file])
            .output()
    };
    
    match output {
        Ok(result) => {
            let content = String::from_utf8_lossy(&result.stdout);
            if content.trim().is_empty() {
                print_warn!("日志内容为空或无法读取");
            } else {
                // 简单的日志高亮
                for line in content.lines() {
                    if line.contains("ERROR") || line.contains("Exception") || line.contains("Failed") {
                        println!("{}", line.red());
                    } else if line.contains("WARN") || line.contains("WARNING") {
                        println!("{}", line.yellow());
                    } else if line.contains("INFO") || line.contains("Started") || line.contains("Success") {
                        println!("{}", line.green());
                    } else if line.contains("DEBUG") {
                        println!("{}", line.bright_black());
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
        Err(e) => {
            print_error!("读取日志文件失败: {}", e);
            println!("提示: 请确保有足够的权限读取日志文件");
        }
    }
    
    println!("{}", "─".repeat(80).bright_blue());
    println!("💡 提示: 使用 {} 实时查看日志", "tail -f".bright_cyan());
}

// 清理日志文件
pub fn clean_logs() -> Result<(), String> {
    let config = GlobalConfig::load();
    let log_dir = &config.log.log_dir;
    
    if let Ok(entries) = fs::read_dir(log_dir) {
        let mut cleaned = 0;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("log") {
                if fs::remove_file(&path).is_ok() {
                    cleaned += 1;
                }
            }
        }
        print_success!("已清理 {} 个日志文件", cleaned);
        Ok(())
    } else {
        let error_msg = format!("无法访问日志目录: {}", log_dir);
        print_error!("{}", error_msg);
        Err(error_msg)
    }
}

// 列出所有日志文件
pub fn list_logs() {
    let config = GlobalConfig::load();
    let log_dir = &config.log.log_dir;
    
    if let Ok(entries) = fs::read_dir(log_dir) {
        let mut logs = Vec::new();
        
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("log") {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if let Ok(metadata) = fs::metadata(&path) {
                        logs.push((name.to_string(), metadata.len()));
                    }
                }
            }
        }
        
        if logs.is_empty() {
            print_warn!("无日志文件");
        } else {
            print_success!("发现 {} 个日志文件:", logs.len());
            logs.sort_by(|a, b| a.0.cmp(&b.0));
            for (name, size) in logs {
                let size_str = if size > 1024 * 1024 {
                    format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
                } else if size > 1024 {
                    format!("{:.1} KB", size as f64 / 1024.0)
                } else {
                    format!("{} B", size)
                };
                
                // 检查是否需要轮转
                let full_path = format!("{}/{}", log_dir, name);
                let needs_rotation = config.should_rotate_log(&full_path);
                let rotation_hint = if needs_rotation {
                    format!(" {}", "[需要轮转]".yellow())
                } else {
                    String::new()
                };
                
                println!("  {} ({}){}", name.cyan(), size_str.bright_green(), rotation_hint);
            }
            
            println!("\n💡 提示:");
            println!("  • 日志保存天数: {} 天", config.log.retention_days.to_string().yellow());
            println!("  • 文件大小限制: {} MB", config.log.max_file_size_mb.to_string().yellow());
            println!("  • 轮转功能: {}", if config.log.enable_rotation { "启用".green() } else { "禁用".red() });
        }
    } else {
        print_warn!("无法访问日志目录: {}", log_dir);
    }
}
