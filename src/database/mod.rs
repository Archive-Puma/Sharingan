use std::time::Duration;
use sea_orm::{ConnectOptions,Database,DbConn};
use crate::error::Error;

pub mod entities;
use entities::prelude::*; 
mod repository;
pub use repository::Repository;

pub type DBConn = DbConn;

async fn connect(url: &str) -> Result<DBConn,Error> {
    let mut opts: ConnectOptions = ConnectOptions::new(url.to_owned());
    opts.min_connections(5)
        .connect_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(10))
        .max_lifetime(Duration::from_secs(10));
    Database::connect(opts).await.map_err(|e| Error::DBConnect(e.to_string()))
}

#[derive(Clone)]
pub struct DB {
    pub investigations: Repository<Investigations>,
    pub users: Repository<Users>,
}
impl DB {
    pub async fn new(url: &str) -> Result<Self,Error> {
        let conn: DBConn = connect(url).await?;

        Ok(Self {
            investigations: Repository::<Investigations>::new(conn.clone()),
            users: Repository::<Users>::new(conn),
        })
    }
}

#[cfg(test)]
mod test {
    // todo: test connect()
}