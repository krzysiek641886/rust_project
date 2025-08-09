@echo off
setlocal
REM Set paths (update prusa_slicer_path as needed)
set prusa_slicer_path=C:\Program Files\PrusaSlicer-2.9.0\prusa-slicer.exe
set price_calculator_params=data_files\price_calculator_params.json
set current_directory=%cd%
@echo on

.\web_server_with_database.exe --ws-path "%current_directory%" --db-name data_files\price_evaluator_database.db --prusa-slicer-path "%prusa_slicer_path%" --system windows --price-params "%price_calculator_params%"
pause
