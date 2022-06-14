use sea_orm::{ActiveModelBehavior,EntityTrait,PrimaryKeyTrait};
use sea_orm::entity::IntoActiveModel;
use std::marker::{Send,PhantomData};
use super::DBConn;
use crate::error::Error;

#[derive(Debug,Clone)]
pub struct Repository<T: EntityTrait> {
    pub conn: DBConn,
    pub __phantom: PhantomData<T>,
}
impl<T> Repository<T>
where
    T: EntityTrait
{
    pub fn new(conn: DBConn) -> Self {
        Self {
            conn,
            __phantom: PhantomData,
        }
    }

    pub async fn create<A>(&self, model: A) -> Result<T::Model,Error>
    where
        T::Model: IntoActiveModel<A>,
        A: ActiveModelBehavior<Entity = T> + Send,
    {
        model.insert(&self.conn)
            .await
            .map_err(|e| Error::DBInsert(e.to_string()))
    }

    pub async fn update<A>(&self, model: A) -> Result<T::Model,Error>
    where
        T::Model: IntoActiveModel<A>,
        A: ActiveModelBehavior<Entity = T> + Send,
    {
        model.update(&self.conn)
            .await
            .map_err(|e| Error::DBUpdate(e.to_string()))
    }

    pub async fn fetch(&self, id: <T::PrimaryKey as PrimaryKeyTrait>::ValueType) -> Result<Option<T::Model>,Error> {
        T::find_by_id(id)
            .one(&self.conn)
            .await
            .map_err(|e| Error::DBFetch(e.to_string()))
    }

    pub async fn fetch_all(&self) -> Result<Vec<T::Model>,Error> {
        T::find()
            .all(&self.conn)
            .await
            .map_err(|e| Error::DBFetch(e.to_string()))
    }

    pub async fn delete<A>(&self, model: A) -> Result<sea_orm::DeleteResult,Error>
    where
        A: ActiveModelBehavior<Entity = T> + Send
    {
        model.delete(&self.conn)
            .await
            .map_err(|e| Error::DBDelete(e.to_string()))
    }
}
