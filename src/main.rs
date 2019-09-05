// systemfd --no-pid -s http::8080 -- cargo watch -x run
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, get};
use listenfd::ListenFd;

pub mod schema;
pub mod app;
pub mod db;
pub mod utils;

#[get("/hello")]
fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there you!")
}

fn index(_req: HttpRequest) -> impl Responder {
    "Hello World!"
}

fn index2() -> impl Responder {
    let b = app::models::buildings::Building::new("fea".to_owned(), "segvr".to_owned());
    HttpResponse::Ok().json(b)
}

fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new()
        .route("/", web::get().to(index))
        .route("/again", web::get().to(index2))
        .service(index3)
    );

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:8080").unwrap()
    };

    server.run().unwrap();
}
