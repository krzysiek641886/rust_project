#!/bin/bash

prusa_slicer_path="/Applications/PrusaSlicer.app/Contents/MacOS/PrusaSlicer"
price_params="data_files/print_price_evaluator_config.json"
current_directory=$(pwd)

./web_server_with_database --ws-path "${current_directory}" --db-name data_files/price_evaulator_database.db --prusa-slicer-path "${prusa_slicer_path}" --system macos --price-params "${price_params}"