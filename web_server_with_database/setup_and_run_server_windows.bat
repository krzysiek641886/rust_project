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
if "%~1"=="" goto show_help
if "%~1"=="-h" goto show_help
if "%~1"=="--help" goto show_help
if "%~1"=="-s" goto setup_project
if "%~1"=="--setup" goto setup_project
if "%~1"=="-r" goto run_server
if "%~1"=="--run" goto run_server
if "%~1"=="-t" goto run_tests
if "%~1"=="--test" goto run_tests
if "%~1"=="--make_package" goto make_package
if "%~1"=="-m" goto make_package

echo Invalid option: %1
goto show_help
exit /b 1

REM Function to display the help menu
:show_help
echo Usage: setup_and_run_server_windows.bat [OPTION]
echo Options:
echo   -h, --help               Show this help message and exit
echo   -s, --setup              Set up the project
echo   -r, --run                Run the server
echo   -t, --test               Run the tests
echo   -m, --make_package       Create a release package
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
target\debug\web_server_with_database.exe --ws-path "%current_directory%" --db-name data_files\price_evaluator_database.db --prusa-slicer-path "%prusa_slicer_path%" --system windows --price-params "%price_calculator_params%"
goto:eof

REM Function to run the tests
:run_tests
echo Running the tests...
cargo test
goto:eof

REM Function to run the tests
:make_package
echo Creating a release package...
cargo build --release
if not exist "target\release\web_server_with_database.exe" (
    echo Error: Release build failed
    exit /b 1
)
mkdir release_package
mkdir release_package\data_files
mkdir release_package\data_files\received_orders
mkdir release_package\data_files\processed_orders
mkdir release_package\src\frontend
copy target\release\web_server_with_database.exe release_package\
copy defaults\price_calculator_params.json release_package\data_files
copy defaults\prusa_config.ini release_package\data_files
copy defaults\print_price_evaluator.bat release_package\print_price_evaluator.bat
copy defaults\README.md release_package\README.md
xcopy /E /I src\frontend release_package\src\frontend
powershell Compress-Archive -Path release_package -DestinationPath release_package.zip -Force
rmdir /s /q release_package

goto:eof

pause
