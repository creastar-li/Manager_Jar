// utils/files.rs - 文件和目录管理工具

use std::fs;
use std::path::Path;

/// 初始化必要的目录结构
pub fn init_directories() {
    let dirs = [
        ".Manager_Jar",
        ".Manager_Jar/logs",
        ".Manager_Jar/configs", 
        ".Manager_Jar/sequences",
        ".Manager_Jar/data"
    ];
    
    for dir in &dirs {
        if let Err(e) = fs::create_dir_all(dir) {
            print_warn!("创建目录失败 {}: {}", dir, e);
        }
    }
}

/// 校验JAR文件名是否合法
pub fn validate_jar_file(jar_file: &str) -> bool {
    let path = Path::new(jar_file);
    let valid = path.exists() && path.is_file() && jar_file.ends_with(".jar");
    
    if !valid {
        if !path.exists() {
            print_error!("文件不存在: {}", jar_file.cyan());
        } else if !path.is_file() {
            print_error!("不是有效文件: {}", jar_file.cyan());
        } else {
            print_error!("不是JAR文件: {}", jar_file.cyan());
        }
    }
    
    valid
}

/// 列出当前目录的JAR文件
pub fn list_available_jars() -> Vec<String> {
    let mut jars = Vec::new();
    
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.ends_with(".jar") && entry.path().is_file() {
                    jars.push(file_name.to_string());
                }
            }
        }
    }
    
    jars.sort();
    jars
}

/// 格式化文件大小
#[allow(dead_code)]
pub fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = size as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if size.fract() == 0.0 {
        format!("{:.0} {}", size, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}
