use actix_web::{App, HttpServer};

mod db;
mod employees;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    db::init();
    let mut listenfd = listenfd::ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().configure(employees::init_routes));
    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = std::env::var("HOST").expect("Please set host in .env");
            let port = std::env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };
    server.run().await
}
