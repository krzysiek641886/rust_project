use crate::common_utils::global_types::{SubmittedOrderData, EvaluationResult};
use std::io::{self};

pub trait SlicerInterfaceImpl: Send + Sync {
    fn ping(&self, prusa_path: &str) -> io::Result<()>;
    fn evaluate(&self, order: &SubmittedOrderData, slicer_path: &str, ws_path: &str) -> EvaluationResult;
}