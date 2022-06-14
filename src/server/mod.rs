use actix_web::{web, App, HttpServer};
use crate::{database::DB,config::Config,error::Error};

mod handlers;
pub mod session;

pub async fn run(cfg: Config, db: DB) -> Result<(),Error> {
    let srv = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(
                web::scope("/api/v1")
                    // users
                    .service(handlers::users::register)
                    .service(handlers::users::list)
                    // investigations
                    .service(handlers::investigations::create)
                    .service(handlers::investigations::list)
                    .service(handlers::investigations::fetch)
                    .service(handlers::investigations::remove)
            )
    }).bind(&cfg.address).map_err(|e| Error::ServerBind(e.to_string()))?;
    println!("Listening on http://{}", &cfg.address);
    srv.run().await.map_err(|e| Error::ServerRun(e.to_string()))
}