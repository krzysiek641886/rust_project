use chrono;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use strum_macros::EnumIter;

/* PUBLIC TYPES */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PrintMaterialType {
    PLA,
    PET,
    ASA,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum StatusType {
    New,
    InProgress,
    Completed,
    Canceled,
}

#[derive(Clone, Debug, Deserialize, Serialize, EnumIter)]
pub enum PrintType {
    Standard,
    Precise,
    ThickLayer,
    FullFill,
}

impl Display for PrintMaterialType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrintMaterialType::PLA => write!(f, "PLA"),
            PrintMaterialType::PET => write!(f, "PET"),
            PrintMaterialType::ASA => write!(f, "ASA"),
        }
    }
}

impl Display for StatusType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusType::New => write!(f, "New"),
            StatusType::InProgress => write!(f, "InProgress"),
            StatusType::Completed => write!(f, "Completed"),
            StatusType::Canceled => write!(f, "Canceled"),
        }
    }
}

impl Display for PrintType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrintType::Standard => write!(f, "Standard"),
            PrintType::Precise => write!(f, "Precise"),
            PrintType::ThickLayer => write!(f, "ThickLayer"),
            PrintType::FullFill => write!(f, "FullFill"),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct SubmittedOrderData {
    pub name: String,
    pub email: String,
    pub copies_nbr: u32,
    pub file_name: String,
    pub nbr_of_chunks: u32,
    pub print_type: PrintType,
    pub material_type: PrintMaterialType,
}

pub struct EvaluationResult {
    pub date: chrono::DateTime<chrono::Utc>,
    pub name: String,
    pub email: String,
    pub copies_nbr: u32,
    pub file_name: String,
    pub price: f64,
    pub material_type: PrintMaterialType,
    pub print_type: PrintType,
    pub status: StatusType,
}

pub struct EvaluatedPrintingParameters {
    pub time: u32,
    pub material_mm: u32,
    pub material_type: PrintMaterialType,
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
