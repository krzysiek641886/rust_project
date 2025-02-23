#!/bin/bash

orca_slicer_path="orca_slicer/build_x86_64/OrcaSlicer/OrcaSlicer.app/Contents/MacOS/OrcaSlicer"

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
    # Add setup commands here
}

# Function to run the server
run_server() {
    echo "Running the server..."
    cargo run -- --ws-path ${PWD} --db-name wyceniarka_database.db --orca-slicer-path ${orca_slicer_path} --system macos
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