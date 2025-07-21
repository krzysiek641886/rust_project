use serde::Deserialize;

/* PUBLIC TYPES */
pub enum PrintMaterialType {
    PLA,
    PET,
    ASA,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SubmittedOrderData {
    pub name: String,
    pub email: String,
    pub copies_nbr: u32,
    pub file_name: String,
    pub nbr_of_chunks: u32,
    // pub material_type: PrintMaterialType,
}

pub struct EvaluationResult {
    pub name: String,
    pub email: String,
    pub copies_nbr: u32,
    pub file_name: String,
    pub price: f64,
}

pub struct EvaluatedPrintingParameters {
    pub time: u32,
    pub material_mm: u32,
}

#[derive(Deserialize)]
pub struct PrinterConfiguration {
    pub material_rate_pla: u32,
    pub material_rate_pet: u32,
    pub material_rate_asa: u32,
    pub hourly_rate_time_threshold: [u32; 3],
    pub hourly_rate_pla_price: [u32; 3],
    pub hourly_rate_pet_price: [u32; 3],
    pub hourly_rate_asa_price: [u32; 3],
}
