use actix_web::{get, App, HttpServer, Responder};
extern crate hostname;

#[get("/")]
async fn get_os_hostname() -> impl Responder {
    let hostname = hostname::get()
        .expect("error when getting hostname")
        .to_str()
        .expect("invalid utf-8 string")
        .to_string();

    format!("Hostname: {}", hostname)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 3333;
    println!("Server listening on: http://localhost:{}", port);

    HttpServer::new(|| App::new().service(get_os_hostname))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
