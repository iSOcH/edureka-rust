use std::sync::atomic::AtomicUsize;

use rocket::{fairing::{Fairing, Info, Kind}, http::{Header, Method}, Data, Request, Response};

// https://www.shuttle.dev/blog/2022/08/04/middleware-in-rust#rocket
pub struct MyCounterFairing {
    get_requests: AtomicUsize,
}

impl MyCounterFairing {
    pub fn new() -> MyCounterFairing {
        MyCounterFairing {
            get_requests: AtomicUsize::new(0)
        }
    }
}

#[rocket::async_trait]
impl Fairing for MyCounterFairing {
    fn info(&self) -> Info {
        Info {
            name: "GET Counter",
            kind: Kind::Request | Kind::Response
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        if let Method::Get = request.method() {
            self.get_requests.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, result: &mut Response<'r>) {
        let header = Header {
            name: "req-count".into(),
            value: self.get_requests.load(std::sync::atomic::Ordering::Relaxed).to_string().into()
        };

        result.set_header(header);
    }
}