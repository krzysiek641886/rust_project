/* IMPORTS FROM LIBRARIES */

/* IMPORTS FROM OTHER MODULES */

/* PRIVATE TYPES AND VARIABLES */

/* PUBLIC TYPES AND VARIABLES */
pub struct EvaluatedPrintingParameters {
    pub time: u32,
}

/* PRIVATE FUNCTIONS */

/* PUBLIC FUNCTIONS */
pub fn calculate_the_price(print_params: EvaluatedPrintingParameters, nr_copies: u32) -> f64 {
    return ((print_params.time * nr_copies) as f64) * 0.01;
}

/* TESTS */
