use rocket::request::Request;
use rocket::response::{self, Responder, Response};

pub struct PreflightResponse();

impl<'r> Responder<'r> for PreflightResponse {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .raw_header("Content-Type", "application/json")
            .raw_header("Access-Control-Allow-Origin", "*")
            .raw_header("Access-Control-Allow-Methods", "GET,POST,PUT,HEAD,OPTIONS")
            .raw_header("Access-Control-Allow-Headers", "Content-Type")
            .raw_header("Access-Control-Allow-Credentials", "true")
            .ok()
    }
}
