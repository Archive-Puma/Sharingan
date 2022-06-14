use std::fmt::{Debug,Display,Formatter,Result as FmtResult};

pub enum Error {
    ServerBind(String),
    ServerRun(String),
    DBConnect(String),
    DBInsert(String),
    DBFetch(String),
    DBUpdate(String),
    DBDelete(String),
}
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self)
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Error::ServerBind(err) => write!(f, "Cannot bind the server ({})", err),
            Error::ServerRun(err) => write!(f, "Cannot start the server ({})", err),
            Error::DBConnect(err) => write!(f, "Cannot connect to the database ({})", err),
            Error::DBInsert(err) => write!(f, "Cannot insert into the database ({})", err),
            Error::DBFetch(err) => write!(f, "Cannot fetch from the database ({})", err),
            Error::DBUpdate(err) => write!(f, "Cannot update in the database ({})", err),
            Error::DBDelete(err) => write!(f, "Cannot delete from the database ({})", err),
        }
    }
}
impl actix_web::ResponseError for Error {}