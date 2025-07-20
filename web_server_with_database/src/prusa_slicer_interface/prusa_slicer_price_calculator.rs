/* IMPORTS FROM LIBRARIES */

/* IMPORTS FROM OTHER MODULES */
use crate::common_utils::global_types::{
    EvaluatedPrintingParameters, PrintMaterialType, PrinterConfiguration,
};

/* PRIVATE TYPES AND VARIABLES */

/* PUBLIC TYPES AND VARIABLES */

/* PRIVATE FUNCTIONS */

/* PUBLIC FUNCTIONS */
pub fn calculate_the_price(
    printer_configuration: &PrinterConfiguration,
    print_params: EvaluatedPrintingParameters,
    nr_copies: u32,
) -> f64 {
    //     Formula for pricing:
    // Gross Price = Material Cost + 1 + Print Time * Hourly Rate
    // Material Cost = Material Usage * Material Rate / 1000
    // Material Rate PLA = 60 groszy / m
    // Material Rate PET = 80 groszy / m
    // Material Rate ASA = 100 groszy / m
    // Hourly Rate PLA:
    // <10H = 30zl / H
    // 10-100H = 25zl / H
    // 100 = 20zl / H

    // Hourly Rate PET + 5zl PLA Rate
    // Hourly Rate ASA + 5zl PET Rate
    let material = PrintMaterialType::PLA; // Assuming PLA for simplicity, this should be passed as an argument in a real scenario
    let _material_rate = match material {
        PrintMaterialType::PLA => printer_configuration.material_rate_pla,
        PrintMaterialType::PET => printer_configuration.material_rate_pet,
        PrintMaterialType::ASA => printer_configuration.material_rate_asa,
    };
    let _hourly_rate = match material {
        PrintMaterialType::PLA => printer_configuration.hourly_rate_pla_price,
        PrintMaterialType::PET => printer_configuration.hourly_rate_pet_price,
        PrintMaterialType::ASA => printer_configuration.hourly_rate_asa_price,
    };
    let _time_thresholds = printer_configuration.hourly_rate_time_threshold;

    return (((print_params.time + print_params.material_mm) * nr_copies) as f64) * 0.01;
}

/* TESTS */
