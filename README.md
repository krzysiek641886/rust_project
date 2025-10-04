# 3D Print Price Evaluator

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![PrusaSlicer](https://img.shields.io/badge/PrusaSlicer-2.5%2B-blue.svg)](https://www.prusa3d.com/prusaslicer/)

A comprehensive web application for 3D print pricing, built with Rust. This system provides an intuitive web interface to upload 3D STL files and calculate printing costs based on customizable pricing parameters.

## Features

- 📤 Upload STL files directly through the web interface
- 💲 Automatic price calculation based on material type, print time, and customizable rates
- 💾 SQLite database for order tracking and management
- 📋 Order history and status tracking
- 🖨️ Integration with PrusaSlicer for accurate print time and material calculations
- 🔄 Real-time WebSocket updates during price calculation

## System Requirements

- **Rust** (version 1.70 or higher)
- **PrusaSlicer** (must be installed separately)
- **SQLite** (bundled with the application)
- Supported Operating Systems:
  - macOS
  - Windows

## Quick Start

### macOS Setup

```bash
# Clone this repository
git clone https://github.com/krzysiek641886/rust_project.git
cd rust_project/web_server_with_database

# Setup the project directory structure and install PrusaSlicer
./setup_and_run_server_macos.sh --setup

# Run the server
./setup_and_run_server_macos.sh --run
```

### Windows Setup

```batch
:: Clone this repository
git clone https://github.com/krzysiek641886/rust_project.git
cd rust_project\web_server_with_database

:: Setup the project directory structure
setup_and_run_server_windows.bat --setup

:: Run the server
setup_and_run_server_windows.bat --run
```

## Configuration

Before running the application, configure your pricing parameters in `data_files/print_price_evaluator_config.json`:

```json
{
    "prusa_path": "/path/to/prusa-slicer",
    "material_rate_pla": 60,
    "material_rate_pet": 80,
    "material_rate_asa": 100,
    "hourly_rate_time_threshold": [0, 10, 100],
    "hourly_rate_pla_price": [30, 25, 20],
    "hourly_rate_pet_price": [35, 30, 25],
    "hourly_rate_asa_price": [40, 35, 30]
}
```

### Configuration Options:

- **prusa_path**: Path to your PrusaSlicer executable
  - Windows: `C:\Program Files\PrusaSlicer\prusa-slicer.exe`
  - macOS: `/Applications/PrusaSlicer.app/Contents/MacOS/PrusaSlicer`
- **material_rate_X**: Material cost rate in 0.01 PLN/m
- **hourly_rate_time_threshold**: Time thresholds in hours for tiered pricing
- **hourly_rate_X_price**: Hourly rates for different materials based on thresholds

## Project Structure

```
web_server_with_database/
├── src/                           # Source code
│   ├── main.rs                    # Application entry point
│   ├── api/                       # Web API implementation
│   ├── common_utils/              # Shared utilities
│   ├── database_handler/          # Database interaction
│   ├── prusa_slicer_interface/    # PrusaSlicer integration
│   └── frontend/                  # Web interface files
├── data_files/                    # Runtime data
│   ├── received_orders/           # Uploaded STL files
│   ├── processed_orders/          # Processed G-code files
│   └── prusa_config_files/        # PrusaSlicer configurations
└── defaults/                      # Default configuration files
```

## Manual Setup

If the setup scripts don't work for your environment, you can manually configure the system:

1. Create the necessary directories:
   ```
   mkdir -p data_files/received_orders
   mkdir -p data_files/processed_orders
   mkdir -p data_files/prusa_config_files
   ```

2. Copy the default config files:
   ```
   cp defaults/print_price_evaluator_config.json data_files/
   cp defaults/prusa_config_files/* data_files/prusa_config_files/
   ```

3. Edit `data_files/print_price_evaluator_config.json` with your PrusaSlicer path and pricing parameters

4. Build and run the application:
   ```
   cargo build --release
   ./target/release/web_server_with_database
   ```

## Development

To build the project from source:

```bash
cargo build       # Debug build
cargo build --release  # Release build
cargo test        # Run tests
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.