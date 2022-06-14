use actix_web::{get,post,web,HttpResponse};
use sea_orm::ActiveValue;
use crate::{database::{DB,entities::users},error::Error,helper::hash};

#[get("/user")]
pub async fn list(db: web::Data<DB>) -> Result<HttpResponse,Error> {
    let response = db.users.fetch_all().await?;
    Ok(HttpResponse::Ok().json(response))
}

#[post("/user")]
pub async fn register(body: web::Json<users::Model>, db: web::Data<DB>) -> Result<HttpResponse,Error> {
    let body = body.into_inner();
    let hashed_password = hash(&body.password).as_ref().to_vec();
    let mut model: users::ActiveModel = body.into();
    model.uuid = ActiveValue::NotSet;
    model.created_at = ActiveValue::NotSet;
    model.hash = ActiveValue::Set(hashed_password);
    let response = db.users.create(model).await?;
    Ok(HttpResponse::Ok().json(response))
}