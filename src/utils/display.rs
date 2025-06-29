// utils/display.rs - 显示和输出工具

use colored::Colorize;

/// 打印命令补全脚本
pub fn print_completions(shell: &str) {
    match shell {
        "bash" => {
            print_success!("生成Bash补全脚本...");
            println!("{}", generate_bash_completions());
        }
        "zsh" => {
            print_success!("生成Zsh补全脚本...");
            println!("{}", generate_zsh_completions());
        }
        "powershell" | "pwsh" => {
            print_success!("生成PowerShell补全脚本...");
            println!("{}", generate_powershell_completions());
        }
        _ => {
            print_warn!("不支持的shell类型: {}，支持 bash/zsh/powershell", shell);
        }
    }
}

/// 打印使用说明
pub fn print_usage() {
    println!("{}", "Manager_Jar - JAR应用管理工具".bright_blue().bold());
    println!();
    println!("{}", "基本用法:".bright_green());
    println!("  {} <命令> [选项] [参数]", "Manager_Jar".bright_cyan());
    println!();
    println!("{}", "主要命令:".bright_green());
    println!("  {}          启动JAR应用", "start".bright_yellow());
    println!("  {}           停止JAR应用", "stop".bright_yellow());
    println!("  {}        重启JAR应用", "restart".bright_yellow());
    println!("  {}         快速启动/重启", "quick".bright_yellow());
    println!("  {}           强制终止应用", "kill".bright_yellow());
    println!("  {}         显示应用状态", "status".bright_yellow());
    println!("  {}           列出运行中的应用", "list".bright_yellow());
    println!();
    println!("{}", "工具命令:".bright_green());
    println!("  {}        显示版本信息", "version".bright_yellow());
    println!("  {}           显示帮助", "help".bright_yellow());
    println!("  {}   生成补全脚本", "completions".bright_yellow());
}

/// 显示版本信息
pub fn show_version() {
    use std::env;
    
    // 使用构建脚本生成的信息
    let version = env!("CARGO_PKG_VERSION");
    let build_time = env!("LOCAL_BUILD_TIME");
    let rustc_version = env!("RUSTC_VERSION");
    let git_hash = env!("GIT_HASH");
    let build_mode = env!("BUILD_MODE");
    
    // 主标题
    println!();
    println!("{}", "╔══════════════════════════════════════════════════════════════════════════╗".bright_blue());
    println!("{} {} {} {}",
        "║".bright_blue(),
        format!("🚀 Manager_Jar {}", format!("v{}", version).bright_green().bold()).bright_cyan().bold(),
        " ".repeat(35 - version.len()),
        "║".bright_blue()
    );
    println!("{} {} {} {}",
        "║".bright_blue(),
        "   专业的 Java 应用程序生命周期管理工具".bright_white(),
        " ".repeat(12),
        "║".bright_blue()
    );
    println!("{}", "╚══════════════════════════════════════════════════════════════════════════╝".bright_blue());
    println!();

    // 基本信息
    println!("📋 {}:", "基本信息".bright_green().bold());
    println!("   🏷️  版本号:     {}", version.bright_green());
    println!("   📅 构建时间:   {}", build_time.bright_yellow());
    println!("   💻 目标平台:   {} ({})", env::consts::OS.bright_cyan(), env::consts::ARCH.bright_cyan());
    println!("   🦀 Rust版本:   {}", rustc_version.bright_red());
    println!("   🏗️  架构模式:   {}", "模块化设计".bright_magenta());
    println!("   🎯 编译模式:   {}", build_mode.bright_green());
    if git_hash != "unknown" && !git_hash.is_empty() {
        println!("   📝 Git提交:    {}", git_hash.bright_blue());
    }
    println!();

    // 快速开始
    println!("💡 {}:", "快速开始".bright_green().bold());
    println!("   使用 {} 查看详细帮助", "Manager_Jar help".bright_cyan());
    println!("   使用 {} 生成命令补全", "Manager_Jar completions".bright_cyan());
    println!();
}

/// 生成Bash补全脚本
fn generate_bash_completions() -> String {
    r#"# Manager_Jar Bash补全脚本
_manager_jar() {
    local cur prev commands
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    
    commands="start stop restart quick kill status list config batch sequence daemon logs version help completions"
    
    case "${prev}" in
        start|stop|restart|quick|kill|status|config)
            COMPREPLY=( $(compgen -f -X '!*.jar' -- ${cur}) )
            return 0
            ;;
        completions)
            COMPREPLY=( $(compgen -W "bash zsh powershell" -- ${cur}) )
            return 0
            ;;
    esac
    
    COMPREPLY=( $(compgen -W "${commands}" -- ${cur}) )
}

complete -F _manager_jar Manager_Jar"#.to_string()
}

/// 生成Zsh补全脚本
fn generate_zsh_completions() -> String {
    r#"#compdef Manager_Jar
# Manager_Jar Zsh补全脚本

_manager_jar() {
    local context state line
    typeset -A opt_args

    _arguments \
        '1: :_manager_jar_commands' \
        '*: :_manager_jar_args' \
        && return 0
}

_manager_jar_commands() {
    _values 'Manager_Jar commands' \
        'start[启动JAR应用]' \
        'stop[停止JAR应用]' \
        'restart[重启JAR应用]' \
        'quick[快速启动/重启]' \
        'kill[强制终止应用]' \
        'status[显示应用状态]' \
        'list[列出运行中的应用]' \
        'version[显示版本]' \
        'help[显示帮助]' \
        'completions[生成补全脚本]'
}

_manager_jar "$@""#.to_string()
}

/// 生成PowerShell补全脚本
fn generate_powershell_completions() -> String {
    r#"# Manager_Jar PowerShell补全脚本
Register-ArgumentCompleter -CommandName Manager_Jar -ScriptBlock {
    param($commandName, $wordToComplete, $cursorPosition)
    
    $commands = @(
        'start', 'stop', 'restart', 'quick', 'kill', 
        'status', 'list', 'version', 'help', 'completions'
    )
    
    if ($wordToComplete) {
        $commands | Where-Object { $_ -like "$wordToComplete*" }
    } else {
        $commands
    }
}"#.to_string()
}
