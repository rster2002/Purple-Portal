use rocket::http::Status;

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
pub fn all_options() -> Status {
    Status::Ok
    /* Intentionally left empty */
}
