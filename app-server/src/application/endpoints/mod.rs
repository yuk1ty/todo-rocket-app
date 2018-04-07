extern crate rocket;

pub mod tasks;

#[get("/hc")]
fn hc() -> &'static str {
    "OK"
}

