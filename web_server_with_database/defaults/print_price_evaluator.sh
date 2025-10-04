#!/bin/bash

prusa_slicer_path="/Applications/PrusaSlicer.app/Contents/MacOS/PrusaSlicer"
app_params="data_files/print_price_evaluator_config.json"
current_directory=$(pwd)

./web_server_with_database --app-params "${app_params}"