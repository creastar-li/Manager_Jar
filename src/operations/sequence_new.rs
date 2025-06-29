// operations/sequence.rs - 序列操作模块

use colored::Colorize;
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use crate::core::process::*;
use crate::core::config::*;
use crate::utils::files::*;

const SEQUENCE_DIR: &str = ".Manager_Jar/sequences";

/// 序列操作分发
pub fn sequence_operation(op: &str, args: &[String]) -> Result<(), String> {
    fs::create_dir_all(SEQUENCE_DIR).ok();

    println!(
        "\n{} {}",
        "=== 序列操作".bright_blue().bold(),
        op.bright_cyan()
    );

    match op {
        "create" => create_sequence(args),
        "start" => start_sequence(args),
        "stop" => stop_sequence(args),
        "restart" => restart_sequence(args),
        "list" => {
            list_sequences();
            Ok(())
        }
        "show" => show_sequence(args),
        "delete" => delete_sequence(args),
        _ => {
            print_error!("不支持的序列操作: {}", op);
            println!("支持的操作: create, start, stop, restart, list, show, delete");
            Err("不支持的序列操作".to_string())
        }
    }
}

/// 创建序列
fn create_sequence(args: &[String]) -> Result<(), String> {
    if args.len() < 2 {
        print_error!("用法: sequence create <序列名> <jar1> [jar2] [jar3] ...");
        return Err("参数不足".to_string());
    }

    let sequence_name = &args[0];
    let jars = &args[1..];

    // 验证所有JAR文件是否存在
    for jar in jars {
        if !validate_jar_file(jar) {
            return Err(format!("JAR文件不存在: {}", jar));
        }
    }

    let sequence_file = PathBuf::from(SEQUENCE_DIR).join(format!("{}.seq", sequence_name));
    let content = jars.join("\n");

    fs::write(&sequence_file, content).map_err(|e| format!("创建序列失败: {}", e))?;

    print_success!("序列 {} 已创建", sequence_name.bright_cyan());
    println!("  包含 {} 个JAR:", jars.len().to_string().bright_green());
    for (i, jar) in jars.iter().enumerate() {
        println!(
            "    {}. {}",
            (i + 1).to_string().bright_blue(),
            jar.yellow()
        );
    }
    println!(
        "  文件: {}",
        sequence_file.display().to_string().cyan()
    );

    Ok(())
}

/// 启动序列
fn start_sequence(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        print_error!("用法: sequence start <序列名>");
        return Err("缺少序列名".to_string());
    }

    let sequence_name = &args[0];
    let jars = load_sequence(sequence_name)?;

    print_success!("开始按序列启动: {}", sequence_name.bright_cyan());
    println!("  包含 {} 个JAR", jars.len().to_string().bright_green());

    let mut success_count = 0;
    let mut failed_count = 0;

    for (index, jar) in jars.iter().enumerate() {
        println!(
            "\n🔹 [{}/{}] 启动: {}",
            (index + 1).to_string().bright_green(),
            jars.len().to_string().bright_blue(),
            jar.bright_cyan()
        );

        if is_jar_running(jar) {
            print_warn!("应用已在运行，跳过");
            continue;
        }

        match start_jar(jar) {
            Ok(_) => {
                success_count += 1;
                thread::sleep(Duration::from_secs(2)); // 序列启动间隔
            }
            Err(e) => {
                print_error!("启动失败: {}", e);
                failed_count += 1;
                
                println!("是否继续启动剩余应用? (y/N)");
                use std::io::Write;
                print!("> ");
                std::io::stdout().flush().ok();
                
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).ok();
                
                if !input.trim().to_lowercase().starts_with('y') {
                    print_warn!("用户取消序列启动");
                    break;
                }
            }
        }
    }

    println!("\n{}", "=== 序列启动完成 ===".bright_blue().bold());
    println!("  成功: {} 个", success_count.to_string().bright_green());
    if failed_count > 0 {
        println!("  失败: {} 个", failed_count.to_string().bright_red());
    }

    Ok(())
}

/// 停止序列
fn stop_sequence(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        print_error!("用法: sequence stop <序列名>");
        return Err("缺少序列名".to_string());
    }

    let sequence_name = &args[0];
    let jars = load_sequence(sequence_name)?;

    print_success!("开始按序列停止: {}", sequence_name.bright_cyan());
    println!("  包含 {} 个JAR", jars.len().to_string().bright_green());

    let mut success_count = 0;
    let mut failed_count = 0;

    // 反向停止（后启动的先停止）
    for (index, jar) in jars.iter().rev().enumerate() {
        println!(
            "\n🔸 [{}/{}] 停止: {}",
            (index + 1).to_string().bright_green(),
            jars.len().to_string().bright_blue(),
            jar.bright_cyan()
        );

        if !is_jar_running(jar) {
            print_warn!("应用未在运行，跳过");
            continue;
        }

        match stop_jar(jar) {
            Ok(_) => {
                success_count += 1;
                thread::sleep(Duration::from_secs(1)); // 停止间隔
            }
            Err(e) => {
                print_error!("停止失败: {}", e);
                failed_count += 1;
            }
        }
    }

    println!("\n{}", "=== 序列停止完成 ===".bright_blue().bold());
    println!("  成功: {} 个", success_count.to_string().bright_green());
    if failed_count > 0 {
        println!("  失败: {} 个", failed_count.to_string().bright_red());
    }

    Ok(())
}

/// 重启序列
fn restart_sequence(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        print_error!("用法: sequence restart <序列名>");
        return Err("缺少序列名".to_string());
    }

    let sequence_name = &args[0];
    
    print_success!("重启序列: {}", sequence_name.bright_cyan());
    
    // 先停止
    println!("\n🔄 第一步: 停止序列");
    if let Err(e) = stop_sequence(args) {
        print_warn!("停止序列时出现错误: {}", e);
    }
    
    // 等待一段时间
    println!("\n⏳ 等待 3 秒..");
    thread::sleep(Duration::from_secs(3));
    
    // 再启动
    println!("\n🔄 第二步: 启动序列");
    start_sequence(args)
}

/// 列出所有序列
fn list_sequences() {
    if let Ok(entries) = fs::read_dir(SEQUENCE_DIR) {
        let mut sequences = Vec::new();
        
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("seq") {
                if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                    if let Ok(content) = fs::read_to_string(&path) {
                        let jar_count = content.lines().filter(|line| !line.trim().is_empty()).count();
                        sequences.push((name.to_string(), jar_count));
                    }
                }
            }
        }
        
        if sequences.is_empty() {
            print_warn!("无已定义的序列");
            println!("💡 使用 'sequence create <名称> <jar1> <jar2>...' 创建序列");
        } else {
            print_success!("已定义的序列 ({} 个):", sequences.len());
            sequences.sort_by(|a, b| a.0.cmp(&b.0));
            for (name, count) in sequences {
                println!("  {} ({} 个JAR)", name.cyan(), count.to_string().yellow());
            }
        }
    } else {
        print_warn!("无法访问序列目录: {}", SEQUENCE_DIR);
    }
}

/// 显示序列详情
fn show_sequence(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        print_error!("用法: sequence show <序列名>");
        return Err("缺少序列名".to_string());
    }

    let sequence_name = &args[0];
    let jars = load_sequence(sequence_name)?;

    print_success!("序列详情: {}", sequence_name.bright_cyan());
    println!("  包含 {} 个JAR:", jars.len().to_string().bright_green());
    
    for (i, jar) in jars.iter().enumerate() {
        let status = if is_jar_running(jar) {
            "运行中".bright_green()
        } else {
            "已停止".bright_red()
        };
        
        println!(
            "    {}. {} [{}]",
            (i + 1).to_string().bright_blue(),
            jar.yellow(),
            status
        );
    }

    Ok(())
}

/// 删除序列
fn delete_sequence(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        print_error!("用法: sequence delete <序列名>");
        return Err("缺少序列名".to_string());
    }

    let sequence_name = &args[0];
    let sequence_file = PathBuf::from(SEQUENCE_DIR).join(format!("{}.seq", sequence_name));

    if !sequence_file.exists() {
        print_error!("序列不存在: {}", sequence_name);
        return Err("序列不存在".to_string());
    }

    fs::remove_file(&sequence_file).map_err(|e| format!("删除序列失败: {}", e))?;
    print_success!("序列 {} 已删除", sequence_name.bright_cyan());

    Ok(())
}

/// 加载序列文件内容
fn load_sequence(sequence_name: &str) -> Result<Vec<String>, String> {
    let sequence_file = PathBuf::from(SEQUENCE_DIR).join(format!("{}.seq", sequence_name));

    if !sequence_file.exists() {
        print_error!("序列不存在: {}", sequence_name);
        return Err("序列不存在".to_string());
    }

    let content = fs::read_to_string(&sequence_file).map_err(|e| format!("读取序列失败: {}", e))?;
    let jars: Vec<String> = content
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if jars.is_empty() {
        print_warn!("序列 {} 为空", sequence_name);
        return Err("序列为空".to_string());
    }

    Ok(jars)
}
