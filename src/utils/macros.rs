// utils/macros.rs - 彩色输出宏定义

/// 成功信息输出宏
#[macro_export]
macro_rules! print_success {
    ($($arg:tt)*) => {
        {
            use colored::Colorize;
            println!("{} {}", "[成功]".green().bold(), format!($($arg)*))
        }
    };
}

/// 警告信息输出宏
#[macro_export]
macro_rules! print_warn {
    ($($arg:tt)*) => {
        {
            use colored::Colorize;
            println!("{} {}", "[警告]".yellow().bold(), format!($($arg)*))
        }
    };
}

/// 错误信息输出宏
#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {
        {
            use colored::Colorize;
            println!("{} {}", "[错误]".red().bold(), format!($($arg)*))
        }
    };
}

/// 信息输出宏
#[macro_export]
macro_rules! print_info {
    ($($arg:tt)*) => {
        {
            use colored::Colorize;
            println!("{} {}", "[信息]".blue().bold(), format!($($arg)*))
        }
    };
}

/// 调试信息输出宏
#[macro_export]
macro_rules! print_debug {
    ($($arg:tt)*) => {
        {
            use colored::Colorize;
            println!("{} {}", "[调试]".magenta().bold(), format!($($arg)*))
        }
    };
}
