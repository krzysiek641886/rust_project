use serde::Deserialize;

/* PUBLIC TYPES */
#[derive(Clone, Debug, Deserialize)]
pub struct SubmittedOrderData {
    pub name: String,
    pub email: String,
    pub copies_nbr: u32,
    pub file_name: String,
    pub nbr_of_chunks: u32,
}

pub struct EvaluationResult {
    pub name: String,
    pub email: String,
    pub copies_nbr: u32,
    pub file_name: String,
    pub price: f64,
}
