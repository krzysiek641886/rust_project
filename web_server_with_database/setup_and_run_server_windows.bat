@echo off
setlocal

REM Set paths (update prusa_slicer_path as needed)
set prusa_slicer_path=C:\Program Files\PrusaSlicer-2.9.0\prusa-slicer.exe
set price_calculator_params=data_files\price_calculator_params.json
set current_directory=%cd%

REM Parse command-line arguments
if "%~1"=="" (
    echo Invalid option: %1
    goto show_help
    exit /b 1
)
if "%~1"=="-h" goto show_help
if "%~1"=="--help" goto show_help
if "%~1"=="-s" goto setup_project
if "%~1"=="--setup" goto setup_project
if "%~1"=="-r" goto run_server
if "%~1"=="--run" goto run_server
if "%~1"=="-t" goto run_tests
if "%~1"=="--test" goto run_tests

echo Invalid option: %1
goto show_help
exit /b 1

REM Function to display the help menu
:show_help
echo Usage: setup_and_run_server_windows.bat [OPTION]
echo Options:
echo   -h, --help       Show this help message and exit
echo   -s, --setup      Set up the project
echo   -r, --run        Run the server
echo   -t, --test       Run the tests
goto:eof

REM Function to set up the project
:setup_project
echo Setting up the project...
if not exist data_files mkdir data_files
if not exist data_files\received_orders mkdir data_files\received_orders
if not exist data_files\processed_orders mkdir data_files\processed_orders
if not exist data_files\prusa_config.ini (
    type nul > data_files\prusa_config.ini
    echo Please add a correct config file in data_files\prusa_config.ini
)
REM PrusaSlicer installation must be done manually on Windows.
if not exist "%price_calculator_params%" (
    type nul > "%price_calculator_params%"
)
goto:eof

REM Function to check if project is ready
:check_project_ready
if not exist data_files (
    echo Error: data_files directory structure not correctly configured
    echo Run script with --setup flag to properly setup the project
    exit /b 1
)
if not exist data_files\processed_orders (
    echo Error: data_files directory structure not correctly configured
    echo Run script with --setup flag to properly setup the project
    exit /b 1
)
if not exist data_files\received_orders (
    echo Error: data_files directory structure not correctly configured
    echo Run script with --setup flag to properly setup the project
    exit /b 1
)
if not exist data_files\prusa_config.ini (
    echo Error: data_files/prusa_config.ini not found
    echo Run script with --setup flag to properly setup the project
    exit /b 1
)
if not exist "%price_calculator_params%" (
    echo Error: %price_calculator_params% not found
    echo Run script with --setup flag to properly setup the project
    exit /b 1
)
REM Check if prusa_slicer_path is set correctly
if not exist "%prusa_slicer_path%" (
    echo Error: PrusaSlicer path is not set correctly
    echo Please set the correct path to PrusaSlicer in the script
    exit /b 1
)
goto:eof

REM Function to run the server
:run_server
call :check_project_ready
echo Starting the server...
if not exist "target\debug\web_server_with_database.exe" (
    echo Building project...
    cargo build
)
target\debug\web_server_with_database.exe --ws-path "C:\Users\krzys\Desktop\RustProject\rust_project\web_server_with_database" --db-name data_files\price_evaulator_database.db --prusa-slicer-path "%prusa_slicer_path%" --system windows --price-params "%price_calculator_params%"
goto:eof

REM Function to run the tests
:run_tests
echo Running the tests...
cargo test
goto:eof

pause
