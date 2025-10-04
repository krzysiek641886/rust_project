@echo off
setlocal
REM Set paths (update prusa_slicer_path as needed)
set prusa_slicer_path=C:\Program Files\PrusaSlicer-2.9.0\prusa-slicer.exe
set app_params=data_files\print_price_evaluator_config.json
set current_directory=%cd%
@echo on

start /B .\web_server_with_database.exe --app-params "%app_params%"
start http://127.0.0.1:8080/
pause
