use actix_web::{get, App, HttpServer, Responder};
use nix::unistd;

#[get("/")]
async fn get_os_hostname() -> impl Responder {
    let hostname_cstr = unistd::gethostname().expect("Failed getting hostname");
    let hostname = hostname_cstr.to_str().expect("Hostname wasn't valid UTF-8");

    format!("Hostname: {}", hostname)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_os_hostname))
        .bind(("127.0.0.1", 3333))?
        .run()
        .await
}
