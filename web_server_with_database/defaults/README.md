## Test Package for 3D Print Price Evaluator

### Initial Setup

Before starting the program, follow these configuration steps:

1. Install Prusa Slicer on your PC
2. Modify line 4 in `print_price_evaluator.bat` to point to your `prusa-slicer.exe` file location
3. Update `data_files\price_calculator_params.json` with your pricing parameters (see pricing guide below)
4. Copy your Prusa Slicer configuration into `data_files\prusa_config.ini` for custom printer settings

### Price Calculator Parameters Guide

#### Material Rates (0.01 PLN/m)
- PLA: 60
- PET: 80
- ASA: 100

#### Time-Based Pricing
Time thresholds indicate hours above which specific hourly rates apply. For example, 0 means the rate applies from print start.

##### Thresholds
- `hourly_rate_time_threshold`: [0, 10, 100] hours

##### Hourly Rates (PLN/h)
Material | Rate 1 | Rate 2 | Rate 3
---------|--------|--------|--------
PLA | 30 | 25 | 20
PET | 35 | 30 | 25
ASA | 40 | 35 | 30


Guidelines for understanding price_calculator_params:
    Material rates in 0.01 PLN / m
    material_rate_pla: 60 (0.01 PLN / m)
    material_rate_pet: 80 (0.01 PLN / m)
    material_rate_asa: 100 (0.01 PLN / m)
    Time thresholds are hours above which the hourly rate applies
    Example: If the first value is 0, it means that the first hourly rate applies from the start of the print
    hourly_rate_time_threshold: [0, 10, 100] hours
    hourly_rate_pla_price: [30, 25, 20] PLN/h
    hourly_rate_pet_price: [35, 30, 25] PLN/h
    hourly_rate_asa_price: [40, 35, 30] PLN/h

