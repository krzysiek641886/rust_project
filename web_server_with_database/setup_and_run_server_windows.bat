@echo off
setlocal

REM Set paths (update prusa_slicer_path as needed)
set prusa_slicer_path=C:\Program Files\PrusaSlicer-2.9.0\prusa-slicer.exe
set app_params=data_files\print_price_evaluator_config.json
set PATH=%USERPROFILE%\.cargo\bin;%PATH%

REM Parse command-line arguments
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
if not exist data_files (
    mkdir data_files
    xcopy /E /I /Y "defaults\data_files" "data_files\"
)
if not exist data_files\received_orders mkdir data_files\received_orders
if not exist data_files\processed_orders mkdir data_files\processed_orders
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
if not exist data_files\prusa_config_files (
    echo Error: data_files/prusa_config_files not found
    echo Run script with --setup flag to properly setup the project
    exit /b 1
)
if not exist "%app_params%" (
    echo Error: %app_params% not found
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
target\debug\web_server_with_database.exe --app-params "%app_params%"
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
mkdir print_price_evaluator
xcopy /E /I defaults\* print_price_evaluator\
mkdir print_price_evaluator\data_files\received_orders
mkdir print_price_evaluator\data_files\processed_orders
mkdir print_price_evaluator\src\frontend
copy target\release\web_server_with_database.exe print_price_evaluator\
xcopy /E /I src\frontend print_price_evaluator\src\frontend
powershell Compress-Archive -Path print_price_evaluator -DestinationPath release_package.zip -Force
rmdir /s /q print_price_evaluator

goto:eof

pause
