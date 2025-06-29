// cli/mod.rs - 命令行接口模块

pub mod commands;
pub mod completions;

// 重新导出主要类型
pub use commands::*;
