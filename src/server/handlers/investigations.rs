use actix_web::{get,post,delete,web,HttpResponse};
use sea_orm::{ActiveValue,entity::prelude::Uuid};
use crate::{database::{DB,entities::investigations},error::Error};

#[get("/investigation")]
pub async fn list(db: web::Data<DB>) -> Result<HttpResponse,Error> {
    let response = db.investigations.fetch_all().await?;
    Ok(HttpResponse::Ok().json(response))
}

#[get("/investigation/{uuid}")]
pub async fn fetch(path: web::Path<Uuid>, db: web::Data<DB>) -> Result<HttpResponse,Error> {
    let uuid: Uuid = path.into_inner();
    let response = db.investigations.fetch(uuid).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[post("/investigation")]
pub async fn create(body: web::Json<investigations::Model>, db: web::Data<DB>) -> Result<HttpResponse,Error> {
    let mut model: investigations::ActiveModel = body.into_inner().into();
    model.uuid = ActiveValue::NotSet;
    model.created_at = ActiveValue::NotSet;
    let response = db.investigations.create(model).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[delete("/investigation/{uuid}")]
pub async fn remove(path: web::Path<Uuid>, db: web::Data<DB>) -> Result<HttpResponse,Error> {
    let uuid: Uuid = path.into_inner();
    let model = investigations::ActiveModel {
        uuid: ActiveValue::Set(uuid),
        ..Default::default()
    };
    db.investigations.delete(model).await?;
    Ok(HttpResponse::Ok().json("{\"acknowledge\": true}"))
}