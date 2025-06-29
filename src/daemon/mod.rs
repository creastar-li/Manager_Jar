// daemon/mod.rs - 守护进程模块
// 这个模块负责管理进程的守护、自动重启、健康检查等功能

pub mod thread_daemon;
pub mod system_daemon;

// 重新导出主要功能（当前暂未使用，保留供未来扩展）
#[allow(unused_imports)]
pub use thread_daemon::{
    ManagerDaemon, get_daemon, start_daemon, stop_daemon, 
    daemon_status, is_daemon_running
};
#[allow(unused_imports)]
pub use system_daemon::SystemDaemon;
