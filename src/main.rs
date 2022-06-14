mod config;
mod database;
mod error;
mod helper;
mod server;

async fn _start() -> Result<(), error::Error> {
    let cfg = config::Config::new();
    let db = database::DB::new(&cfg.database).await?;
    server::run(cfg, db).await
}

#[actix_web::main]
async fn main() {
    if let Err(err) = _start().await {
        println!("{}", err);
        std::process::exit(1);
    }
}
