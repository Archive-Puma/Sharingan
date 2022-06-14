//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "investigations")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[serde(skip_deserializing)]
    pub uuid: Uuid,
    #[sea_orm(unique)]
    pub name: String,
    pub description: String,
    #[serde(skip_deserializing)]
    pub created_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::users_investigations::Entity")]
    UsersInvestigations,
}

impl Related<super::users_investigations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UsersInvestigations.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
