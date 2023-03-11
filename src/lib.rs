pub mod database;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate diesel;


use actix_web::{App, HttpServer, web};


pub struct MessageApp {
    port: u16,
}

impl MessageApp {
    pub fn new(port: u16) -> Self {

        MessageApp { port }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let conn_pool = database::establish_connection();
        println!("Starting http server: my_ip:{}", self.port);
        HttpServer::new(move || {
            App::new().app_data(web::Data::new(conn_pool.clone()))
        })
        .bind(("0.0.0.0", self.port))?
        .workers(8).run().await
    }
}

