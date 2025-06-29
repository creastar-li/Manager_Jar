// build.rs - 构建脚本
// 在编译时生成构建信息

use chrono::{Utc, Local};
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // 生成构建时间戳
    let build_time = Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    let local_build_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    // 获取构建信息
    let rustc_version = get_rustc_version();
    let git_hash = get_git_hash();
    let git_branch = get_git_branch();
    let build_mode = if cfg!(debug_assertions) { "Debug" } else { "Release" };
    let target_triple = env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());
    let host_triple = env::var("HOST").unwrap_or_else(|_| "unknown".to_string());
    let profile = env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string());
    
    // 输出环境变量供编译时使用
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);
    println!("cargo:rustc-env=LOCAL_BUILD_TIME={}", local_build_time);
    println!("cargo:rustc-env=RUSTC_VERSION={}", rustc_version);
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=GIT_BRANCH={}", git_branch);
    println!("cargo:rustc-env=BUILD_MODE={}", build_mode);
    println!("cargo:rustc-env=TARGET_TRIPLE={}", target_triple);
    println!("cargo:rustc-env=HOST_TRIPLE={}", host_triple);
    println!("cargo:rustc-env=BUILD_PROFILE={}", profile);
    
    // 生成构建信息文件
    generate_build_info(&build_time, &local_build_time, &rustc_version, &git_hash, &git_branch, build_mode, &target_triple, &profile);
    
    // 告诉Cargo在这些文件改变时重新构建
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=src/");
    
    // 如果有.git目录，监控git变化
    if Path::new(".git").exists() {
        println!("cargo:rerun-if-changed=.git/HEAD");
        println!("cargo:rerun-if-changed=.git/refs/");
    }
    
    // 更美观的构建信息输出
    println!("cargo:warning=🚀 Manager_Jar 构建信息已生成");
    println!("cargo:warning=📅 构建时间: {}", local_build_time);
    println!("cargo:warning=🦀 Rust版本: {}", rustc_version);
    if !git_hash.is_empty() && git_hash != "unknown" {
        if !git_branch.is_empty() && git_branch != "unknown" {
            println!("cargo:warning=🌿 Git分支: {} ({})", git_branch, git_hash);
        } else {
            println!("cargo:warning=📝 Git提交: {}", git_hash);
        }
    }
    println!("cargo:warning=🎯 目标平台: {}", target_triple);
    println!("cargo:warning=📊 构建模式: {}", profile);
}

fn get_rustc_version() -> String {
    env::var("RUSTC_VERSION").unwrap_or_else(|_| {
        // 尝试从rustc --version获取
        std::process::Command::new("rustc")
            .arg("--version")
            .output()
            .map(|output| {
                String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .to_string()
            })
            .unwrap_or_else(|_| "unknown".to_string())
    })
}

fn get_git_hash() -> String {
    // 尝试从git获取当前提交哈希
    std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .map(|output| {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            } else {
                "unknown".to_string()
            }
        })
        .unwrap_or_else(|_| "unknown".to_string())
}

fn get_git_branch() -> String {
    // 尝试从git获取当前分支名
    std::process::Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .map(|output| {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            } else {
                "unknown".to_string()
            }
        })
        .unwrap_or_else(|_| "unknown".to_string())
}

fn generate_build_info(build_time: &str, local_build_time: &str, rustc_version: &str, git_hash: &str, git_branch: &str, build_mode: &str, target_triple: &str, profile: &str) {
    let build_info_content = format!(
        r#"// 构建信息 - 自动生成，请勿手动编辑
// Generated at: {}

pub const BUILD_TIME: &str = "{}";
pub const LOCAL_BUILD_TIME: &str = "{}";
pub const RUSTC_VERSION: &str = "{}";
pub const GIT_HASH: &str = "{}";
pub const GIT_BRANCH: &str = "{}";
pub const BUILD_MODE: &str = "{}";
pub const TARGET_TRIPLE: &str = "{}";
pub const BUILD_PROFILE: &str = "{}";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

/// 获取完整的构建信息字符串
pub fn get_build_info() -> String {{
    format!(
        "{{}} v{{}} ({{}})\n构建时间: {{}}\nRust版本: {{}}\nGit信息: {{}} ({{}})\n目标平台: {{}}",
        NAME, VERSION, BUILD_MODE, LOCAL_BUILD_TIME, RUSTC_VERSION, GIT_BRANCH, GIT_HASH, TARGET_TRIPLE
    )
}}

/// 获取简短的构建信息
pub fn get_short_build_info() -> String {{
    if GIT_HASH != "unknown" {{
        format!("v{{}} ({{}} {{}}) [{{}}]", VERSION, BUILD_MODE, LOCAL_BUILD_TIME, &GIT_HASH[..7.min(GIT_HASH.len())])
    }} else {{
        format!("v{{}} ({{}} {{}})", VERSION, BUILD_MODE, LOCAL_BUILD_TIME)
    }}
}}

/// 获取详细的构建环境信息
pub fn get_build_environment() -> String {{
    format!(
        "构建环境:\n  时间: {{}}\n  Rust: {{}}\n  目标: {{}}\n  模式: {{}}\n  分支: {{}}\n  提交: {{}}",
        LOCAL_BUILD_TIME, RUSTC_VERSION, TARGET_TRIPLE, BUILD_PROFILE, GIT_BRANCH, GIT_HASH
    )
}}
"#,
        build_time, build_time, local_build_time, rustc_version, git_hash, git_branch, build_mode, target_triple, profile
    );
    
    // 确保src目录存在
    let src_dir = Path::new("src");
    if !src_dir.exists() {
        fs::create_dir_all(src_dir).expect("Failed to create src directory");
    }
    
    // 写入构建信息文件
    let build_info_path = src_dir.join("build_info.rs");
    fs::write(&build_info_path, build_info_content)
        .expect("Failed to write build info file");
}
