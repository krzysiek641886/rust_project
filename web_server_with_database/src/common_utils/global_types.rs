use serde::Deserialize;

/* PUBLIC TYPES */
#[derive(Clone, Debug, Deserialize)]
pub struct SubmittedOrderData {
    pub name: String,
    pub email: String,
    pub copies_nbr: u32,
    pub file_name: String,
    pub nbr_of_chunks: usize
}

pub struct EvaluationResult {
    pub _price: f64,
}
