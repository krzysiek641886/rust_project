/* PUBLIC TYPES */
pub struct SubmittedOrderData {
    pub name: Option<String>,
    pub email: Option<String>,
    pub copies_nbr: u32,
    pub file_name: Option<String>,
}

pub struct EvaluationResult {
    pub _price: f64,
}
