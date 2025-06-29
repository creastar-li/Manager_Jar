// main.rs - JAR包管理工具 - 模块化版本
extern crate colored;

// 新的模块结构
#[macro_use]
mod utils;
mod core;
mod cli;
mod operations;
mod daemon;

// ====== 必要的 use 导入 ======
use clap::Parser;
use std::env;
use crate::utils::display::{print_completions, print_usage, show_version};
use crate::cli::commands::*;
use colored::Colorize;

// ═══════════════════════════════════════════════════════════════════════
//                            启动程序
// ═══════════════════════════════════════════════════════════════════════

fn main() {
    // 检查是否以守护进程模式启动
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--daemon-mode" {
        daemon::SystemDaemon::run_daemon_mode();
        return;
    }
    
    // 初始化目录结构
    utils::files::init_directories();
    
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Completions { shell }) => {
            match shell.as_str() {
                "bash" => print_completions("bash"),
                "zsh" => print_completions("zsh"),
                "powershell" | "ps" => print_completions("powershell"),
                _ => print_warn!("不支持的shell类型: {}，支持 bash/zsh/powershell", shell),
            }
        },
        Some(Commands::Start { jar, args }) => {
            if !utils::files::validate_jar_file(jar.as_str()) { return; }
            if let Err(e) = core::process::start_jar(jar.as_str(), args.as_slice()) {
                print_error!("{}", e);
            }
        },
        Some(Commands::Stop { jar }) => {
            if !utils::files::validate_jar_file(jar.as_str()) { return; }
            if let Err(e) = core::process::stop_jar(jar.as_str()) {
                print_error!("{}", e);
            }
        },
        Some(Commands::Restart { jar, args }) => {
            if !utils::files::validate_jar_file(jar.as_str()) { return; }
            if let Err(e) = core::process::restart_jar(jar.as_str(), args.as_slice()) {
                print_error!("{}", e);
            }
        },
        Some(Commands::Status { jar }) => {
            if let Some(jar) = jar {
                if !utils::files::validate_jar_file(jar.as_str()) { return; }
            }
            core::process::show_jar_status(jar.as_deref());
        },
        Some(Commands::List) => {
            let jars = utils::files::list_available_jars();
            if jars.is_empty() {
                print_warn!("当前目录下无可用 JAR 文件");
            } else {
                print_success!("发现 {} 个可用 JAR 文件:", jars.len());
                for jar in jars { 
                    let status = if core::process::is_jar_running(&jar) { "运行中".green() } else { "已停止".red() };
                    println!("  {} [{}]", jar.cyan(), status); 
                }
            }
            println!();
            core::process::list_running_jars();
        },
        Some(Commands::Log { jar, lines }) => {
            if !utils::files::validate_jar_file(jar.as_str()) { return; }
            core::logging::show_jar_log(jar.as_str(), *lines)
        },
        Some(Commands::Kill { jar }) => {
            if !utils::files::validate_jar_file(jar.as_str()) { return; }
            if let Err(e) = core::process::kill_jar(jar.as_str()) {
                print_error!("{}", e);
            }
        },
        Some(Commands::Config { jar, args }) => {
            if !utils::files::validate_jar_file(jar.as_str()) { return; }
            if let Err(e) = core::config::save_jar_config(jar.as_str(), args.as_slice()) {
                print_error!("{}", e);
            }
        },
        Some(Commands::Quick { jar }) => {
            if !utils::files::validate_jar_file(jar.as_str()) { return; }
            if let Err(e) = core::config::quick_start_jar(jar.as_str()) {
                print_error!("{}", e);
            }
        },
        Some(Commands::Batch { op, jars }) => {
            if jars.is_empty() {
                print_warn!("请指定要批量操作的 JAR 文件名（用空格分隔）");
                let available = utils::files::list_available_jars();
                if !available.is_empty() {
                    println!("可用文件: {}", available.join(", "));
                }
                return;
            }
            // 验证所有JAR文件
            let mut valid_jars = Vec::new();
            for jar in jars {
                if utils::files::validate_jar_file(jar) {
                    valid_jars.push(jar.clone());
                }
            }
            if valid_jars.is_empty() {
                print_error!("没有有效的JAR文件");
                return;
            }
            if let Err(e) = operations::batch::batch_operation(op.as_str(), &valid_jars) {
                print_error!("{}", e);
            }
        },
        Some(Commands::Sequence { op, args }) => {
            if let Err(e) = operations::sequence::sequence_operation(op.as_str(), args.as_slice()) {
                print_error!("{}", e);
            }
        },
        Some(Commands::Logs { action }) => {
            match action {
                LogsAction::List => {
                    core::logging::list_logs();
                },
                LogsAction::Clean => {
                    if let Err(e) = core::logging::clean_logs() {
                        print_error!("{}", e);
                    }
                },
            }
        },
        Some(Commands::Configs { action }) => {
            match action {
                ConfigsAction::List => {
                    core::config::list_configs();
                },
                ConfigsAction::Show { jar } => {
                    if !utils::files::validate_jar_file(jar.as_str()) { return; }
                    if let Err(e) = core::config::show_config(jar.as_str()) {
                        print_error!("{}", e);
                    }
                },
                ConfigsAction::Delete { jar } => {
                    if !utils::files::validate_jar_file(jar.as_str()) { return; }
                    if let Err(e) = core::config::delete_config(jar.as_str()) {
                        print_error!("{}", e);
                    }
                },
            }
        },
        Some(Commands::GlobalConfig { action }) => {
            use crate::core::config::GlobalConfig;
            match action {
                GlobalConfigAction::Show => {
                    let config = GlobalConfig::load();
                    config.display();
                },
                GlobalConfigAction::Edit => {
                    print_success!("请编辑配置文件: {}", GlobalConfig::config_file_path().cyan());
                    println!("提示: 编辑后使用 'global-config reload' 重新加载配置");
                },
                GlobalConfigAction::Reset => {
                    let default_config = GlobalConfig::default();
                    if let Err(e) = default_config.save() {
                        print_error!("重置配置失败: {}", e);
                    } else {
                        print_success!("配置已重置为默认值");
                    }
                },
                GlobalConfigAction::Reload => {
                    print_success!("配置已重新加载");
                },
                GlobalConfigAction::CleanLogs => {
                    let config = GlobalConfig::load();
                    match config.cleanup_old_logs() {
                        Ok(cleaned) => {
                            if cleaned == 0 {
                                print_success!("没有过期日志需要清理");
                            }
                        },
                        Err(e) => print_error!("清理过期日志失败: {}", e),
                    }
                },
                GlobalConfigAction::SetLogDir { path } => {
                    let mut config = GlobalConfig::load();
                    config.log.log_dir = path.clone();
                    if let Err(e) = config.save() {
                        print_error!("保存配置失败: {}", e);
                    } else {
                        print_success!("日志目录已设置为: {}", path.cyan());
                    }
                },
                GlobalConfigAction::SetRetentionDays { days } => {
                    let mut config = GlobalConfig::load();
                    config.log.retention_days = *days;
                    if let Err(e) = config.save() {
                        print_error!("保存配置失败: {}", e);
                    } else {
                        print_success!("日志保存天数已设置为: {} 天", days.to_string().yellow());
                    }
                },
                GlobalConfigAction::SetMaxLogSize { size_mb } => {
                    let mut config = GlobalConfig::load();
                    config.log.max_file_size_mb = *size_mb;
                    if let Err(e) = config.save() {
                        print_error!("保存配置失败: {}", e);
                    } else {
                        print_success!("日志文件最大大小已设置为: {} MB", size_mb.to_string().yellow());
                    }
                },
                GlobalConfigAction::SetLogRotation { enable } => {
                    let mut config = GlobalConfig::load();
                    config.log.enable_rotation = *enable;
                    if let Err(e) = config.save() {
                        print_error!("保存配置失败: {}", e);
                    } else {
                        let status = if *enable { "启用".green() } else { "禁用".red() };
                        print_success!("日志轮转已{}", status);
                    }
                },
            }
        },
        Some(Commands::Daemon { action }) => {
            match action {
                DaemonAction::Start => {
                    if let Err(e) = daemon::SystemDaemon::start() {
                        print_error!("{}", e);
                    }
                },
                DaemonAction::Stop => {
                    if let Err(e) = daemon::SystemDaemon::stop() {
                        print_error!("{}", e);
                    }
                },
                DaemonAction::Status => {
                    daemon::SystemDaemon::status();
                },
                DaemonAction::Restart => {
                    if let Err(e) = daemon::SystemDaemon::restart() {
                        print_error!("{}", e);
                    }
                },
            }
        },
        Some(Commands::Version) => show_version(),
        None => {
            print_usage();
        }
    }
}
