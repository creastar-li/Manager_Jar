# Manager_Jar

<div align="center">

**Language**: [🇨🇳 中文](README.md) | [🇺🇸 English](README_EN.md)

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-Windows-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.87+-orange.svg)
![Version](https://img.shields.io/badge/version-0.1.0-green.svg)

**Professional Java Application Lifecycle Management Tool**

A powerful, modular JAR package manager for starting, stopping, restarting, monitoring, and batch operations of Java applications.

[Features](#features) • [Installation](#installation) • [Quick Start](#quick-start) • [Documentation](#documentation) • [Contributing](#contributing)

</div>

---

## Features

### 🚀 Core Functionality
- **Process Management**: Start, stop, restart JAR applications
- **Status Monitoring**: Real-time JAR status and resource monitoring  
- **Log Management**: Centralized log viewing and management
- **Configuration Management**: Flexible parameter configuration and presets

### 🔥 Advanced Features
- **Batch Operations**: Manage multiple JAR applications simultaneously
- **Sequence Management**: Start/stop application groups in predefined order
- **Daemon Process**: System-level daemon with auto-restart and monitoring
- **Security Protection**: Safe process management and resource protection

### 💻 Developer Friendly
- **Modern CLI**: Intuitive command-line interface with colored output
- **Command Completion**: Auto-completion for Bash, Zsh, PowerShell
- **Comprehensive Documentation**: Complete usage guide and API documentation
- **Modular Design**: Clean code structure, easy to extend and maintain

## Installation

### Prerequisites
- Windows operating system
- Java Runtime Environment (JRE 8+)
- One or more JAR files

### Method 1: Build from Source (Recommended)

```powershell
# Clone the repository
git clone https://github.com/YourUsername/Manager_Jar.git
cd Manager_Jar

# Build release version
cargo build --release

# Optional: Add executable to PATH
copy .\target\release\Manager_Jar.exe C:\Windows\System32\
```

### Method 2: Using Build Scripts

```powershell
# PowerShell users
.\build.ps1

# Or using CMD
.\build.bat
```

### Method 3: Development Environment

```powershell
# Install Rust (if not installed)
# Visit https://rustup.rs/

# Clone and enter project directory
git clone https://github.com/YourUsername/Manager_Jar.git
cd Manager_Jar

# Run in development mode
cargo run -- --help
```

## Quick Start

### Basic Operations

```powershell
# Start a JAR application
Manager_Jar.exe start myapp.jar

# Start with parameters
Manager_Jar.exe start myapp.jar -Xmx1g -server

# Check status
Manager_Jar.exe status myapp.jar

# Stop application
Manager_Jar.exe stop myapp.jar

# Restart application
Manager_Jar.exe restart myapp.jar
```

### Configuration Management

```powershell
# Save configuration
Manager_Jar.exe config myapp.jar --args "-Xmx2g -server" --name "production"

# Quick start with saved configuration
Manager_Jar.exe quick myapp.jar --config "production"

# List all configurations
Manager_Jar.exe configs list
```

### Batch Operations

```powershell
# Batch start multiple applications
Manager_Jar.exe batch start app1.jar app2.jar app3.jar

# Batch stop
Manager_Jar.exe batch stop app1.jar app2.jar

# Batch status check
Manager_Jar.exe batch status app1.jar app2.jar
```

### Sequence Management

```powershell
# Create application sequence
Manager_Jar.exe sequence create webapps app1.jar app2.jar app3.jar

# Start by sequence (app1 -> app2 -> app3)
Manager_Jar.exe sequence start webapps

# Stop by sequence (app3 -> app2 -> app1)
Manager_Jar.exe sequence stop webapps
```

### Log Management

```powershell
# View real-time logs
Manager_Jar.exe log myapp.jar --follow

# View last 100 lines
Manager_Jar.exe log myapp.jar --lines 100

# View error logs
Manager_Jar.exe logs error myapp.jar
```

### Daemon Process

```powershell
# Start system daemon
Manager_Jar.exe daemon start

# Add application to monitoring with auto-restart
Manager_Jar.exe daemon add myapp.jar --auto-restart

# Check daemon status
Manager_Jar.exe daemon status
```

## Usage Examples

### Example 1: Microservice Management
```powershell
# Create microservice startup sequence
Manager_Jar.exe sequence create microservices config-server.jar eureka-server.jar gateway.jar user-service.jar

# Start all services in sequence
Manager_Jar.exe sequence start microservices

# Start daemon monitoring
Manager_Jar.exe daemon start

# Add critical services to monitoring
Manager_Jar.exe daemon add config-server.jar --auto-restart
Manager_Jar.exe daemon add eureka-server.jar --auto-restart
```

### Example 2: Development Environment
```powershell
# Save debug configuration
Manager_Jar.exe config myapp.jar --args "-Xmx1g -Xdebug -Xrunjdwp:transport=dt_socket,server=y,suspend=n,address=5005" --name "debug"

# Quick start in debug mode
Manager_Jar.exe quick myapp.jar --config "debug"

# View real-time logs
Manager_Jar.exe log myapp.jar --follow
```

### Example 3: Production Deployment
```powershell
# Save production configuration
Manager_Jar.exe config webapp.jar --args "-Xmx4g -Xms2g -server -XX:+UseG1GC" --name "production"

# Start application
Manager_Jar.exe quick webapp.jar --config "production"

# Start background daemon
Manager_Jar.exe daemon start --background

# Add to monitoring with auto-restart
Manager_Jar.exe daemon add webapp.jar --auto-restart
```

## Project Structure

```
Manager_Jar/
├── src/
│   ├── main.rs              # Main program entry
│   ├── build_info.rs        # Build information (auto-generated)
│   ├── cli/                 # Command line interface
│   │   ├── commands.rs      # CLI command definitions
│   │   └── completions.rs   # Command completion
│   ├── core/                # Core functionality
│   │   ├── process.rs       # Process management
│   │   ├── config.rs        # Configuration management
│   │   └── logging.rs       # Logging system
│   ├── operations/          # Operation modules
│   │   ├── batch.rs         # Batch operations
│   │   └── sequence.rs      # Sequence management
│   ├── daemon/              # Daemon process
│   │   ├── thread_daemon.rs # Thread daemon
│   │   └── system_daemon.rs # System daemon
│   └── utils/               # Utility modules
│       ├── macros.rs        # Macro definitions
│       ├── files.rs         # File operations
│       └── display.rs       # Display utilities
├── build.rs                 # Build script
├── Cargo.toml              # Project configuration
├── build.ps1               # PowerShell build script
├── build.sh                # Bash build script
├── build.bat               # CMD build script
├── README.md               # Project documentation
└── USAGE_GUIDE.md          # Detailed usage guide
```

## Configuration

### Global Configuration

```powershell
# Set global configuration
Manager_Jar.exe global-config set log_level info
Manager_Jar.exe global-config set max_processes 10
Manager_Jar.exe global-config set default_jvm_args "-Xmx1g"

# View configuration
Manager_Jar.exe global-config show

# Reset configuration
Manager_Jar.exe global-config reset
```

### Configuration File Locations

- **Windows**: `%APPDATA%\Manager_Jar\`
  - Global config: `config\global.toml`
  - JAR configs: `config\jars\*.toml`
  - Log files: `logs\*.log`

### Configuration Example

```toml
# global.toml
[settings]
log_level = "info"
max_processes = 10
default_jvm_args = "-Xmx1g -server"
auto_restart = true

[daemon]
check_interval = 30
restart_delay = 5
max_restart_attempts = 3
```

## Documentation

- 📖 **[Detailed Usage Guide](USAGE_GUIDE.md)** - Complete command reference and examples
- 🏗️ **[Project Structure](#project-structure)** - Architecture and module organization
- ⚙️ **[Configuration](#configuration)** - Configuration management guide
- 🔧 **[Development](#development)** - Development setup and contribution guide

## Development

### Development Environment Setup

```powershell
# 1. Install Rust
# Visit https://rustup.rs/ for installation

# 2. Clone project
git clone https://github.com/YourUsername/Manager_Jar.git
cd Manager_Jar

# 3. Install dependencies and run tests
cargo test

# 4. Run in development mode
cargo run -- --help
```

### Available Development Tasks

```powershell
# Build project
cargo build

# Run tests
cargo test

# Check code
cargo check

# Format code
cargo fmt

# Clean build files
cargo clean

# Build release version
cargo build --release
```

### VS Code Tasks

The project includes pre-configured VS Code tasks:

- `Build Manager_Jar` - Build project
- `Build Manager_Jar Release` - Build release version
- `Run Tests` - Run tests
- `Format Code` - Format code
- `Check Code` - Check code
- `Run Manager_Jar` - Run program

## Command Reference

### Available Commands

```
Manager_Jar.exe [COMMAND]

Commands:
  completions    Generate command line completion scripts
  start          Start specified JAR package
  stop           Stop specified JAR package
  restart        Restart specified JAR package
  status         Check JAR package status
  list           List all available JAR files
  log            View JAR logs
  kill           Force kill JAR process
  config         Configure JAR parameters
  quick          Quick start JAR with saved configuration
  batch          Batch operations
  sequence       Sequence batch management for JAR groups
  logs           Log management
  configs        Configuration management
  global-config  Global configuration management
  daemon         System-level daemon process management
  version        Show version information
  help           Print help message or help for given subcommand
```

### Command Completion

```powershell
# Generate PowerShell completion
Manager_Jar.exe completions powershell > Manager_Jar_completions.ps1

# Generate Bash completion
Manager_Jar.exe completions bash > Manager_Jar_completions.bash

# Generate Zsh completion
Manager_Jar.exe completions zsh > Manager_Jar_completions.zsh
```

## System Requirements

### Runtime Environment
- **Java Runtime Environment (JRE) 8+** - Required for running JAR packages
- **Operating System**: Windows 10+

### Performance Characteristics
- **Memory Usage**: < 10MB
- **Startup Time**: < 100ms
- **CPU Usage**: Near-zero overhead
- **Platform Support**: x86_64, ARM64

## Contributing

We welcome contributions of all kinds!

### How to Contribute

1. **Report Bugs**: Describe issues in [Issues](../../issues)
2. **Feature Requests**: Propose new features in [Issues](../../issues)
3. **Code Contributions**: Fork the project and submit Pull Requests
4. **Documentation**: Help improve documentation and examples

### Contribution Guidelines

1. Fork this repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Development Standards

- Use `cargo fmt` to format code
- Use `cargo clippy` to check code quality
- Add tests for new features
- Update relevant documentation

## Changelog

### v0.1.0 (2025-07-02)

#### 🎉 Initial Release
- ✨ Complete JAR package management functionality
- 🏗️ Modular architecture design
- 🎨 Modern CLI interface
- 📝 Complete documentation and user guide

#### 🚀 Core Features
- JAR package start, stop, restart
- Process status monitoring
- Log management system
- Configuration management

#### 🔥 Advanced Features
- Batch operation support
- Sequence management
- System daemon process
- Command line completion

#### 🛠️ Technical Features
- Rust 1.87+ support
- Automatic build information injection
- Cross-platform build scripts
- Zero compilation warnings

## Support

### Getting Help

- 📚 Check [USAGE_GUIDE.md](USAGE_GUIDE.md) for detailed usage guide
- 💬 Ask questions in [Issues](../../issues)
- 📧 Contact maintainer: creastar@gmail.com

### Documentation Resources

- 📚 [Complete Usage Guide](USAGE_GUIDE.md)
- 🔧 [Development Documentation](docs/DEVELOPMENT.md)
- 📝 [Changelog](CHANGELOG.md)
- ❓ [FAQ](USAGE_GUIDE.md#troubleshooting)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

<div align="center">

**⭐ If this project helps you, please give it a star!**

[![GitHub stars](https://img.shields.io/github/stars/YourUsername/Manager_Jar?style=social)](../../stargazers)
[![GitHub forks](https://img.shields.io/github/forks/YourUsername/Manager_Jar?style=social)](../../network/members)

Made with ❤️ by [CreaStar](https://github.com/CreaStar)

**[⬆ Back to Top](#manager_jar)**

</div>
