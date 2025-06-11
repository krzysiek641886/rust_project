use actix_web::{web, HttpRequest, HttpResponse};

use crate::common_utils::global_types::{EvaluationResult, SubmittedOrderData};
use std::io::Result;

pub trait SlicerInterfaceImpl: Send + Sync {
    fn ping(&self, prusa_path: &str) -> Result<()>;
    fn evaluate(
        &self,
        order: &SubmittedOrderData,
        slicer_path: &str,
        ws_path: &str,
    ) -> EvaluationResult;
}

pub trait DatabaseInterfaceImpl: Send + Sync {
    fn initialize_db(&self, db_name: &str) -> Result<()>;
    fn add_form_submission_to_db(&self, form_fields: SubmittedOrderData) -> Result<()>;
    fn read_orders_from_db(&self) -> Result<Vec<SubmittedOrderData>>;
    fn add_evaluation_to_db(&self, eval_result: &EvaluationResult) -> Result<()>;
    fn remove_order_from_db(&self, form_fields: &SubmittedOrderData) -> Result<()>;
}

pub trait WebSocketInterfaceImpl {
    async fn start_web_socket_session(
        &self,
        req: HttpRequest,
        stream: web::Payload,
    ) -> HttpResponse;
}
