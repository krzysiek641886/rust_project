# 3D Print Price Evaluator

## Setup Guide

Before running the application, follow these configuration steps:

1. Install [Prusa Slicer](https://www.prusa3d.com/prusaslicer/) on your computer
2. Configure the `print_price_evaluator_config.json` file in the `data_files` directory with:
   - The path to your Prusa Slicer executable:
     - Windows: `"prusa_path": "C:\\Program Files\\PrusaSlicer\\prusa-slicer.exe"`
     - macOS: `"prusa_path": "/Applications/PrusaSlicer.app/Contents/MacOS/PrusaSlicer"`
   - Your pricing parameters (see pricing guide below)
3. Make sure you have the proper Prusa Slicer configuration files in the `data_files/prusa_config_files` directory

## Running the Application

To run the application, simply execute the `web_server_with_database` executable with the path to your configuration file:

```
./web_server_with_database --app-params data_files/print_price_evaluator_config.json
```

The application will start a web server at http://127.0.0.1:8080 where you can access the 3D print price calculator.

## Price Calculator Configuration Guide

### Configuration File Format

The `print_price_evaluator_config.json` file should include these parameters:

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

### Material Rates (0.01 PLN/m)
- PLA: 60 (0.01 PLN / m)
- PET: 80 (0.01 PLN / m)
- ASA: 100 (0.01 PLN / m)

### Time-Based Pricing
Time thresholds indicate hours above which specific hourly rates apply:

#### Thresholds
- `hourly_rate_time_threshold`: [0, 10, 100] hours
  - Example: If the first value is 0, it means that the first hourly rate applies from the start of the print
  - When print time exceeds 10 hours, the second rate applies
  - When print time exceeds 100 hours, the third rate applies

#### Hourly Rates (PLN/h)
| Material | Rate 1 (0-10h) | Rate 2 (10-100h) | Rate 3 (>100h) |
|----------|----------------|------------------|----------------|
| PLA      | 30             | 25               | 20             |
| PET      | 35             | 30               | 25             |
| ASA      | 40             | 35               | 30             |

## Pricing Formula

The price calculation follows this formula:
- Gross Price = (Material Cost + Print Time Cost) × Number of Copies + 1 PLN
- Material Cost = Material Usage × Material Rate / 100,000 (PLN)
- Print Time Cost = Print Time (seconds) × Hourly Rate / 3600 (PLN)

