use aci_diplom::MessageApp;

extern crate diesel;
extern crate diesel_migrations;
extern crate log;
extern crate env_logger;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let app = MessageApp::new(8080);
    app.run().await
}