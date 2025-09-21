use chrono::TimeZone;
use chrono::Utc;

/* IMPORTS FROM LIBRARIES */
/* IMPORTS FROM OTHER MODULES */
use crate::common_utils::global_types::{PrintMaterialType, PrintType, StatusType};

/* PRIVATE TYPES AND VARIABLES */
/* PUBLIC TYPES AND VARIABLES */
/* PRIVATE FUNCTIONS */
/* PUBLIC FUNCTIONS */

pub fn str_to_print_material_type(material: &str) -> Result<PrintMaterialType, &'static str> {
    match material {
        "PLA" => Ok(PrintMaterialType::PLA),
        "PET" => Ok(PrintMaterialType::PET),
        "ASA" => Ok(PrintMaterialType::ASA),
        _ => Err("Unknown material type"),
    }
}

pub fn str_to_status_type(status: &str) -> Result<StatusType, &'static str> {
    match status {
        "New" => Ok(StatusType::New),
        "InProgress" => Ok(StatusType::InProgress),
        "Completed" => Ok(StatusType::Completed),
        "Canceled" => Ok(StatusType::Canceled),
        _ => Err("Unknown status type"),
    }
}

pub fn chrono_to_datetime(date: &chrono::DateTime<Utc>) -> String {
    date.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn datetime_to_chrono(
    date_str: &str,
) -> Result<chrono::DateTime<chrono::Utc>, chrono::ParseError> {
    let naive_datetime = chrono::NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S")?;
    Ok(chrono::Utc.from_utc_datetime(&naive_datetime))
}

pub fn str_to_print_type(print_type: &str) -> Result<PrintType, &'static str> {
    match print_type {
        "ThickStrong" => Ok(PrintType::ThickStrong),
        "ThickSoft" => Ok(PrintType::ThickSoft),
        "PreciseStrong" => Ok(PrintType::PreciseStrong),
        "PreciseSoft" => Ok(PrintType::PreciseSoft),
        _ => Err("Unknown print type"),
    }
}
