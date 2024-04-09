
pub use ipp::model::PrinterState;
pub use ipp::model::JobState;
use flutter_rust_bridge::frb;

use url::Url;

use crate::utils::ipp_client::{self, IppPrinterState, PrintJobState};

fn cups_build_url(server_info: &CupsServerInfo, queue_id: Option<String>) -> String {
    let mut cups_url = Url::parse(&server_info.uri).unwrap();
    if !server_info.username.is_empty() && !server_info.password.is_empty() {
        cups_url.set_username(&server_info.username).unwrap();
        cups_url.set_password(Some(&server_info.password)).unwrap();
    }

    match queue_id {
        Some(queue_id) => cups_url.join("printers/").unwrap().join(&queue_id).unwrap(),
        None => cups_url,
    }.to_string()
}

pub fn cups_get_printers(server_info: CupsServerInfo) -> Vec<IppPrinterState> {
    let uri = cups_build_url(&server_info, None);
    ipp_client::get_printers(uri, server_info.ignore_tls_errors)
}

pub fn cups_get_printer_state(server_info: CupsServerInfo, queue_id: String) -> IppPrinterState {
    let uri = cups_build_url(&server_info, Some(queue_id));
    ipp_client::get_printer_state(uri, server_info.ignore_tls_errors)
}

pub fn cups_resume_printer(server_info: CupsServerInfo, queue_id: String) {
    let uri = cups_build_url(&server_info, Some(queue_id));
    ipp_client::resume_printer(uri, server_info.ignore_tls_errors);
}

pub fn cups_get_jobs_states(server_info: CupsServerInfo, queue_id: String) -> Vec<PrintJobState> {
    let uri = cups_build_url(&server_info, Some(queue_id));
    ipp_client::get_jobs_states(uri, server_info.ignore_tls_errors)
}

pub fn cups_print_job(server_info: CupsServerInfo, queue_id: String, job_name: String, pdf_data: Vec<u8>) {
    let uri = cups_build_url(&server_info, Some(queue_id));
    ipp_client::print_job(uri, server_info.ignore_tls_errors, job_name, pdf_data);
}

pub fn cups_release_job(server_info: CupsServerInfo, queue_id: String, job_id: i32) {
    let uri = cups_build_url(&server_info, Some(queue_id));
    ipp_client::release_job(uri, server_info.ignore_tls_errors, job_id);
}

// /////// //
// Structs //
// /////// //

pub struct CupsServerInfo {
    pub uri: String,
    pub ignore_tls_errors: bool,
    pub username: String,
    pub password: String,
}

// ////////////// //
// Mirror structs //
// ////////////// //

#[frb(mirror(PrinterState))]
pub enum _PrinterState {
    Idle = 3,
    Processing = 4,
    Stopped = 5,
}

#[frb(mirror(JobState))]
pub enum _JobState {
    Pending = 3,
    PendingHeld = 4,
    Processing = 5,
    ProcessingStopped = 6,
    Canceled = 7,
    Aborted = 8,
    Completed = 9,
}
