// cli/completions.rs - 命令行补全功能

use clap::CommandFactory;
use clap_complete::shells::{Bash, Zsh, PowerShell};
use crate::cli::Cli;

/// 处理补全命令
#[allow(dead_code)]
pub fn handle_completions(shell: &str) {
    let mut cmd = Cli::command();
    match shell {
        "bash" => {
            print_success!("生成Bash补全脚本...");
            clap_complete::generate(Bash, &mut cmd, "Manager_Jar", &mut std::io::stdout());
        },
        "zsh" => {
            print_success!("生成Zsh补全脚本...");
            clap_complete::generate(Zsh, &mut cmd, "Manager_Jar", &mut std::io::stdout());
        },
        "powershell" | "ps" => {
            print_success!("生成PowerShell补全脚本...");
            clap_complete::generate(PowerShell, &mut cmd, "Manager_Jar", &mut std::io::stdout());
        },
        _ => print_warn!("不支持的shell类型: {}，支持 bash/zsh/powershell", shell),
    }
}
