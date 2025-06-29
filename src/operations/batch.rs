// batch.rs - 批量/序列操作相关
use colored::Colorize;
use std::fs;
use std::thread;
use std::time::Duration;

use crate::core::process::{start_jar_simple, stop_jar, kill_jar, quick_jar, get_jar_status};

#[allow(dead_code)]
const SEQUENCE_DIR: &str = ".Manager_Jar/sequences";

// 批量操作分发
pub fn batch_operation(op: &str, jars: &[String]) -> Result<(), String> {
    if jars.is_empty() {
        return Err("没有指定JAR文件".to_string());
    }

    println!(
        "\n{} {} ({} 个文件)",
        "=== 批量操作".bright_blue().bold(),
        op.bright_cyan(),
        jars.len().to_string().bright_green()
    );

    match op {
        "start" => {
            batch_start(jars);
            Ok(())
        }
        "stop" => {
            batch_stop(jars);
            Ok(())
        }
        "quick" => {
            batch_quick(jars);
            Ok(())
        }
        "restart" => {
            batch_restart(jars);
            Ok(())
        }
        "kill" => {
            batch_kill(jars);
            Ok(())
        }
        _ => {
            print_error!("不支持的批量操作: {}", op);
            println!("支持的操作: start, stop, restart, quick, kill");
            Err("不支持的批量操作".to_string())
        }
    }
}

// 批量启动
fn batch_start(jars: &[String]) {
    let mut success_count = 0;
    let mut fail_count = 0;

    for (i, jar) in jars.iter().enumerate() {
        println!(
            "\n{} {}/{} {}",
            "[启动]".bright_green(),
            (i + 1).to_string().bright_yellow(),
            jars.len().to_string().bright_yellow(),
            jar.bright_white()
        );

        match start_jar_simple(jar) {
            Ok(_) => {
                success_count += 1;
                print_success!("启动成功: {}", jar);
            }
            Err(e) => {
                fail_count += 1;
                print_error!("启动失败: {} - {}", jar, e);
            }
        }

        // 批量操作间隔
        if i < jars.len() - 1 {
            thread::sleep(Duration::from_millis(500));
        }
    }

    println!(
        "\n{} 成功: {} | 失败: {}",
        "批量启动完成".bright_blue().bold(),
        success_count.to_string().bright_green(),
        fail_count.to_string().bright_red()
    );
}

// 批量停止
fn batch_stop(jars: &[String]) {
    let mut success_count = 0;
    let mut fail_count = 0;

    for (i, jar) in jars.iter().enumerate() {
        println!(
            "\n{} {}/{} {}",
            "[停止]".bright_red(),
            (i + 1).to_string().bright_yellow(),
            jars.len().to_string().bright_yellow(),
            jar.bright_white()
        );

        match stop_jar(jar) {
            Ok(_) => {
                success_count += 1;
                print_success!("停止成功: {}", jar);
            }
            Err(e) => {
                fail_count += 1;
                print_error!("停止失败: {} - {}", jar, e);
            }
        }

        if i < jars.len() - 1 {
            thread::sleep(Duration::from_millis(300));
        }
    }

    println!(
        "\n{} 成功: {} | 失败: {}",
        "批量停止完成".bright_blue().bold(),
        success_count.to_string().bright_green(),
        fail_count.to_string().bright_red()
    );
}

// 批量重启
fn batch_restart(jars: &[String]) {
    println!("{}", "开始批量重启...".bright_blue());

    // 先停止所有
    batch_stop(jars);
    
    println!("\n{}", "等待进程完全停止...".bright_yellow());
    thread::sleep(Duration::from_secs(2));
    
    // 再启动所有
    batch_start(jars);
}

// 批量快速操作
fn batch_quick(jars: &[String]) {
    let mut success_count = 0;
    let mut fail_count = 0;

    for (i, jar) in jars.iter().enumerate() {
        println!(
            "\n{} {}/{} {}",
            "[快速]".bright_cyan(),
            (i + 1).to_string().bright_yellow(),
            jars.len().to_string().bright_yellow(),
            jar.bright_white()
        );

        match quick_jar(jar) {
            Ok(_) => {
                success_count += 1;
                print_success!("快速操作成功: {}", jar);
            }
            Err(e) => {
                fail_count += 1;
                print_error!("快速操作失败: {} - {}", jar, e);
            }
        }

        if i < jars.len() - 1 {
            thread::sleep(Duration::from_millis(200));
        }
    }

    println!(
        "\n{} 成功: {} | 失败: {}",
        "批量快速操作完成".bright_blue().bold(),
        success_count.to_string().bright_green(),
        fail_count.to_string().bright_red()
    );
}

// 批量强制终止
fn batch_kill(jars: &[String]) {
    let mut success_count = 0;
    let mut fail_count = 0;

    for (i, jar) in jars.iter().enumerate() {
        println!(
            "\n{} {}/{} {}",
            "[强杀]".bright_red().bold(),
            (i + 1).to_string().bright_yellow(),
            jars.len().to_string().bright_yellow(),
            jar.bright_white()
        );

        match kill_jar(jar) {
            Ok(_) => {
                success_count += 1;
                print_success!("强制终止成功: {}", jar);
            }
            Err(e) => {
                fail_count += 1;
                print_error!("强制终止失败: {} - {}", jar, e);
            }
        }

        if i < jars.len() - 1 {
            thread::sleep(Duration::from_millis(100));
        }
    }

    println!(
        "\n{} 成功: {} | 失败: {}",
        "批量强制终止完成".bright_blue().bold(),
        success_count.to_string().bright_green(),
        fail_count.to_string().bright_red()
    );
}

// 创建序列
#[allow(dead_code)]
pub fn create_sequence(name: &str, jars: &[String]) -> Result<(), String> {
    if jars.is_empty() {
        return Err("序列不能为空".to_string());
    }

    // 确保序列目录存在
    if let Some(home) = dirs::home_dir() {
        let seq_dir = home.join(SEQUENCE_DIR);
        if !seq_dir.exists() {
            fs::create_dir_all(&seq_dir).map_err(|e| format!("创建序列目录失败: {}", e))?;
        }

        let seq_file = seq_dir.join(format!("{}.seq", name));
        let content = jars.join("\n");
        
        fs::write(&seq_file, content).map_err(|e| format!("写入序列文件失败: {}", e))?;
        
        print_success!("序列已创建: {} ({} 个JAR)", name, jars.len());
        println!("序列文件: {}", seq_file.display());
        
        Ok(())
    } else {
        Err("无法获取用户主目录".to_string())
    }
}

// 列出所有序列
#[allow(dead_code)]
pub fn list_sequences() -> Result<(), String> {
    if let Some(home) = dirs::home_dir() {
        let seq_dir = home.join(SEQUENCE_DIR);
        
        if !seq_dir.exists() {
            println!("{}", "没有找到序列目录".bright_yellow());
            return Ok(());
        }

        let entries = fs::read_dir(&seq_dir).map_err(|e| format!("读取序列目录失败: {}", e))?;
        
        let mut sequences = Vec::new();
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("seq") {
                    if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                        // 读取序列内容
                        if let Ok(content) = fs::read_to_string(&path) {
                            let jar_count = content.lines().filter(|line| !line.trim().is_empty()).count();
                            sequences.push((name.to_string(), jar_count));
                        }
                    }
                }
            }
        }

        if sequences.is_empty() {
            println!("{}", "没有找到序列文件".bright_yellow());
        } else {
            println!("{}", "可用序列:".bright_blue().bold());
            for (name, count) in sequences {
                println!("  {} {} {} JAR",
                    "•".bright_green(),
                    name.bright_white(),
                    format!("({} 个)", count).bright_cyan()
                );
            }
        }
        
        Ok(())
    } else {
        Err("无法获取用户主目录".to_string())
    }
}

// 删除序列
#[allow(dead_code)]
pub fn delete_sequence(name: &str) -> Result<(), String> {
    if let Some(home) = dirs::home_dir() {
        let seq_file = home.join(SEQUENCE_DIR).join(format!("{}.seq", name));
        
        if !seq_file.exists() {
            return Err(format!("序列不存在: {}", name));
        }

        fs::remove_file(&seq_file).map_err(|e| format!("删除序列文件失败: {}", e))?;
        
        print_success!("序列已删除: {}", name);
        Ok(())
    } else {
        Err("无法获取用户主目录".to_string())
    }
}

// 显示序列内容
#[allow(dead_code)]
pub fn show_sequence(name: &str) -> Result<(), String> {
    if let Some(home) = dirs::home_dir() {
        let seq_file = home.join(SEQUENCE_DIR).join(format!("{}.seq", name));
        
        if !seq_file.exists() {
            return Err(format!("序列不存在: {}", name));
        }

        let content = fs::read_to_string(&seq_file).map_err(|e| format!("读取序列文件失败: {}", e))?;
        let jars: Vec<&str> = content.lines().filter(|line| !line.trim().is_empty()).collect();
        
        println!("{} {}", "序列:".bright_blue().bold(), name.bright_white());
        println!("{} {} 个JAR", "包含:".bright_cyan(), jars.len().to_string().bright_green());
        
        for (i, jar) in jars.iter().enumerate() {
            println!("  {}. {}", 
                (i + 1).to_string().bright_yellow(),
                jar.bright_white()
            );
        }
        
        Ok(())
    } else {
        Err("无法获取用户主目录".to_string())
    }
}

// 执行序列
#[allow(dead_code)]
pub fn run_sequence(name: &str, operation: &str) -> Result<(), String> {
    if let Some(home) = dirs::home_dir() {
        let seq_file = home.join(SEQUENCE_DIR).join(format!("{}.seq", name));
        
        if !seq_file.exists() {
            return Err(format!("序列不存在: {}", name));
        }

        let content = fs::read_to_string(&seq_file).map_err(|e| format!("读取序列文件失败: {}", e))?;
        let jars: Vec<String> = content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|s| s.to_string())
            .collect();
        
        if jars.is_empty() {
            return Err("序列为空".to_string());
        }

        println!("{} {} {} {}",
            "执行序列:".bright_blue().bold(),
            name.bright_white(),
            "操作:".bright_cyan(),
            operation.bright_green()
        );

        batch_operation(operation, &jars)
    } else {
        Err("无法获取用户主目录".to_string())
    }
}

// 检查序列中JAR的状态
#[allow(dead_code)]
pub fn check_sequence_status(name: &str) -> Result<(), String> {
    if let Some(home) = dirs::home_dir() {
        let seq_file = home.join(SEQUENCE_DIR).join(format!("{}.seq", name));
        
        if !seq_file.exists() {
            return Err(format!("序列不存在: {}", name));
        }

        let content = fs::read_to_string(&seq_file).map_err(|e| format!("读取序列文件失败: {}", e))?;
        let jars: Vec<&str> = content.lines().filter(|line| !line.trim().is_empty()).collect();
        
        if jars.is_empty() {
            return Err("序列为空".to_string());
        }

        println!("{} {}", "序列状态:".bright_blue().bold(), name.bright_white());
        
        let mut running_count = 0;
        let mut stopped_count = 0;
        
        for jar in &jars {
            match get_jar_status(jar) {
                Ok(status) => {
                    let status_display = if status.is_running {
                        running_count += 1;
                        "运行中".bright_green()
                    } else {
                        stopped_count += 1;
                        "已停止".bright_red()
                    };
                    
                    println!("  {} {} {} {}",
                        "•".bright_blue(),
                        jar.bright_white(),
                        "-".bright_black(),
                        status_display
                    );
                }
                Err(e) => {
                    stopped_count += 1;
                    println!("  {} {} {} {}",
                        "•".bright_blue(),
                        jar.bright_white(),
                        "-".bright_black(),
                        format!("错误: {}", e).bright_red()
                    );
                }
            }
        }
        
        println!("\n{} 运行: {} | 停止: {}",
            "总计:".bright_blue().bold(),
            running_count.to_string().bright_green(),
            stopped_count.to_string().bright_red()
        );
        
        Ok(())
    } else {
        Err("无法获取用户主目录".to_string())
    }
}
