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

    let time_seconds = print_params.time;
    let material_mm = print_params.material_mm;

    // Select material rate based on material type
    let material_rate_cents_per_m = match print_params.material_type {
        PrintMaterialType::PLA => printer_configuration.material_rate_pla,
        PrintMaterialType::PET => printer_configuration.material_rate_pet,
        PrintMaterialType::ASA => printer_configuration.material_rate_asa,
    };

    // Find the appropriate hourly rate based on time thresholds
    let time_thresholds = printer_configuration.hourly_rate_time_threshold;
    let mut rate_index = 0;
    for i in 0..time_thresholds.len() {
        let print_time_lower_bound_seconds = time_thresholds[i] * 3600; // Convert hours to seconds
        if time_seconds > print_time_lower_bound_seconds {
            rate_index = i;
        } else {
            break;
        }
    }

    let hourly_rate_pln = match print_params.material_type {
        PrintMaterialType::PLA => printer_configuration.hourly_rate_pla_price[rate_index],
        PrintMaterialType::PET => printer_configuration.hourly_rate_pet_price[rate_index],
        PrintMaterialType::ASA => printer_configuration.hourly_rate_asa_price[rate_index],
    };

    // Price calculation
    let material_cost_pln = ((material_mm * material_rate_cents_per_m) as f64) / (1000.0 * 100.0);
    let print_time_cost_pln = (time_seconds * hourly_rate_pln) as f64 / 3600.0; // Convert seconds to hours
    let gross_unit_price = material_cost_pln + print_time_cost_pln;
    let extra_fee_per_each_order = 1.0; // PLN
    return gross_unit_price * nr_copies as f64 + extra_fee_per_each_order;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_printer_configuration() -> PrinterConfiguration {
        PrinterConfiguration {
            material_rate_pla: 60, // groszy per m
            material_rate_pet: 80,
            material_rate_asa: 100,
            hourly_rate_time_threshold: [0, 10, 100], // hours
            hourly_rate_pla_price: [30, 25, 20],      // PLN per hour
            hourly_rate_pet_price: [35, 30, 25],
            hourly_rate_asa_price: [40, 35, 30],
        }
    }

    #[test]
    fn test_calculate_the_price_simple_pla() {
        let printer_config = default_printer_configuration();
        let print_params = EvaluatedPrintingParameters {
            time: 3600,        // 1 hour
            material_mm: 1000, // 1 meter
            material_type: PrintMaterialType::PLA,
        };
        let nr_copies = 7;
        let price = calculate_the_price(&printer_config, print_params, nr_copies);
        println!("Price for 7 copies of PLA: {:.2} PLN", price);
        let expected = 7.0 * (0.6 + 30.0) + 1.0; // 0.6 PLN for material + 30 PLN for 1 hour + 1 PLN extra fee
        assert!(
            (price - expected).abs() < 1e-2,
            "price: {}, expected: {}",
            price,
            expected
        );
    }
}
