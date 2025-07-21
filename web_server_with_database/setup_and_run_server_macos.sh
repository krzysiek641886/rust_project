#!/bin/bash

prusa_slicer_path="/Applications/PrusaSlicer.app/Contents/MacOS/PrusaSlicer"
price_calculator_params="data_files/price_calculator_params.json"

# Function to display the help menu
show_help() {
    echo "Usage: setup_and_run_server.sh [OPTION]"
    echo "Options:"
    echo "  -h, --help       Show this help message and exit"
    echo "  -s, --setup      Set up the project"
    echo "  -r, --run        Run the server"
}

# Function to set up the project
setup_project() {
    echo "Setting up the project..."
    mkdir -p data_files
    mkdir -p data_files/received_orders
    mkdir -p data_files/processed_orders
    if [ ! -f data_files/prusa_config.ini ]; then
        touch data_files/prusa_config.ini
        echo "Please add a correct config file in data_files/prusa_config.ini"
    fi
    brew install --cask prusaslicer
    if [ ! -f ${price_calculator_params} ]; then
        touch ${price_calculator_params}
    fi

}

check_project_ready() {
    if [ ! -d data_files ] || 
       [ ! -d data_files/processed_orders ] || 
       [ ! -d data_files/received_orders ] || 
       [ ! -f data_files/prusa_config.ini ] || 
       [ ! -f ${price_calculator_params} ]; then
        echo "Error: data_files directory structure not correctly configured"
        echo "Run script with --setup flag to properly setup the project"
        exit;
    fi
    #Check if prusa_slicer_path is set correctly
    if [ ! -f "${prusa_slicer_path}" ]; then
        echo "Error: PrusaSlicer path is not set correctly"
        echo "Please set the correct path to PrusaSlicer in the script"
        exit;
    fi
    #Check if the PrusaSlicer application is installed
    if [ ! -d "/Applications/PrusaSlicer.app" ]; then
        echo "Error: PrusaSlicer application is not installed"
        echo "Please install PrusaSlicer application from https://www.prusa3d.com/prusaslicer/"
        exit;
    fi
    #Check if the database file exists
    if [ ! -f "data_files/price_evaulator_database.db" ]; then
        echo "Error: Database file data_files/price_evaulator_database.db does not exist"
        echo "Please run the server with --setup flag to create the database file"
        exit;
    fi

}

# Function to run the server
run_server() {
    check_project_ready
    echo "Starting the server..."
    cargo run -- --ws-path ${PWD} --db-name data_files/price_evaulator_database.db --prusa-slicer-path ${prusa_slicer_path} --system macos --price-params ${price_calculator_params}
}

# Function to run the server
run_tests() {
    echo "Running the tests..."
    cargo test
}

# Parse command-line arguments
case "$1" in
    -h|--help)
        show_help
        ;;
    -s|--setup)
        setup_project
        ;;
    -r|--run)
        run_server
        ;;
    -t|--test)
        run_tests
        ;;
    *)
        echo "Invalid option: $1"
        show_help
        exit 1
        ;;
esac
